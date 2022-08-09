use async_trait::async_trait;
use fehler::throws;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use crate::args::CliArgsCircuit;
use crate::ext::types::lang_schema::LangGate;
use crate::ext::{CircuitGenerator, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::utils::{init_one, inverse};
use crate::{Error, CLIFFORD_GATES, CLIFFORD_GATES_2, CLIFFORD_GATES_INV, PAULI_GATES};

pub struct StructCircuitGenerator {
    args: CliArgsCircuit,
}

// Structured mirror benchmarking with some restrictions:
// Always the result should be all zeros
// Second part of algorithm is always inverse to the first part in everything
// Length is counted as 2d.

impl StructCircuitGenerator {
    #[throws]
    pub fn from_args(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl CircuitGenerator for StructCircuitGenerator {
    async fn generate(
        &mut self,
        _lang_schema: &LangSchemaDyn,
        i: i32,
        j: i32,
        iter: i32,
    ) -> Result<Option<LangCircuit>, Error> {
        let iter = if self.args.rand { iter } else { 0 };

        let oqs_i = i;
        let mut oqs_gates = vec![];
        let mut oqs_inv_gates = vec![];

        let c_len2 = CLIFFORD_GATES_2.len();

        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);

        let mut a = iter;
        let mut b = iter;
        for _ in 1..=j {
            let mut v: Vec<i32> = (0..i).collect();
            if self.args.shuffle {
                v.shuffle(&mut rng);
            }

            for mut ii in &v.into_iter().chunks(2) {
                let i1 = ii.next().unwrap(); // cannot fail
                let p_gate_index = b as usize % PAULI_GATES.len();
                let c_gate_index = a as usize % c_len2;
                b += 1;

                oqs_gates.push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(i1).build());
                oqs_inv_gates.push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(i1).build());

                if let Some(i2) = ii.next() {
                    oqs_gates.push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(i2).build());
                    oqs_inv_gates
                        .push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(i2).build());

                    oqs_gates.push(
                        LangGate::builder()
                            .t(CLIFFORD_GATES_2[c_gate_index])
                            .i(i1)
                            .other(i2)
                            .build(),
                    );
                    oqs_inv_gates.push(
                        LangGate::builder()
                            .t(CLIFFORD_GATES_2[c_gate_index])
                            .i(i1)
                            .other(i2)
                            .build(),
                    );

                    a += 1;
                }
                // NO space for double gate
                else {
                    oqs_gates
                        .push(LangGate::builder().t(CLIFFORD_GATES[c_gate_index]).i(i1).build());
                    oqs_inv_gates.push(
                        LangGate::builder().t(CLIFFORD_GATES_INV[c_gate_index]).i(i1).build(),
                    );
                }
            }
        }

        oqs_gates = inverse(self.args.bench, oqs_gates, oqs_inv_gates, i);

        // Add NOT gate when should change init state
        if self.args.init_one {
            oqs_gates = init_one(oqs_gates, i);
        }

        Ok(Some(
            LangCircuit::builder()
                .creg(oqs_i)
                .qreg(oqs_i)
                .gates(oqs_gates)
                .build(),
        ))
    }
}
