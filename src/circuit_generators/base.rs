use std::path::Path;

use async_trait::async_trait;
use derive_more::Constructor;
use fehler::{throw, throws};
use openqasm as oq;
use oq::GenericError;
use walkdir::{DirEntry, WalkDir};

use crate::traits::CircuitGenerator;
use crate::Error;

#[derive(Debug, Constructor)]
pub struct BaseCircuitGenerator;

#[throws]
fn get_base_circ() -> String {
    let path = "./data/base.qasm"; // TODO arg
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
        throw!(crate::Error::SomeError)
    }
    else {
        circuit.to_string()
    }
}

#[async_trait]
impl CircuitGenerator for BaseCircuitGenerator {
    async fn generate(&mut self, i: i32, j: i32, iter: i32) -> Result<Option<String>, Error> {
        // TODO check circuit size
        // if j >= self.entries.len() as i32 {
        //     Ok(None)
        // }
        // else {
        //
        // }

        let circuit = get_base_circ()?;

        // iterate the circuit
        // isolate operations to a specific size
        // create inverse circuit
        // add measurements

        unimplemented!()
    }
}
