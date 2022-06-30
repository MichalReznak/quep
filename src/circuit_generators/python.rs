//! # Python Circuit generator:
//! class CircuitGenerator:
//!     // TODO add init with args to all python versions
//!     def generate(i: int, j: int, it: int) -> dict[str, any]:
//!         return {'width': 4, 'gates': [{'t': 'X', 'i': 0, 'other': 0}]}

use async_trait::async_trait;
use fehler::throws;
use pyo3::prelude::*;
use pythonize::depythonize;

use crate::args::CliArgsCircuit;
use crate::ext::{CircuitGenerator, LangSchemaDyn};
use crate::lang_schemas::LangCircuit;
use crate::Error;

#[allow(dead_code)]
pub struct PythonCircuitGenerator {
    args: CliArgsCircuit,
    py_instance: PyObject,
}

#[allow(dead_code)]
impl PythonCircuitGenerator {
    #[throws]
    pub fn from_args(args: &CliArgsCircuit) -> Self {
        // TODO should add some type of path to file
        let py_instance = Python::with_gil(|py| {
            let code = std::fs::read_to_string("./python/ext/circuit_generator.py")?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("CircuitGenerator")?.into();
            qiskit.call0(py)
        })?;

        Self {
            args: args.clone(),
            py_instance,
        }
    }
}

#[async_trait]
impl CircuitGenerator for PythonCircuitGenerator {
    async fn generate(
        &mut self,
        _lang_schema: &LangSchemaDyn,
        i: i32,
        j: i32,
        iter: i32,
    ) -> Result<Option<LangCircuit>, Error> {
        Python::with_gil(|py| {
            let res = self.py_instance.call_method1(py, "generate", (i, j, iter))?;
            if res.is_none(py) {
                Ok(None)
            }
            else {
                Ok(Some(depythonize::<LangCircuit>(res.as_ref(py))?))
            }
        })
    }
}
