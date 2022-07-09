use std::fmt::Write;
use std::path::Path;

use async_trait::async_trait;
use fehler::{throw, throws};
use openqasm::{Errors, GenericError, Parser, ProgramVisitor, SourceCache};
use pyo3::{pyclass, pymethods, PyResult};

use crate::args::types::LangSchemaType;
use crate::args::CliArgsLangSchema;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::{LangGate, LangGateType};
use crate::ext::LangSchema;
use crate::lang_schemas::LangCircuit;
use crate::Error;

mod parser;

const CIRCUIT_TEMPLATE: &str = r#"
OPENQASM 2.0;
include "qelib1.inc";

qreg q[%WIDTH%];
creg c[%WIDTH%];

%RESET%

barrier q;

%GATES%

barrier q;

measure q -> c;
"#;

fn gate_to_string(gate: &LangGate) -> String {
    use LangGateType::*;
    match gate.t {
        Id => format!("id q[{}];", gate.i),
        X => format!("x q[{}];", gate.i),
        Y => format!("y q[{}];", gate.i),
        Z => format!("z q[{}];", gate.i),
        H => format!("h q[{}];", gate.i),
        S => format!("s q[{}];", gate.i),
        Sdg => format!("sdg q[{}];", gate.i),
        Cx => format!("cx q[{}], q[{}];", gate.i, gate.other.unwrap()),
        Cz => format!("cz q[{}], q[{}];", gate.i, gate.other.unwrap()),
        Swap => format!("swap q[{}], q[{}];", gate.i, gate.other.unwrap()),
        Barrier => {
            if gate.i >= 0 {
                format!("barrier q[{}];", gate.i)
            }
            else {
                "barrier q;".to_string()
            }
        }
        Dummy => panic!(),
    }
}

#[pyclass]
#[derive(Default, Clone)]
pub struct OpenQasmSchema {
    pub gates: Vec<LangGate>,
}

impl OpenQasmSchema {
    #[throws]
    pub fn from_args(_: &CliArgsLangSchema) -> Self {
        Self { gates: vec![] }
    }
}

#[pymethods]
impl OpenQasmSchema {
    #[pyo3(name = "parse_file")]
    fn parse_file_py(&self, path: &str) -> PyResult<Vec<LangGate>> {
        Ok(self.parse_file(path).unwrap())
    }

    #[pyo3(name = "as_string")]
    fn as_string_py(&mut self, circ: LangCircuit) -> PyResult<GenCircuit> {
        Ok(self.as_string(circ).unwrap())
    }
}

#[async_trait]
impl LangSchema for OpenQasmSchema {
    fn parse_file(&self, path: &str) -> Result<Vec<LangGate>, Error> {
        // Type check
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

        let program = {
            if let Err(errors) = check {
                errors.print(&mut cache)?;
                throw!(crate::Error::SomeError)
            }

            check.unwrap()
        };

        let mut pp = parser::ProgramParser::new();
        pp.visit_program(&program)?;
        Ok(pp.gates)
    }

    fn as_string(&mut self, circ: LangCircuit) -> Result<GenCircuit, Error> {
        // Add width
        let res = CIRCUIT_TEMPLATE.replace("%WIDTH%", &circ.width.to_string());

        // Add resets
        let mut resets = String::new();
        for i in 0..circ.width {
            writeln!(&mut resets, "reset q[{}];", i)?;
        }
        let res = res.replace("%RESET%", &resets);

        // Add gates
        let mut gates = String::new();
        for gate in circ.gates {
            writeln!(&mut gates, "{}", gate_to_string(&gate))?;
        }
        let res = res.replace("%GATES%", &gates);

        Ok(GenCircuit::builder().circuit(res).t(LangSchemaType::OpenQasm).build())
    }
}
