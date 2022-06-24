use std::path::Path;

use async_trait::async_trait;
use log::error;
use openqasm as oq;
use oq::GenericError;
use walkdir::{DirEntry, WalkDir};

use crate::args::types::CircuitSchemaType;
use crate::args::CliArgsCircuit;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::CircuitGenerator;
use crate::Error;

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
    async fn generate(
        &mut self,
        _: i32,
        j: i32,
        _: i32,
        _mirror: bool,
    ) -> Result<Option<GenCircuit>, Error> {
        if j >= self.entries.len() as i32 {
            Ok(None)
        }
        else {
            let path = self.entries[j as usize].path();
            let mut circuit = std::fs::read_to_string(path)?;
            circuit.remove_matches("\r");

            let mut cache = oq::SourceCache::new();
            let mut parser = oq::Parser::new(&mut cache);

            let check: Result<_, oq::Errors> = try {
                parser.parse_source(circuit.to_string(), Some(&Path::new(".")));
                parser.done().to_errors()?.type_check().to_errors()?;
            };
            if let Err(errors) = check {
                errors.print(&mut cache)?;
                error!("{errors:#?}");
                Err(crate::Error::SomeError)
            }
            else {
                Ok(Some(
                    GenCircuit::builder().circuit(circuit).t(CircuitSchemaType::OpenQasm).build(),
                )) // TODO openqasm only for now
            }
        }
    }
}
