use std::collections::HashMap;
use std::path::Path;

use async_trait::async_trait;
use derive_more::Constructor;
use fehler::throw;
use openqasm as oq;
use openqasm::{Decl, Span, Stmt};
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
"#;

const CIRCUIT_RESULT: &str = r#"
OPENQASM 2.0;
include "qelib1.inc";

qreg q[%SIZE%];
creg c[%SIZE%];

%CIRCUIT%

barrier q;

%CIRCUIT_INV%

measure q -> c;
"#;


fn get_base_circ(i: i32) -> Result<oq::Program, Error> {
    let path = "./data/base.template.qasm"; // TODO arg
    let mut circuit = std::fs::read_to_string(path)?;
    circuit.remove_matches("\r");
    let circuit = CIRCUIT_PLACEHOLDER
        .replace("%SIZE%", &i.to_string())
        .replace("%CIRCUIT%", &circuit);

    let mut cache = oq::SourceCache::new();
    let mut parser = oq::Parser::new(&mut cache);
    parser.parse_source(circuit, Some(&Path::new(".")));

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
//      Number of qubits does not change

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

        // TODO barriers support
        // TODO different order of operations

        let program = get_base_circ(4)?; // TODO

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

        // println!("{program:#?}");

        let mut gates = vec![];
        let mut index_count = HashMap::new();

        for lex in program {
            match *lex {
                // isolate operations to a specific size
                Decl::Stmt(ref stmt) => {
                    match &**stmt {
                        Stmt::U { .. } => {}
                        Stmt::CX { .. } => {}

                        // For now only gates are used
                        Stmt::Gate { name, args, .. } => {
                            let name = (&**name).to_string();
                            // let args = &**args;
                            // TODO does not need to have index
                            let args: Vec<_> = (&**args)
                                .into_iter()
                                .map(|e| &**e)
                                .map(|e| (e.name.to_string(), e.index.unwrap()))
                                .collect();
                            println!("{name:#?}, {args:#?}");

                            gates.push((name, args.clone()));

                            for (_, arg) in args {
                                if let Some(i) = index_count.get_mut(&arg) {
                                    *i += 1;

                                    // TODO stop on some size
                                }
                                else {
                                    index_count.insert(arg, 1);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        println!("{index_count:#?}");
        for i in 0..4 {
            let arg = index_count.get(&i).unwrap_or_else(|| &0);
            println!("{arg}");
        }

        let mut inv_gates = gates.clone();
        inv_gates.reverse();

        println!("{gates:?}");
        println!("{inv_gates:?}");

        // TODO push to Program

        unimplemented!()
    }
}
