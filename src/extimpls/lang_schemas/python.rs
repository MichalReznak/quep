//! # LangSchema interface:
//! class LangSchema:
//!     def __init__(self, args):
//!         pass
//!
//!     def check_constraints(self, config) -> bool:
//!         return {'correct': False, 'reason': 'Some reason'}
//!
//!     def parse_file(path: str) -> [dict[str, any]]:
//!         return [{'t': 'X', 'i': 0, 'other': 0}]
//!
//!     def as_string(circ: dict[str, any]) -> dict[str, any]:
//!         return {'circuit': '', 't': 'OpenQasm'}

use async_trait::async_trait;
use fehler::throws;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{pyclass, PyObject, Python};
use pythonize::depythonize;

use crate::args::CliArgsLangSchema;
use crate::error::Constraint;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::LangGate;
use crate::ext::LangSchema;
use crate::lang_schemas::LangCircuit;
use crate::Error::PyDowncastError;
use crate::{CliArgs, Error};

#[pyclass]
#[derive(Clone)]
pub struct PythonSchema {
    pub gates: Vec<LangGate>,
    py_instance: PyObject,
}

impl PythonSchema {
    #[throws]
    pub fn from_args(args: &CliArgsLangSchema) -> Self {
        let py_instance = Python::with_gil(|py| {
            let code = std::fs::read_to_string(&args.path)?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("LangSchema")?.into();
            qiskit.call1(py, (args.clone(),))
        })?;

        Self {
            gates: vec![],
            py_instance,
        }
    }
}

#[pymethods]
impl PythonSchema {
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
impl LangSchema for PythonSchema {
    fn check_constraints(&self, args: &CliArgs) -> Result<(), Error> {
        Python::with_gil(|py| {
            if let Ok(method) = self.py_instance.getattr(py, "check_constraints") {
                let res = method.call1(py, (args.clone(),))?;
                let res = res.cast_as::<PyDict>(py).map_err(|_| PyDowncastError).unwrap();
                if !res.get_item("correct").unwrap().extract::<bool>()? {
                    Constraint {
                        reason: res.get_item("reason").unwrap().to_string(),
                    }
                    .fail()?;
                }
            }
            Ok(())
        })
    }

    fn parse_file(&self, path: &str) -> Result<Vec<LangGate>, Error> {
        Python::with_gil(|py| {
            let res = self.py_instance.call_method1(py, "parse_file", (path,))?;
            if res.is_none(py) {
                Constraint {
                    reason: "Parse file error".to_string(),
                }
                .fail()?;
            }

            Ok(depythonize::<Vec<LangGate>>(res.as_ref(py))?)
        })
    }

    fn as_string(&mut self, circ: LangCircuit) -> Result<GenCircuit, Error> {
        Python::with_gil(|py| {
            let res = self.py_instance.call_method1(py, "as_string", (circ,))?;
            if res.is_none(py) {
                Constraint {
                    reason: "Stringification error".to_string(),
                }
                .fail()?;
            }

            Ok(depythonize::<GenCircuit>(res.as_ref(py))?)
        })
    }
}
