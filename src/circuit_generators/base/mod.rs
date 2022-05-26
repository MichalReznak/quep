use std::collections::{HashMap, HashSet};
use std::path::Path;

use async_trait::async_trait;
use collection_literals::collection;
use fehler::{throw, throws};
use openqasm::{Errors, GenericError, Parser, Program, ProgramVisitor, SourceCache};
use parser::ProgramParser;
use printers::ProgramPrinter;

use crate::args::CliArgsCircuit;
use crate::traits::CircuitGenerator;
use crate::Error;

mod parser;
mod printers;

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

fn get_base_circ(path: &str) -> Result<Program, Error> {
    let mut circuit = std::fs::read_to_string(path)?;
    circuit.remove_matches("\r");

    // TODO allow to define size, or increase until circuit can be parsed?
    let circuit = CIRCUIT_PLACEHOLDER
        .replace("%SIZE%", &64.to_string())
        .replace("%CIRCUIT%", &circuit);

    let mut cache = SourceCache::new();
    let mut parser = Parser::new(&mut cache);
    parser.parse_source(circuit, Some(&Path::new(".")));

    let check: Result<_, Errors> = try {
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

#[throws]
pub fn parse_circuit(program: &Program, depth: i32, width: i32) -> HashSet<i32> {
    let mut program_parser = ProgramParser::new(depth, width);
    program_parser.visit_program(&program).unwrap();
    program_parser.included_gates
}

#[throws]
pub fn print_circuit(included_gates: HashSet<i32>, program: &Program, size: i32) -> String {
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

    let mut program = program.clone();
    program.decls = inv;
    let mut pp_inv =
        ProgramPrinter::with_gates(included_gates, inverse_gates, pp.current_gate_i - 1);
    pp_inv.visit_program(&program).unwrap();

    CIRCUIT_RESULT
        .replace("%SIZE%", &size.to_string())
        .replace("%CIRCUIT%", &pp.result())
        .replace("%CIRCUIT_INV%", &pp_inv.result())
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
// Depth is number of gates on each qubit it's not with automatic identity gates

#[allow(dead_code)]
#[derive(Debug)]
pub struct BaseCircuitGenerator {
    args: CliArgsCircuit,
}

impl BaseCircuitGenerator {
    pub fn new(args: &CliArgsCircuit) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl CircuitGenerator for BaseCircuitGenerator {
    async fn generate(
        &mut self,
        depth: i32,
        width: i32,
        _iter: i32,   // TODO
        _parse: bool, // TODO
    ) -> Result<Option<String>, Error> {
        let depth = depth + 1;
        let width = width + 1;
        // TODO check circuit size
        // TODO barriers support
        // TODO different order of operations

        let program2 = get_base_circ(&self.args.source)?;

        let included_gates = parse_circuit(&program2, depth, width)?;

        let print_program = print_circuit(included_gates, &program2, width)?;
        println!("{print_program}");

        Ok(Some(print_program))
    }
}
