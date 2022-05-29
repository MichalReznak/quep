use async_trait::async_trait;
use log::debug;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::Python;
use snafu::OptionExt;
use tokio::time::{Duration, Instant};

use crate::args::CliArgsProvider;
use crate::error::OutOfBounds;
use crate::traits::QcProvider;
use crate::utils::debug;
use crate::Error;
use crate::Error::PyDowncastError;

pub struct IbmqQcProvider {
    args: CliArgsProvider,

    dur: Option<Instant>,
    py_instance: Option<PyObject>,
}

impl IbmqQcProvider {
    pub fn new(args: &CliArgsProvider) -> Self {
        Self {
            dur: None,
            py_instance: None,
            args: args.clone(),
        }
    }
}

#[async_trait]
impl QcProvider for IbmqQcProvider {
    async fn connect(&mut self) -> Result<(), Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            let code = std::fs::read_to_string(&format!("{}/ibmq.py", self.args.python_dir))?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Ibmq")?.into();
            let qiskit = qiskit.call0(py)?;

            qiskit.call_method0(py, "auth")?;
            self.py_instance = Some(qiskit);
            Ok(())
        })
    }

    async fn set_circuit(&mut self, circuit: String) -> Result<(), Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            self.py_instance.as_ref().context(OutOfBounds)?.call_method1(
                py,
                "set_circuit",
                (circuit, debug()),
            )?;
            Ok(())
        })
    }

    async fn append_circuit(&mut self, circuit: String) -> Result<(), Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            self.py_instance.as_ref().context(OutOfBounds)?.call_method1(
                py,
                "append_circuit",
                (circuit, debug()),
            )?;
            Ok(())
        })
    }

    async fn run(&self) -> Result<String, Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            let fun = self.py_instance.as_ref().context(OutOfBounds)?.call_method0(py, "run")?;
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

    async fn run_all(&self) -> Result<Vec<String>, Error> {
        Python::with_gil(|py| {
            let fun =
                self.py_instance.as_ref().context(OutOfBounds)?.call_method0(py, "run_all")?;
            let res = fun.cast_as::<PyList>(py).map_err(|_| PyDowncastError)?;
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
