use async_trait::async_trait;
use fehler::throws;

use crate::args::types::{CircuitBenchType, OrchestratorType};
use crate::args::CliArgsCircuit;
use crate::error::Constraint;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::utils::cycle;
use crate::{CliArgs, Error};

#[allow(dead_code)]
pub struct VolumeCircuitGenerator {
    args: CliArgsCircuit,
}

impl VolumeCircuitGenerator {
    #[throws]
    pub fn from_args(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl CircuitGenerator for VolumeCircuitGenerator {
    fn check_constraints(&self, args: &CliArgs) -> Result<(), Error> {
        if !matches!(args.orch.t, OrchestratorType::Volume | OrchestratorType::Single) {
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
        _lang_schema: &LangSchemaDyn,
        i: i32,
        _: i32,
        _: i32,
    ) -> Result<Option<LangCircuit>, Error> {
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

        Ok(Some(LangCircuit::builder().gates(result).width(i).build()))
    }
}
