use std::collections::HashMap;
use std::io::BufWriter;
use std::ops::Index;
use std::path::Path;
use std::rc::Rc;
use std::sync::Mutex;

use async_trait::async_trait;
use collection_literals::collection;
use fehler::{throw, throws};
use log::debug;
use openqasm::{
    Decl, Errors, GenericError, Linearize, Parser, Program, ProgramVisitor, SourceCache, Span,
};
use parser::ProgramParser;
use printers::ProgramPrinter;

use crate::args::CliArgsCircuit;
use crate::circuit_generators::base::printers::{GatePrinter, TemplatePrinter};
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::CircuitGenerator;
use crate::lang_schemas::{LangCircuit, OpenQasmSchema};
use crate::Error;
use crate::ext::LangSchema;
mod parser;
mod printers;

fn get_base_circ(path: &str) -> Result<Program, Error> {
    let mut circuit = std::fs::read_to_string(path)?;
    circuit.remove_matches("\r");

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


#[throws]
fn oqs_parse_circuit(
    oqs: &OpenQasmSchema,
    depth: i32,
    width: i32,
) -> (Vec<LangGate>, Vec<LangGate>) {
    let mut counts = HashMap::<i32, i32>::new();
    let mut gates = vec![];
    let mut inv_gates = vec![];


    for gate in &oqs.gates {
        if matches!(gate.t, LangGateType::Barrier) {
            gates.push(gate.clone());
            inv_gates.push(gate.inverse());
            continue;
        }

        let mut first_ok = true;
        let mut second_ok = true;

        let count = if let Some(c) = counts.get_mut(&gate.i) {
            *c
        }
        else {
            0
        };

        first_ok = count + 1 < depth && gate.i < width; // <= ??

        use LangGateType::*;
        match gate.t {
            Cx |
            Cz |
            Swap => {
                let count = if let Some(c) = counts.get_mut(&gate.other.unwrap()) {
                    *c
                }
                else {
                    0
                };

                second_ok = count + 1 < depth; // <= ??
            }
            _ => {}
        }

        // Do all indices fulfill the limit?
        if first_ok && second_ok {
            // Add limits for the next gate
            if let Some(c) = counts.get_mut(&gate.i) {
                *c += 1;
            }
            else {
                counts.insert(gate.i, 1);
            }

            match gate.t {
                Cx |
                Cz |
                Swap => {
                    if let Some(c) = counts.get_mut(&gate.other.unwrap()) {
                        *c += 1;
                    }
                    else {
                        counts.insert(gate.other.unwrap(), 1);
                    }
                }
                _ => {}
            }

            gates.push(gate.clone());
            inv_gates.push(gate.inverse());
        }
    }

    inv_gates.reverse();
    (gates, inv_gates)
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

        let mut oqs = OpenQasmSchema::from_path(&self.args.source)?;

        let (gates, inv_gates) = oqs_parse_circuit(&oqs, depth, width)?;

        let lang_circuit = LangCircuit::builder().width(width).gates(gates).inv_gates(inv_gates).build();
        let circuit = oqs.as_string(lang_circuit).await?;

        println!("{circuit}");
        Ok(Some(circuit))
    }
}
