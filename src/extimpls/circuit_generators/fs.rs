use std::path::Path;

use async_trait::async_trait;
use itertools::interleave;
use log::error;
use openqasm as oq;
use oq::GenericError;
use snafu::OptionExt;
use walkdir::{DirEntry, WalkDir};

use crate::args::types::{CircuitBenchType, CircuitSchemaType, OrchestratorType};
use crate::args::CliArgsCircuit;
use crate::error::{Constraint, OutOfBounds};
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::{CircuitGenerator, LangSchema, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::utils::oqs_parse_circuit;
use crate::{CliArgs, Error};

#[allow(dead_code)]
#[derive(Debug)]
pub struct FsCircuitGenerator {
    args: CliArgsCircuit,

    entries: Vec<DirEntry>,
}

impl FsCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        let entries = WalkDir::new("data") // TODO needs to be arg
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

// TODO use mirror somehow, set state to one
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

        if !matches!(args.circuit.schema, CircuitSchemaType::OpenQasm) {
            Constraint {
                reason: "FileSystem Circuit Generator supports OpenQasm files only".to_string(),
            }
            .fail()?;
        }
        Ok(())
    }

    // TODO switch arguments
    async fn generate(
        &mut self,
        lang_schema: &LangSchemaDyn,
        width: i32,
        j: i32,
        _: i32,
    ) -> Result<Option<LangCircuit>, Error> {
        if j > self.entries.len() as i32 {
            Ok(None)
        }
        else {
            let path = self.entries[(j - 1) as usize].path();
            let mut circuit = std::fs::read_to_string(path)?;
            circuit.remove_matches("\r");

            {
                let mut cache = oq::SourceCache::new();
                let check: Result<_, oq::Errors> = try {
                    let mut parser = oq::Parser::new(&mut cache);
                    parser.parse_source(circuit.to_string(), Some(&Path::new(".")));
                    parser.done().to_errors()?.type_check().to_errors()?;
                };

                if let Err(errors) = check {
                    errors.print(&mut cache)?;
                    error!("{errors:#?}");
                    return Err(Error::SomeError);
                }
            }

            // TODO copied code from base.rs
            // TODO remove, don't know how
            let oqs_gates = lang_schema
                .parse_file(self.entries[(j - 1) as usize].path().to_str().context(OutOfBounds)?)
                .await?;

            // TODO should have depth and width?
            let (mut oqs_gates, mut inv_gates) = oqs_parse_circuit(oqs_gates, i32::MAX, width)?;

            use CircuitBenchType::*;

            match self.args.bench {
                Mirror => {
                    // TODO interleave with barriers??
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
