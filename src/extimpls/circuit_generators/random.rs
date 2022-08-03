//! Randomized mirror benchmarking with some restrictions:
//! Always the result should be all zeros
//! Second part of algorithm is always inverse to the first part in everything
//! Length is counted as 2d.
//! It is using **uniform sampling**

use async_trait::async_trait;
use fehler::throws;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::SliceRandom;
use rand::thread_rng;

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
    #[throws]
    pub fn from_args(args: &CliArgsCircuit) -> Self {
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
        let mut rng = thread_rng();
        let p_rand: Uniform<usize> = Uniform::from(0..PAULI_GATES.len());
        let c_rand: Uniform<usize> = Uniform::from(0..CLIFFORD_GATES.len());
        let c2_rand: Uniform<usize> = Uniform::from(0..CLIFFORD_GATES_2.len());

        let mut oqs_gates = vec![];
        let mut oqs_inv_gates = vec![];
        let oqs_width = i;

        for _ in 1..=j {
            let mut v: Vec<i32> = (0..i).collect();
            if self.args.shuffle {
                v.shuffle(&mut rng);
            }

            for mut ii in &v.into_iter().chunks(2) {
                let i1 = ii.next().unwrap(); // cannot fail
                let p_gate_index = p_rand.sample(&mut rng);
                let c_gate_index = c_rand.sample(&mut rng);
                let c2_gate_index = c2_rand.sample(&mut rng);

                oqs_gates.push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(i1).build());
                oqs_inv_gates.push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(i1).build());

                if let Some(i2) = ii.next() {
                    oqs_gates.push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(i2).build());
                    oqs_inv_gates
                        .push(LangGate::builder().t(PAULI_GATES[p_gate_index]).i(i2).build());

                    oqs_gates.push(
                        LangGate::builder()
                            .t(CLIFFORD_GATES_2[c2_gate_index])
                            .i(i1)
                            .other(i2)
                            .build(),
                    );
                    oqs_inv_gates.push(
                        LangGate::builder()
                            .t(CLIFFORD_GATES_2[c2_gate_index])
                            .i(i1)
                            .other(i2)
                            .build(),
                    );
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

        Ok(Some(LangCircuit::builder().width(oqs_width).gates(oqs_gates).build()))
    }
}
