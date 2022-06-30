//! # LangSchema interface:
//! TODO

use async_trait::async_trait;
use fehler::throws;
use pyo3::prelude::*;
use pyo3::{PyObject, Python};
use pythonize::{depythonize, pythonize};

use crate::error::Constraint;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::LangGate;
use crate::ext::LangSchema;
use crate::lang_schemas::LangCircuit;
use crate::Error;

pub struct PythonSchema {
    pub gates: Vec<LangGate>,
    py_instance: PyObject,
}

impl PythonSchema {
    #[throws]
    pub fn from_args() -> Self {
        // TODO should add some type of path to file
        let py_instance = Python::with_gil(|py| {
            let code = std::fs::read_to_string("./lang_schema.py")?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("LangSchema")?.into();
            qiskit.call0(py)
        })?;

        Self {
            gates: vec![],
            py_instance,
        }
    }
}

#[async_trait]
impl LangSchema for PythonSchema {
    async fn parse_file(&self, path: &str) -> Result<Vec<LangGate>, Error> {
        Python::with_gil(|py| {
            let res = self.py_instance.call_method1(py, "parse_file", (path,))?;
            if res.is_none(py) {
                Constraint {
                    reason: "Shouldn't be a constraint" // TODO
                        .to_string(),
                }
                .fail()?;
            }

            Ok(depythonize::<Vec<LangGate>>(res.as_ref(py))?)
        })
    }

    async fn as_string(&mut self, circ: LangCircuit) -> Result<GenCircuit, Error> {
        Python::with_gil(|py| {
            let circ = pythonize(py, &circ)?;
            let res = self.py_instance.call_method1(py, "as_string", (circ,))?;
            if res.is_none(py) {
                Constraint {
                    reason: "Shouldn't be a constraint" // TODO
                        .to_string(),
                }
                .fail()?;
            }

            Ok(depythonize::<GenCircuit>(res.as_ref(py))?)
        })
    }
}
