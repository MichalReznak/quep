use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;
use std::sync::Mutex;

use async_trait::async_trait;
use derive_more::Constructor;
use fehler::throw;
use openqasm as oq;
use openqasm::{Linearize, ProgramVisitor};
use oq::GenericError;

use crate::circuit_generators::base::gate_printer::{GatePrinter, ProgramPrinter};
use crate::traits::CircuitGenerator;
use crate::Error;

mod gate_printer;

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

const _CIRCUIT_RESULT: &str = r#"
OPENQASM 2.0;
include "qelib1.inc";

qreg q[%SIZE%];
creg c[%SIZE%];

%CIRCUIT%

barrier q;

%CIRCUIT_INV%

measure q -> c;
"#;

// fn construct_circuit(circuit: Vec<>)

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

        let mut program2 = get_base_circ(4)?; // TODO

        // let program: Vec<_> = program2
        //     .decls
        //     .clone()
        //     .into_iter()
        //     .filter(|e| {
        //         let Span { inner, .. } = e;
        //
        //         use Decl::*;
        //         match **inner {
        //             Include { .. } | Def { .. } => false,
        //             CReg { .. } | QReg { .. } | Stmt(_) => true,
        //         }
        //     })
        //     .collect();

        // println!("{program:#?}");

        // let mut gates = vec![];
        // let mut index_count = HashMap::new();

        // println!("{index_count:#?}");
        // for i in 0..4 {
        //     let arg = index_count.get(&i).unwrap_or_else(|| &0);
        //     println!("{arg}");
        // }

        // let mut inv_gates = gates.clone();
        // inv_gates.reverse();

        // println!("{gates:?}");
        // println!("{inv_gates:?}");

        // let result: Vec<_> =
        // gates.into_iter().chain(inv_gates.into_iter()).collect();
        //
        // println!("{result:?}");

        let mut inv = program2.decls.clone();
        inv.reverse();

        program2.decls = program2.decls.clone().into_iter().chain(inv.into_iter()).collect();

        let mut pp = ProgramPrinter::new();
        pp.visit_program(&program2).unwrap(); // TODO can be used for parsing without gate change

        println!("{}", pp.result());

        let buf = Rc::new(Mutex::new(BufWriter::new(Vec::new())));

        let mut _l = Linearize::new(GatePrinter::new(buf.clone()));
        // l.visit_program(&program2).unwrap();
        //
        // // TODO now all gates are parsed to the base form, maybe an option to keep it
        // as defined? let bytes = buf.lock().unwrap().get_mut().clone();
        // let string = String::from_utf8(bytes).unwrap();
        // println!("{string}");

        unimplemented!()
    }
}
