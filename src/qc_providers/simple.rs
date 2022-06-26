use async_trait::async_trait;
use ext::types::MetaInfo;
use pyo3::prelude::*;
use snafu::OptionExt;

use crate::args::CliArgsProvider;
use crate::error::OutOfBounds;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::QcProvider;
use crate::utils::{debug, provider_meta_info, provider_run};
use crate::{ext, Error};

pub struct SimpleQcProvider {
    // args: CliArgsProvider,
    py_instance: Option<PyObject>,
}

impl SimpleQcProvider {
    pub fn new(_: &CliArgsProvider) -> Self {
        Self {
            // args: args.clone(),
            py_instance: None,
        }
    }
}

#[async_trait]
impl QcProvider for SimpleQcProvider {
    async fn connect(&mut self) -> Result<(), Error> {
        Python::with_gil(|py| {
            let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "./python/simple.py"));
            let module = PyModule::from_code(py, code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Simple")?.into();
            let qiskit = qiskit.call0(py)?;

            qiskit.call_method0(py, "auth")?;

            self.py_instance = Some(qiskit);
            Ok(())
        })
    }

    async fn append_circuit(&mut self, circuit: GenCircuit) -> Result<(), Error> {
        Python::with_gil(|py| {
            self.py_instance.as_ref().context(OutOfBounds)?.call_method1(
                py,
                "append_circuit",
                (circuit.circuit, circuit.t.to_string(), debug()),
            )?;
            Ok(())
        })
    }

    async fn run(&self) -> Result<Vec<String>, Error> {
        provider_run(self.py_instance.as_ref().context(OutOfBounds)?).await
    }

    async fn meta_info(&self) -> Result<MetaInfo, Error> {
        provider_meta_info(self.py_instance.as_ref().context(OutOfBounds)?).await
    }
}
