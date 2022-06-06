use std::fmt::Write;

use async_trait::async_trait;

use crate::args::types::CircuitSchemaType;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::LangSchema;
use crate::lang_schemas::LangCircuit;
use crate::Error;

// TODO do it gate independent
const CIRCUIT_TEMPLATE: &str = r#"
from qiskit import *

circ = QuantumCircuit(%WIDTH%, %WIDTH%)

%RESET%

circ.barrier()

%GATES%

circ.barrier()

%INV_GATES%

circ.barrier()

%MEASURE%
"#;

// TODO add all measures

// TODO remove fixed 0
fn gate_to_string(gate: &LangGate) -> String {
    // TODO add barrier

    use LangGateType::*;
    match gate.t {
        Id => format!("circ.id({})", gate.i),
        X => format!("circ.x({})", gate.i),
        Y => format!("circ.y({})", gate.i),
        Z => format!("circ.z({})", gate.i),
        H => format!("circ.h({})", gate.i),
        S => format!("circ.s({})", gate.i),
        Sdg => format!("circ.sdg({})", gate.i),
        Cx => format!("circ.cx({}, {})", gate.i, gate.other.unwrap()),
        Cz => format!("circ.cz({}, {})", gate.i, gate.other.unwrap()),
        Swap => format!("circ.swap({}, {})", gate.i, gate.other.unwrap()),
        Barrier => {
            if gate.i >= 0 {
                format!("circ.barrier({})", gate.i)
            }
            else {
                format!("circ.barrier()")
            }
        }
    }
}

pub struct QiskitSchema {
    pub gates: Vec<LangGate>,
}

impl QiskitSchema {
    pub fn new() -> Self {
        Self { gates: vec![] }
    }
}

// impl QiskitSchema {
//     #[throws]
//     pub fn from_path(path: &str) -> Self {
//         // // Type check
//         // let mut circuit = std::fs::read_to_string(path)?;
//         // circuit.remove_matches("\r");
//         //
//         // let mut cache = SourceCache::new();
//         // let mut parser = Parser::new(&mut cache);
//         // parser.parse_source(circuit, Some(&Path::new(".")));
//         //
//         // let check: Result<_, Errors> = try {
//         //     let res = parser.done().to_errors()?;
//         //     res.type_check().to_errors()?;
//         //     res
//         // };
//         //
//         // let program = {
//         //     if let Err(errors) = check {
//         //         errors.print(&mut cache)?;
//         //         throw!(crate::Error::SomeError)
//         //     }
//         //
//         //     check.unwrap()
//         // };
//         //
//         // let mut pp = parser::ProgramParser::new();
//         // pp.visit_program(&program)?;
//         //
//         // Self { gates: pp.gates }
//     }
// }

#[async_trait]
impl LangSchema for QiskitSchema {
    fn get_gates(&self) -> Vec<LangGate> {
        self.gates.clone()
    }

    async fn parse_file(&mut self, path: &str) -> Result<(), Error> {
        todo!("Cannot parse qiskit circuit")
    }

    // TODO check if is valid
    // TODO when openqasm lib is removed it can be as wasm module
    async fn as_string(&mut self, circ: LangCircuit) -> Result<GenCircuit, Error> {
        // Add width
        let res = CIRCUIT_TEMPLATE.replace("%WIDTH%", &circ.width.to_string());

        // Add resets
        let mut resets = String::new();
        for i in 0..circ.width {
            writeln!(&mut resets, "circ.reset({})", i)?;
        }
        let res = res.replace("%RESET%", &resets);

        // Add gates
        let mut gates = String::new();
        for gate in circ.gates {
            writeln!(&mut gates, "{}", gate_to_string(&gate))?;
        }
        let res = res.replace("%GATES%", &gates);

        // Add inverse gates
        let mut gates = String::new();
        if let Some(inv_gates) = circ.inv_gates {
            for gate in inv_gates {
                writeln!(&mut gates, "{}", gate_to_string(&gate))?;
            }
        }
        let res = res.replace("%INV_GATES%", &gates);

        let mut measures = String::new();
        for i in 0..circ.width {
            writeln!(&mut measures, "circ.measure({}, {})", i, i)?;
        }
        let res = res.replace("%MEASURE%", &measures);

        println!("{res}");

        Ok(GenCircuit::builder().circuit(res).t(CircuitSchemaType::Qiskit).build())
    }
}
