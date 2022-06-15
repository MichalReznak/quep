use async_trait::async_trait;
use log::debug;
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;

use crate::args::CliArgsCircuit;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchema};
use crate::lang_schemas::LangCircuit;
use crate::{Chooser, Error};

#[allow(dead_code)]
pub struct RandMirrorCircuitGenerator {
    args: CliArgsCircuit,
}

impl RandMirrorCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

// Randomized mirror benchmarking with some restrictions:
// Always the result should be all zeros
// Second part of algorithm is always inverse to the first part in everything
// Length is counted as 2d.
// It is using **uniform sampling**

#[async_trait]
impl CircuitGenerator for RandMirrorCircuitGenerator {
    async fn generate(
        &mut self,
        i: i32,
        j: i32,
        _iter: i32,
        mirror: bool,
    ) -> Result<Option<GenCircuit>, Error> {
        use LangGateType::*;
        let pauli_gates = [Id, X, Y, Z];
        let clifford_gates = [H, S, Id, X, Y, Z];
        let clifford_gates_inv = [H, Sdg, Id, X, Y, Z];
        let clifford_gates_2 = [Cx, Cz, Swap];

        let mut rng = rand::rngs::SmallRng::from_entropy();
        let p_rand: Uniform<usize> = Uniform::from(0..4);
        let c_rand: Uniform<usize> = Uniform::from(0..9);

        let i = i + 1;
        let j = j + 1;
        let mut oqs_gates = vec![];
        let mut oqs_inv_gates = vec![];
        let oqs_width = i;

        let c_len = clifford_gates.len();

        let mut skip = false;
        for _ in 0..j {
            for ii in 0..i {
                let p_gate_index = p_rand.sample(&mut rng);
                let c_gate_index = c_rand.sample(&mut rng);

                if skip {
                    skip = false;
                }
                else if c_gate_index < c_len {
                    oqs_gates
                        .push(LangGate::builder().t(clifford_gates[c_gate_index]).i(ii).build());
                    oqs_inv_gates.push(
                        LangGate::builder().t(clifford_gates_inv[c_gate_index]).i(ii).build(),
                    );
                }
                // NO space for double gate
                else if ii == i - 1 {
                    oqs_gates.push(
                        LangGate::builder().t(clifford_gates[c_gate_index - c_len]).i(ii).build(),
                    );
                    oqs_inv_gates.push(
                        LangGate::builder()
                            .t(clifford_gates_inv[c_gate_index - c_len])
                            .i(ii)
                            .build(),
                    );
                }
                else {
                    oqs_gates.push(
                        LangGate::builder()
                            .t(clifford_gates_2[c_gate_index - c_len])
                            .i(ii)
                            .other(ii + 1)
                            .build(),
                    );
                    oqs_inv_gates.push(
                        LangGate::builder()
                            .t(clifford_gates_2[c_gate_index - c_len])
                            .i(ii)
                            .other(ii + 1)
                            .build(),
                    );
                    skip = true;
                }

                oqs_gates.push(LangGate::builder().t(pauli_gates[p_gate_index]).i(ii).build());
                oqs_inv_gates.push(LangGate::builder().t(pauli_gates[p_gate_index]).i(ii).build());
            }
        }

        oqs_inv_gates.reverse();

        let oqs = if mirror {
            LangCircuit::builder()
                .width(oqs_width)
                .gates(oqs_gates)
                .inv_gates(oqs_inv_gates)
                .build()
        }
        else {
            LangCircuit::builder().width(oqs_width).gates(oqs_gates).build()
        };
        let c = Chooser::get_lang_schema(self.args.schema).as_string(oqs).await?;
        debug!("{}", c.circuit);

        Ok(Some(c))
    }
}
