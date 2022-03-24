use std::time::Duration;

use async_trait::async_trait;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};
use tokio::time::Instant;

use crate::Error;
use crate::traits::QcProvider;

pub struct QiskitQcProvider {
    dur: Option<Instant>,
}

impl QiskitQcProvider {
    pub fn new() -> Self {
        Self { dur: None }
    }
}

#[async_trait]
impl QcProvider for QiskitQcProvider {
    async fn connect(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn run(&self, circuit: String) -> Result<String, Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            let module =
                PyModule::from_code(py, include_str!("../../python/src/qiskit.py"), "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Qiskit")?.into();
            let qiskit = qiskit.call0(py)?;

            qiskit.call_method0(py, "auth")?;

            let args = PyTuple::new(py, &[&circuit]);
            let fun = qiskit.call_method1(py, "run", args)?;
            let res = fun.cast_as::<PyString>(py).unwrap();
            println!("{res:#?}");
            Ok(())
        })?;

        Ok(circuit)
    }

    fn start_measure(&mut self) {
        self.dur = Some(Instant::now());
    }

    fn stop_measure(&mut self) -> Duration {
        if let Some(dur) = self.dur {
            Instant::now() - dur
        }
        else {
            unreachable!()
        }
    }
}
