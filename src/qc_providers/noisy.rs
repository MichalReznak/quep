use std::time::Duration;

use async_trait::async_trait;
use log::debug;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tokio::time::Instant;

use crate::args::CliArgsProvider;
use crate::traits::QcProvider;
use crate::utils::debug;
use crate::Error;
use crate::Error::PyDowncastError;

pub struct NoisyQcProvider {
    args: CliArgsProvider,

    dur: Option<Instant>,
    py_instance: Option<PyObject>,
}

impl NoisyQcProvider {
    pub fn new(args: &CliArgsProvider) -> Self {
        Self {
            dur: None,
            py_instance: None,
            args: args.clone(),
        }
    }
}

#[async_trait]
impl QcProvider for NoisyQcProvider {
    async fn connect(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn set_circuit(&mut self, circuit: String) -> Result<(), Error> {
        Python::with_gil(|py| {
            let code = std::fs::read_to_string(&format!("{}/noisy.py", &self.args.python_dir))?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Noisy")?.into();
            let qiskit = qiskit.call0(py)?;

            qiskit.call_method0(py, "auth")?;

            qiskit.call_method1(py, "set_circuit", (circuit, debug()))?;

            self.py_instance = Some(qiskit);
            Ok(())
        })
    }

    async fn run(&self) -> Result<String, Error> {
        Python::with_gil(|py| {
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
