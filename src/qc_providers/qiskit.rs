use std::time::Duration;

use async_trait::async_trait;
use load_file::load_str;
use log::debug;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use tokio::time::Instant;

use crate::traits::QcProvider;
use crate::Error;
use crate::Error::PyDowncastError;

pub struct QiskitQcProvider {
    dur: Option<Instant>,
    dir: String,
}

impl QiskitQcProvider {
    pub fn new(dir: &str) -> Self {
        Self {
            dur: None,
            dir: dir.to_string(),
        }
    }
}

#[async_trait]
impl QcProvider for QiskitQcProvider {
    async fn connect(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn run(&self, circuit: String) -> Result<String, Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            let code = load_str!(&format!("{}/qiskit.py", &self.dir));
            let module = PyModule::from_code(py, code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Qiskit")?.into();
            let qiskit = qiskit.call0(py)?;

            qiskit.call_method0(py, "auth")?;

            let args = PyTuple::new(py, &[&circuit]);
            let fun = qiskit.call_method1(py, "run", args)?;
            let res = fun.cast_as::<PyDict>(py).map_err(|_| PyDowncastError)?;

            let mut highest = ("".to_string(), 0);
            for (key, val) in res.iter() {
                let val: i32 = val.extract()?;

                if val > highest.1 {
                    highest = (key.to_string(), val);
                }
            }

            debug!("{res:#?}");
            Ok(format!("{}: {}", highest.0, highest.1))
        })
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
