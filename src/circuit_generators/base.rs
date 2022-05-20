use async_trait::async_trait;
use derive_more::Constructor;
use fehler::{throw};
use openqasm as oq;
use openqasm::{Decl, Span};
use oq::GenericError;

use crate::traits::CircuitGenerator;
use crate::Error;

#[derive(Debug, Constructor)]
pub struct BaseCircuitGenerator;

// TODO allow user to define it
const CIRCUIT_PLACEHOLDER: &str = r#"
OPENQASM 2.0;
include "qelib1.inc";

qreg q[%SIZE%];
creg c[%SIZE%];

%CIRCUIT%

measure q -> c;
"#;

fn get_base_circ(i: i32) -> Result<oq::Program, Error> {
    let path = "./data/base.template.qasm"; // TODO arg
    let mut circuit = std::fs::read_to_string(path)?;
    circuit.remove_matches("\r");
    let circuit = CIRCUIT_PLACEHOLDER.replace("%CIRCUIT%", &circuit);
    let circuit = circuit.replace("%SIZE%", &i.to_string());

    let mut cache = oq::SourceCache::new();
    let mut parser = oq::Parser::new(&mut cache);
    parser.parse_source(circuit, Some("."));

    let check: Result<_, oq::Errors> = try {
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

// NOTES:
// order of reg defines order in circuit
// barrier inputs I-gates to unset ones
// Now restricted version -> single qreg[n] and creg[n]
//      Measure is done on all
//      They define only the circuit

#[async_trait]
impl CircuitGenerator for BaseCircuitGenerator {
    async fn generate(&mut self, i: i32, _j: i32, _iter: i32) -> Result<Option<String>, Error> {
        // TODO check circuit size
        // if j >= self.entries.len() as i32 {
        //     Ok(None)
        // }
        // else {
        //
        // }

        // TODO barriers support
        // TODO different order of operations

        let program = get_base_circ(i)?;

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
        for lex in program {
            use Decl::*;
            match *lex {
                Include { .. } => {}
                QReg { .. } => {}
                CReg { .. } => {}
                Def { .. } => {}
                Stmt(_) => {}
            }
        }

        // isolate operations to a specific size
        // create inverse circuit
        // add measurements

        unimplemented!()
    }
}
