use async_trait::async_trait;
use derive_more::Constructor;
use fehler::{throw, throws};
use openqasm as oq;
use openqasm::{Decl, Span};
use oq::GenericError;

use crate::traits::CircuitGenerator;
use crate::Error;

#[derive(Debug, Constructor)]
pub struct BaseCircuitGenerator;

fn get_base_circ() -> Result<oq::Program, Error> {
    let path = "./data/base.qasm"; // TODO arg
                                   // let mut circuit = std::fs::read_to_string(path)?;
                                   // circuit.remove_matches("\r\n");

    let check: Result<_, oq::Errors> = try {
        let mut cache = oq::SourceCache::new();
        let mut parser = oq::Parser::new(&mut cache);
        parser.parse_file(path);
        let res = parser.done().to_errors()?;
        res.type_check().to_errors()?;
        res
    };

    match check {
        Err(errors) => {
            println!("{errors:#?}");
            throw!(crate::Error::SomeError)
        }
        Ok(check) => Ok(check),
    }
}

#[async_trait]
impl CircuitGenerator for BaseCircuitGenerator {
    async fn generate(&mut self, _i: i32, _j: i32, _iter: i32) -> Result<Option<String>, Error> {
        // TODO check circuit size
        // if j >= self.entries.len() as i32 {
        //     Ok(None)
        // }
        // else {
        //
        // }

        let program = get_base_circ()?;

        let program: Vec<_> = program
            .decls
            .into_iter()
            .filter(|e| {
                let Span { inner, .. } = e;

                use Decl::*;
                match **inner {
                    Include { .. } | Def { .. } => false,
                    CReg { .. } | QReg { .. } | Stmt(_) => true,
                }
            })
            .collect();

        println!("{program:#?}");

        // iterate the circuit
        // isolate operations to a specific size
        // create inverse circuit
        // add measurements

        unimplemented!()
    }
}
