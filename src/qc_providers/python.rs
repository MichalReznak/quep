//! # Python implementation provider:
//! class QcProvider:
//!     def get_meta_info(self):
//!         return <meta-info>
//!
//!     def auth(self):
//!         # Any authorization necessary
//!
//!     def clear_circuits(self: 'QcProvider'):
//!         # Clear circuits
//!
//!     def append_circuit(self: 'QcProvider',
//!             circuit: str, lang_schema_type: str, log: bool):
//!         # These are the circuits that will be executed
//!
//!     def run_all(self: 'QcProvider') -> str:
//!         # Execute on provider and return the result
//!         # in a form of Dict[str, int]

use async_trait::async_trait;
use fehler::throws;
use pyo3::prelude::*;

use crate::args::CliArgsProvider;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::MetaInfo;
use crate::ext::QcProvider;
use crate::utils::{debug, provider_meta_info, provider_run};
use crate::Error;

pub struct PythonQcProvider {
    // args: CliArgsProvider,
    py_instance: PyObject,
}

impl PythonQcProvider {
    #[throws]
    pub fn from_args(args: &CliArgsProvider) -> Self {
        let py_instance = Python::with_gil(|py| {
            // TODO should be python dir?
            let code = std::fs::read_to_string(&format!("{}/qc_provider.py", args.python_dir))?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("QcProvider")?.into();
            qiskit.call0(py)
        })?;

        Self {
            py_instance,
            // args: args.clone(),
        }
    }
}

#[async_trait]
impl QcProvider for PythonQcProvider {
    async fn connect(&mut self) -> Result<(), Error> {
        Python::with_gil(|py| {
            self.py_instance.call_method0(py, "auth")?;
            Ok(())
        })
    }

    async fn append_circuit(&mut self, circuit: GenCircuit) -> Result<(), Error> {
        Python::with_gil(|py| {
            self.py_instance.call_method1(
                py,
                "append_circuit",
                (circuit.circuit, circuit.t.to_string(), debug()),
            )?;
            Ok(())
        })
    }

    async fn run(&self) -> Result<Vec<String>, Error> {
        provider_run(&self.py_instance).await
    }

    async fn meta_info(&self) -> Result<MetaInfo, Error> {
        provider_meta_info(&self.py_instance).await
    }
}
