use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;
use std::sync::Mutex;

use async_trait::async_trait;
use fehler::{throw, throws};
use log::debug;
use openqasm::{
    Decl, Errors, GenericError, Linearize, Parser, Program, ProgramVisitor, SourceCache, Span,
};
use parser::ProgramParser;
use printers::ProgramPrinter;

use crate::args::CliArgsCircuit;
use crate::circuit_generators::base::printers::{GatePrinter, TemplatePrinter};
use crate::traits::CircuitGenerator;
use crate::Error;

mod parser;
mod printers;

fn get_base_circ(path: &str) -> Result<Program, Error> {
    let mut circuit = std::fs::read_to_string(path)?;
    circuit.remove_matches("\r");

    // TODO allow to define size, or increase until circuit can be parsed?
    // let circuit = CIRCUIT_PLACEHOLDER
    //     .replace("%SIZE%", &64.to_string())
    //     .replace("%CIRCUIT%", &circuit);

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
pub fn parse_circuit(program: &Program, depth: i32, width: i32) -> Vec<Span<Decl>> {
    let mut program_parser = ProgramParser::new(depth, width);
    program_parser.visit_program(program)?;
    program_parser.parsed_gates(program)
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

    #[throws]
    fn print_circuit(
        &self,
        program: &Program,
        template: String,
        _size: i32,
        parse: bool,
    ) -> String {
        if parse {
            // TODO cannot be run, does not output runnable circuit
            let buf = Rc::new(Mutex::new(BufWriter::new(vec![])));
            let mut gp = Linearize::new(GatePrinter::new(buf.clone()));
            gp.visit_program(program).unwrap();
            let mut lock = buf.lock().unwrap();
            let buf = lock.get_mut();
            String::from_utf8(buf.clone())?

            // TODO inverse only gates
            // let mut inv = program.decls.clone();
            // inv.reverse();
            //
            // let mut program = program.clone();
            // program.decls = inv; // TODO inverse gates
            // let buf = Rc::new(Mutex::new(BufWriter::new(vec![])));
            //
            // let mut gp_inv = Linearize::new(GatePrinter::new(buf.clone()));
            // gp_inv.visit_program(&program)?;
            // let mut lock = buf.lock().unwrap();
            // let buf = lock.get_mut();
            // let gp_inv_result = String::from_utf8(buf.clone())?;

            // CIRCUIT_RESULT.replace("%SIZE%", &size.to_string())
            // .replace("%CIRCUIT%", &gp_result)
            // .replace("%CIRCUIT_INV%", &gp_inv_result)
        }
        else {
            let mut pp = ProgramPrinter::new();
            pp.visit_program(program)?;

            let mut inv = program.decls.clone();
            inv.reverse();

            let mut program = program.clone();
            program.decls = inv;

            // TODO only inverse gates
            let mut pp_inv = ProgramPrinter::with_gates(self.args.inverse_gates.clone());
            pp_inv.visit_program(&program)?;

            template
                .replace("%GATES%", &pp.result()?)
                .replace("%GATES_INV%", &pp_inv.result()?)
        }
    }
}

#[async_trait]
impl CircuitGenerator for BaseCircuitGenerator {
    async fn generate(
        &mut self,
        depth: i32,
        width: i32,
        _iter: i32, // TODO
    ) -> Result<Option<String>, Error> {
        let depth = depth + 1;
        let width = width + 1;
        // TODO check circuit size
        // TODO barriers support
        // TODO different order of operations

        let mut program2 = get_base_circ(&self.args.source)?;

        // TODO create template from base circuit and then include parsed gates
        let mut tp = TemplatePrinter::new(width);
        tp.visit_program(&program2)?;
        debug!("{}", tp.result()?);

        program2.decls = parse_circuit(&program2, depth, width)?;

        let print_program = self.print_circuit(&program2, tp.result()?, width, self.args.parse)?;
        debug!("{print_program}");

        Ok(Some(print_program))
    }
}
