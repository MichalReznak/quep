use async_trait::async_trait;
use load_file::load_str;
use log::debug;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::Python;
use tokio::time::{Duration, Instant};

use crate::traits::QcProvider;
use crate::utils::debug;
use crate::Error;
use crate::Error::PyDowncastError;

pub struct IbmqQcProvider {
    dur: Option<Instant>,
    dir: String,
    py_instance: Option<PyObject>,
}

impl IbmqQcProvider {
    pub fn new(dir: &str) -> Self {
        Self {
            dur: None,
            py_instance: None,
            dir: dir.to_string(),
        }
    }
}

#[async_trait]
impl QcProvider for IbmqQcProvider {
    async fn connect(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn set_circuit(&mut self, circuit: String) -> Result<(), Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            let code = load_str!(&format!("{}/ibmq.py", self.dir));
            let module = PyModule::from_code(py, code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Ibmq")?.into();
            let qiskit = qiskit.call0(py)?;

            qiskit.call_method0(py, "auth")?;
            qiskit.call_method1(py, "set_circuit", (circuit, debug()))?;

            self.py_instance = Some(qiskit);
            Ok(())
        })
    }

    async fn run(&self) -> Result<String, Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            let fun = self.py_instance.as_ref().unwrap().call_method0(py, "run")?;
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
