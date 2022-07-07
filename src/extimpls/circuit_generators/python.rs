//! # Python Circuit generator:
//! class CircuitGenerator:
//!     def __init__(self, args):
//!         pass
//!
//!     def check_constraints(self, config) -> dict[str, any]:
//!         return {'correct: False, 'reason': 'Some reason'}
//!
//!     def generate(self, lang_schema: any, i: int, j: int, it: int) -> dict[str, any]:
//!         return {'width': 4, 'gates': [{'t': 'X', 'i': 0, 'other': 0}]}

use async_trait::async_trait;
use fehler::throws;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pythonize::depythonize;

use crate::args::CliArgsCircuit;
use crate::error::Constraint;
use crate::ext::{CircuitGenerator, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::Error::PyDowncastError;
use crate::{CliArgs, Error};

#[allow(dead_code)]
pub struct PythonCircuitGenerator {
    args: CliArgsCircuit,
    py_instance: PyObject,
}

#[allow(dead_code)]
impl PythonCircuitGenerator {
    #[throws]
    pub fn from_args(args: &CliArgsCircuit) -> Self {
        let py_instance = Python::with_gil(|py| {
            let code = std::fs::read_to_string(&args.path)?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("CircuitGenerator")?.into();
            qiskit.call1(py, (args.clone(),))
        })?;

        Self {
            args: args.clone(),
            py_instance,
        }
    }
}

#[async_trait]
impl CircuitGenerator for PythonCircuitGenerator {
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

    async fn generate(
        &mut self,
        lang_schema: &LangSchemaDyn,
        i: i32,
        j: i32,
        iter: i32,
    ) -> Result<Option<LangCircuit>, Error> {
        Python::with_gil(|py| {
            use LangSchemaDyn::*;
            let res = match lang_schema {
                OpenQasmSchema(a) => {
                    self.py_instance.call_method1(py, "generate", (a.clone(), i, j, iter))
                }
                QiskitSchema(a) => {
                    self.py_instance.call_method1(py, "generate", (a.clone(), i, j, iter))
                }
                PythonSchema(a) => {
                    self.py_instance.call_method1(py, "generate", (a.clone(), i, j, iter))
                }
            }?;

            if res.is_none(py) {
                Ok(None)
            }
            else {
                Ok(Some(depythonize::<LangCircuit>(res.as_ref(py))?))
            }
        })
    }
}
