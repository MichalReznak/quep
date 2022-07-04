//! Iter: How many valid gates should be skipped

use async_trait::async_trait;
use fehler::throws;
use itertools::interleave;
use snafu::OptionExt;
use walkdir::{DirEntry, WalkDir};

use crate::args::types::{CircuitBenchType, LangSchemaType, OrchestratorType};
use crate::args::CliArgsCircuit;
use crate::error::{Constraint, OutOfBounds};
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchema, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::utils::oqs_parse_circuit;
use crate::{dir, CliArgs, Error};

#[allow(dead_code)]
#[derive(Debug)]
pub struct FsCircuitGenerator {
    args: CliArgsCircuit,

    entries: Vec<DirEntry>,
}

impl FsCircuitGenerator {
    #[throws]
    pub fn from_args(args: &CliArgsCircuit) -> Self {
        let source = dir(&args.source)?;

        let entries = WalkDir::new(&source)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();

        Self {
            args: args.clone(),
            entries,
        }
    }
}

#[async_trait]
impl CircuitGenerator for FsCircuitGenerator {
    fn check_constraints(&self, args: &CliArgs) -> Result<(), Error> {
        if !matches!(args.orch.t, OrchestratorType::Linear | OrchestratorType::Single) {
            Constraint {
                reason: "FileSystem Circuit Generator requires Linear or Single Orchestrator"
                    .to_string(),
            }
            .fail()?;
        }

        if !matches!(args.lang_schema.t, LangSchemaType::OpenQasm) {
            Constraint {
                reason: "FileSystem Circuit Generator supports OpenQasm files only".to_string(),
            }
            .fail()?;
        }
        Ok(())
    }

    async fn generate(
        &mut self,
        lang_schema: &LangSchemaDyn,
        width: i32,
        j: i32,
        iter: i32,
    ) -> Result<Option<LangCircuit>, Error> {
        if j > self.entries.len() as i32 {
            Ok(None)
        }
        else {
            let oqs_gates = lang_schema
                .parse_file(self.entries[(j - 1) as usize].path().to_str().context(OutOfBounds)?)
                .await?;

            let (mut oqs_gates, mut inv_gates) =
                oqs_parse_circuit(oqs_gates, i32::MAX, width, iter)?;

            use CircuitBenchType::*;

            match self.args.bench {
                Mirror => {
                    oqs_gates.push(LangGate::builder().t(LangGateType::Barrier).i(-1).build());
                    oqs_gates.extend(inv_gates.into_iter());
                }
                Cycle => {
                    inv_gates.reverse();
                    oqs_gates = interleave(oqs_gates, inv_gates).collect();
                }
                None => {}
            }

            // Add NOT gate when should change init state
            if self.args.init_one {
                let mut gates = vec![];
                for i in 0..width {
                    gates.push(LangGate::builder().t(LangGateType::X).i(i).build());
                }
                gates.push(LangGate::builder().t(LangGateType::Barrier).i(-1).build());
                gates.extend(oqs_gates);
                oqs_gates = gates;
            }

            Ok(Some(LangCircuit::builder().width(width).gates(oqs_gates).build()))
        }
    }
}
