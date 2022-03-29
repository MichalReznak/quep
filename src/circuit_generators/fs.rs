use std::path::Path;

use async_trait::async_trait;
use openqasm as oq;
use oq::GenericError;
use walkdir::{DirEntry, WalkDir};

use crate::traits::CircuitGenerator;
use crate::Error;

pub struct FsCircuitGenerator {
    entries: Vec<DirEntry>,
}

impl FsCircuitGenerator {
    pub fn new() -> Self {
        let entries = WalkDir::new("data")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();

        Self { entries }
    }
}

#[async_trait]
impl CircuitGenerator for FsCircuitGenerator {
    async fn generate(&self, i: i32) -> Result<Option<String>, Error> {
        if i >= self.entries.len() as i32 {
            Ok(None)
        }
        else {
            let path = self.entries[i as usize].path();
            let mut circuit = std::fs::read_to_string(path)?;
            circuit.remove_matches("\r\n");

            let check: Result<_, oq::Errors> = try {
                let mut cache = oq::SourceCache::new();
                let mut parser = oq::Parser::new(&mut cache);
                parser.parse_source(circuit.to_string(), Some(&Path::new(".")));
                parser.done().to_errors()?.type_check().to_errors()?;
            };
            if let Err(errors) = check {
                println!("{errors:#?}");
                Err(crate::Error::SomeError)
            }
            else {
                Ok(Some(circuit.to_string()))
            }
        }
    }
}
