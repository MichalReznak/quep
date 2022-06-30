use async_trait::async_trait;
use fehler::throws;
use pyo3::prelude::*;

use crate::args::CliArgsProvider;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::MetaInfo;
use crate::ext::QcProvider;
use crate::utils::{debug, provider_meta_info, provider_run};
use crate::Error;

pub struct NoisyQcProvider {
    // args: CliArgsProvider,
    py_instance: PyObject,
}

impl NoisyQcProvider {
    #[throws]
    pub fn from_args(_: &CliArgsProvider) -> Self {
        let py_instance = Python::with_gil(|py| {
            let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "./python/noisy.py"));
            let module = PyModule::from_code(py, code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Noisy")?.into();
            qiskit.call0(py)
        })?;

        Self {
            py_instance,
            // args: args.clone(),
        }
    }
}

#[async_trait]
impl QcProvider for NoisyQcProvider {
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
