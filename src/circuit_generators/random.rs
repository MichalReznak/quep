//! Randomized mirror benchmarking with some restrictions:
//! Always the result should be all zeros
//! Second part of algorithm is always inverse to the first part in everything
//! Length is counted as 2d.
//! It is using **uniform sampling**

use async_trait::async_trait;
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;

use crate::args::CliArgsCircuit;
use crate::ext::types::lang_schema::LangGate;
use crate::ext::{CircuitGenerator, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::utils::{init_one, inverse};
use crate::{Error, CLIFFORD_GATES, CLIFFORD_GATES_2, CLIFFORD_GATES_INV, PAULI_GATES};

pub struct RandCircuitGenerator {
    args: CliArgsCircuit,
}

impl RandCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl CircuitGenerator for RandCircuitGenerator {
    async fn generate(
        &mut self,
        _lang_schema: &LangSchemaDyn,
        i: i32,
        j: i32,
        _iter: i32,
    ) -> Result<Option<LangCircuit>, Error> {
        let mut rng = rand::rngs::SmallRng::from_entropy();
        let p_rand: Uniform<usize> = Uniform::from(0..4);
        let c_rand: Uniform<usize> = Uniform::from(0..9);

        let mut oqs_gates = vec![];
        let mut oqs_inv_gates = vec![];
        let oqs_width = i;

        let c_len = CLIFFORD_GATES.len();

        let mut skip = false;
        for _ in 1..=j {
            for ii in 0..i {
                let p_gate_index = p_rand.sample(&mut rng);
                let c_gate_index = c_rand.sample(&mut rng);

                if skip {
                    skip = false;
                }
                else if c_gate_index < c_len {
                    oqs_gates
                        .push(LangGate::builder().t(CLIFFORD_GATES[c_gate_index]).i(ii).build());
                    oqs_inv_gates.push(
                        LangGate::builder().t(CLIFFORD_GATES_INV[c_gate_index]).i(ii).build(),
                    );
                }
                // NO space for double gate
                else if ii == i - 1 {
                    oqs_gates.push(
                        LangGate::builder().t(CLIFFORD_GATES[c_gate_index - c_len]).i(ii).build(),
                    );
                    oqs_inv_gates.push(
                        LangGate::builder()
                            .t(CLIFFORD_GATES_INV[c_gate_index - c_len])
                            .i(ii)
                            .build(),
                    );
                }
                else {
                    oqs_gates.push(
                        LangGate::builder()
                            .t(CLIFFORD_GATES_2[c_gate_index - c_len])
                            .i(ii)
                            .other(ii + 1)
                            .build(),
                    );
                    oqs_inv_gates.push(
                        LangGate::builder()
                            .t(CLIFFORD_GATES_2[c_gate_index - c_len])
                            .i(ii)
                            .other(ii + 1)
                            .build(),
                    );
                    skip = true;
                }

                oqs_gates.push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(ii).build());
                oqs_inv_gates.push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(ii).build());
            }
        }

        oqs_gates = inverse(self.args.bench, oqs_gates, oqs_inv_gates, i);

        // Add NOT gate when should change init state
        if self.args.init_one {
            oqs_gates = init_one(oqs_gates, i);
        }

        Ok(Some(LangCircuit::builder().width(oqs_width).gates(oqs_gates).build()))
    }
}
