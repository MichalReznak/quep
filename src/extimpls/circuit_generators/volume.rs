use async_trait::async_trait;
use fehler::throws;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::SeedableRng;

use crate::args::types::{CircuitBenchType, OrchestratorType};
use crate::args::CliArgsCircuit;
use crate::error::Constraint;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::utils::{cycle, init_one, inverse};
use crate::{CircuitType, CliArgs, Error, CLIFFORD_GATES_2};

#[allow(dead_code)]
pub struct VolumeCircuitGenerator {
    args: CliArgsCircuit,
}

impl VolumeCircuitGenerator {
    #[throws]
    pub fn from_args(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }

    #[throws]
    async fn generate1(
        &mut self,
        _: &LangSchemaDyn,
        i: i32,
        _: i32,
        _: i32,
    ) -> Option<LangCircuit> {
        let j = if matches!(self.args.bench, CircuitBenchType::None) {
            i
        }
        else {
            i / 2
        };

        use LangGateType::*;
        let gates = vec![X, H, Z, Y];
        let mut result = vec![];

        for j in 0..j {
            for i in 0..i {
                let gate =
                    LangGate::builder().t(gates[(i + j) as usize % gates.len()]).i(i).build();
                result.push(gate);
            }
        }

        {
            let mut inv_result = result.clone();
            inv_result.reverse();

            use CircuitBenchType::*;
            match self.args.bench {
                Mirror => {
                    result.push(LangGate::builder().t(Barrier).i(-1).build());
                    result.extend(inv_result.into_iter());
                }
                Cycle => {
                    result = cycle(result, inv_result, i);
                }
                None => {}
            }
        }

        // Add NOT gate when should change init state
        if self.args.init_one || i % 2 == 1 {
            let mut gates = vec![];
            for i in 0..i {
                gates.push(LangGate::builder().t(X).i(i).build());
            }
            gates.push(LangGate::builder().t(Barrier).i(-1).build());
            gates.extend(result);
            result = gates;
        }

        Some(LangCircuit::builder().creg(i).qreg(i).gates(result).build())
    }

    #[throws]
    async fn generate2(
        &mut self,
        _: &LangSchemaDyn,
        i: i32,
        _: i32,
        iter: i32,
    ) -> Option<LangCircuit> {
        let j = if matches!(self.args.bench, CircuitBenchType::None) {
            i
        }
        else {
            i / 2
        };
        let iter = if self.args.rand { iter } else { 0 };

        let oqs_i = i;
        let mut oqs_gates = vec![];
        let mut oqs_inv_gates = vec![];

        let c_len2 = CLIFFORD_GATES_2.len();
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);

        let mut a = iter;
        for _ in 1..=j {
            let mut v: Vec<i32> = (0..i).collect();
            if self.args.shuffle {
                v.shuffle(&mut rng);
            }

            for (i1, i2) in v.into_iter().tuples() {
                let c2_gate_index = a as usize % c_len2;

                let gate =
                    LangGate::builder().t(CLIFFORD_GATES_2[c2_gate_index]).i(i1).other(i2).build();
                oqs_gates.push(gate.clone());
                oqs_inv_gates.push(gate);

                a += 1;
            }
        }

        oqs_gates = inverse(self.args.bench, oqs_gates, oqs_inv_gates, i);

        // Add NOT gate when should change init state
        if self.args.init_one || i % 2 == 1 {
            oqs_gates = init_one(oqs_gates, i);
        }

        Some(
            LangCircuit::builder()
                .creg(oqs_i)
                .qreg(oqs_i)
                .gates(oqs_gates)
                .build(),
        )
    }
}

#[async_trait]
impl CircuitGenerator for VolumeCircuitGenerator {
    fn check_constraints(&self, args: &CliArgs) -> Result<(), Error> {
        use OrchestratorType::*;
        if !matches!(args.orch.t, Volume | Single) {
            Constraint {
                reason: "Volume Circuit Generator requires Volume or Single Orchestrator"
                    .to_string(),
            }
            .fail()?;
        }
        Ok(())
    }

    async fn generate(
        &mut self,
        lang_schema: &LangSchemaDyn,
        i: i32,
        j: i32,
        iter: i32,
    ) -> Result<Option<LangCircuit>, Error> {
        match self.args.t {
            CircuitType::Volume => self.generate1(lang_schema, i, j, iter).await,
            CircuitType::Volume2 => self.generate2(lang_schema, i, j, iter).await,
            _ => unreachable!(),
        }
    }
}
