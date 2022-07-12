use async_trait::async_trait;
use fehler::throws;

use crate::args::types::{CircuitBenchType, OrchestratorType};
use crate::args::CliArgsCircuit;
use crate::error::Constraint;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::utils::{cycle, init_one, inverse};
use crate::{CircuitType, CliArgs, Error, CLIFFORD_GATES, CLIFFORD_GATES_2, CLIFFORD_GATES_INV};

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

        Some(LangCircuit::builder().gates(result).width(i).build())
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

        let c_len = CLIFFORD_GATES.len();
        let c_len2 = CLIFFORD_GATES_2.len();

        let mut a = iter;
        let mut b = iter;
        let mut skip = false;
        for _ in 1..=j {
            for ii in 0..i {
                let c_gate_index = b as usize % c_len;
                let c2_gate_index = a as usize % c_len2;

                if skip {
                    skip = false;
                }
                // NO space for double gate
                else if ii == i - 1 {
                    oqs_gates
                        .push(LangGate::builder().t(CLIFFORD_GATES[c_gate_index]).i(ii).build());
                    oqs_inv_gates.push(
                        LangGate::builder().t(CLIFFORD_GATES_INV[c_gate_index]).i(ii).build(),
                    );
                    b += 1;
                }
                else {
                    let gate = LangGate::builder()
                        .t(CLIFFORD_GATES_2[c2_gate_index])
                        .i(ii)
                        .other(ii + 1)
                        .build();
                    oqs_gates.push(gate.clone());
                    oqs_inv_gates.push(gate);

                    a += 1;
                    skip = true;
                }
            }
        }

        oqs_gates = inverse(self.args.bench, oqs_gates, oqs_inv_gates, i);

        // Add NOT gate when should change init state
        if self.args.init_one || i % 2 == 1 {
            oqs_gates = init_one(oqs_gates, i);
        }

        Some(LangCircuit::builder().width(oqs_i).gates(oqs_gates).build())
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
