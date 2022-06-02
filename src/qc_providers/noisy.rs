use std::time::Duration;

use async_trait::async_trait;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use snafu::OptionExt;
use tokio::time::Instant;

use crate::args::CliArgsProvider;
use crate::error::OutOfBounds;
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
    async fn connect(&mut self) -> Result<(), Error> {
        Python::with_gil(|py| {
            let code = std::fs::read_to_string(&format!("{}/noisy.py", &self.args.python_dir))?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Noisy")?.into();
            let qiskit = qiskit.call0(py)?;

            qiskit.call_method0(py, "auth")?;

            self.py_instance = Some(qiskit);
            Ok(())
        })
    }

    async fn set_circuit(&mut self, circuit: String) -> Result<(), Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            self.py_instance
                .as_ref()
                .context(OutOfBounds)?
                .call_method0(py, "clear_circuits")?;
            Ok(())
        })?;
        self.append_circuit(circuit).await
    }

    async fn append_circuit(&mut self, circuit: String) -> Result<(), Error> {
        Python::with_gil(|py| {
            self.py_instance.as_ref().context(OutOfBounds)?.call_method1(
                py,
                "append_circuit",
                (circuit, debug()),
            )?;
            Ok(())
        })
    }

    async fn run(&self) -> Result<String, Error> {
        Ok(self.run_all().await?.get(0).unwrap().to_string())
    }

    async fn run_all(&self) -> Result<Vec<String>, Error> {
        Python::with_gil(|py| {
            let fun =
                self.py_instance.as_ref().context(OutOfBounds)?.call_method0(py, "run_all")?;

            // Change to vector
            // TODO better?
            let res: Vec<_> = if let Ok(list) = fun.cast_as::<PyList>(py) {
                list.into_iter().collect()
            }
            else {
                vec![fun.cast_as::<PyDict>(py).map_err(|_| PyDowncastError).unwrap()]
            };

            let res: Vec<_> = res
                .into_iter()
                .map(|e| e.cast_as::<PyDict>().map_err(|_| PyDowncastError).unwrap())
                .map(|e| {
                    let mut highest = ("".to_string(), 0);
                    for (key, val) in e.into_iter() {
                        println!("{key:#?}, {val:#?}");
                        let val: i32 = val.extract().unwrap();

                        if val > highest.1 {
                            highest = (key.to_string(), val);
                        }
                    }
                    format!("{}: {}", highest.0, highest.1)
                })
                .collect();
            Ok(res)
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
