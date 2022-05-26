use std::collections::{HashMap, HashSet};
use std::path::Path;

use async_trait::async_trait;
use collection_literals::collection;
use derive_more::Constructor;
use fehler::{throw, throws};
use itertools::Itertools;
use openqasm as oq;
use openqasm::{Program, ProgramVisitor};
use oq::GenericError;
use parser::ProgramParser;
use printers::ProgramPrinter;

use crate::traits::CircuitGenerator;
use crate::Error;

mod parser;
mod printers;

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

barrier q;

measure q -> c;
"#;

fn get_base_circ(i: i32) -> Result<oq::Program, Error> {
    let path = "./base.template.qasm"; // TODO arg
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
            errors.print(&mut cache)?;
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
// Non inverse gates are not handled
// For now only a single qreg and creg can be defined
// Gates needs to be defined only in an entry file (otherwise order is wrong)

#[throws]
pub fn parse_circuit(program: &Program, _depth: i32) -> HashSet<i32> {
    let mut program_parser = ProgramParser::new(2);
    program_parser.visit_program(&program).unwrap();

    println!("{:#?}", program_parser.counts);
    println!("{:#?}", program_parser.included_gates.clone().into_iter().sorted());

    program_parser.included_gates
}

#[throws]
pub fn print_circuit(included_gates: HashSet<i32>, program: &Program) -> String {
    // TODO allow dynamic definition
    let inverse_gates = collection! {
        HashMap<&str, &str>;
        "s" => "sdg",
        "t" => "tdg",
    };

    let mut pp = ProgramPrinter::new(included_gates.clone());
    pp.visit_program(&program).unwrap();

    let mut inv = program.decls.clone();
    inv.reverse();

    // TODO inverse gates are incorrect, counter needs to go from length to 0
    let mut program = program.clone();
    program.decls = inv;
    let mut pp_inv =
        ProgramPrinter::with_gates(included_gates, inverse_gates, pp.current_gate_i - 1);
    pp_inv.visit_program(&program).unwrap();

    // println!("Normal:");
    // println!("{}", pp.result());
    // println!("INVERSE:");
    // println!("{}", pp_inv.result());

    CIRCUIT_RESULT
        .replace("%SIZE%", &4.to_string())
        .replace("%CIRCUIT%", &pp.result())
        .replace("%CIRCUIT_INV%", &pp_inv.result())
}

#[async_trait]
impl CircuitGenerator for BaseCircuitGenerator {
    async fn generate(
        &mut self,
        depth: i32,
        _width: i32,
        _iter: i32,
        parse: bool,
    ) -> Result<Option<String>, Error> {
        // TODO check circuit size
        // TODO barriers support
        // TODO different order of operations

        let program2 = get_base_circ(4)?; // TODO

        let included_gates = parse_circuit(&program2, depth)?;

        let print_program = print_circuit(included_gates, &program2)?;
        println!("{print_program}");

        Ok(Some(print_program))
    }
}
