use std::time::Duration;

use async_trait::async_trait;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyInt, PyTuple};
use tokio::time::Instant;

use crate::traits::QcProvider;
use crate::Error;

pub struct NoisyQcProvider {
    dur: Option<Instant>,
}

impl NoisyQcProvider {
    pub fn new() -> Self {
        Self { dur: None }
    }
}

#[async_trait]
impl QcProvider for NoisyQcProvider {
    async fn connect(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn run(&self, circuit: String) -> Result<String, Error> {
        let res = Python::with_gil(|py| -> Result<_, Error> {
            let module =
                PyModule::from_code(py, include_str!("../../python/noisy/lib.py"), "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Qiskit")?.into();
            let qiskit = qiskit.call0(py)?;

            qiskit.call_method0(py, "auth")?;

            let args = PyTuple::new(py, &[&circuit]);
            let fun = qiskit.call_method1(py, "run", args)?;
            let res = fun.cast_as::<PyDict>(py).unwrap();

            let mut highest = ("".to_string(), 0);
            for (key, val) in res.iter() {
                let val: i32 = val.extract().unwrap();

                if val > highest.1 {
                    highest = (key.to_string(), val);
                }
            }

            println!("{res:#?}");
            Ok(format!("{}: {}", highest.0, highest.1))
        })?;

        Ok(res)
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
