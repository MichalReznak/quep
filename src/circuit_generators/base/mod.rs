use std::collections::HashMap;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;
use std::sync::Mutex;

use async_trait::async_trait;
use collection_literals::collection;
use derive_more::Constructor;
use fehler::throw;
use openqasm as oq;
use openqasm::{Linearize, ProgramVisitor};
use oq::GenericError;

use crate::circuit_generators::base::gate_printer::{GatePrinter, ProgramParser, ProgramPrinter};
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

#[async_trait]
impl CircuitGenerator for BaseCircuitGenerator {
    async fn generate(&mut self, depth: i32, _width: i32, _iter: i32) -> Result<Option<String>, Error> {
        // TODO check circuit size
        // TODO barriers support
        // TODO different order of operations

        let mut program2 = get_base_circ(4)?; // TODO

        // println!("{name:?}: {args:?}");

        // let args: Vec<_> = (*args).iter().map(|e| &**e).collect();
        //
        // let mut inserts = collection! { HashMap<i32, i32> };
        //
        // // TODO do not depend on index
        // for arg in args {
        //     let i = arg.index.unwrap() as i32;
        //     match inserts.get_mut(&i) {
        //         None => { inserts.insert(i, 1); },
        //         Some(val) => { let mut a = *val; a += 1; inserts.insert(i, a); },
        //     };
        // }
        //
        // let any = inserts.iter().any(|(k, v)| {
        //     let counts = if let Some(a) = self.counts.get(k) {
        //         *a
        //     }
        //     else {
        //         0
        //     };
        //
        //     let inserts_count = if let Some(a) = inserts.get(k) {
        //         *a
        //     }
        //     else {
        //         0
        //     };
        //
        //     counts + inserts_count <= self.depth // TODO <= or < ??
        // });
        // if !any {
        //
        // }


        


        // let mut gates = vec![];
        // let mut index_count = HashMap::new();

        // println!("{index_count:#?}");
        // for i in 0..4 {
        //     let arg = index_count.get(&i).unwrap_or_else(|| &0);
        //     println!("{arg}");
        // }

        let mut program_parser = ProgramParser::new(4);
        program_parser.visit_program(&program2).unwrap();

        println!("{:?}", program_parser.counts);

        // TODO allow dynamic definition
        let inverse_gates = collection! {
            HashMap<&str, &str>;
            "s" => "sdg",
            "t" => "tdg",
        };

        let mut pp = ProgramPrinter::new();
        pp.visit_program(&program2).unwrap();

        let mut inv = program2.decls.clone();
        inv.reverse();
        program2.decls = inv;
        let mut pp_inv = ProgramPrinter::with_gates(inverse_gates);
        pp_inv.visit_program(&program2).unwrap();

        // println!("Normal:");
        // println!("{}", pp.result());
        // println!("INVERSE:");
        // println!("{}", pp_inv.result());

        let res = CIRCUIT_RESULT
            .replace("%SIZE%", &4.to_string())
            .replace("%CIRCUIT%", &pp.result())
            .replace("%CIRCUIT_INV%", &pp_inv.result());

        // println!("{res}");
        Ok(Some(res))
    }
}
