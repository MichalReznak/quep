use std::time::Duration;

use async_trait::async_trait;
use ext::types::MetaInfo;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use snafu::OptionExt;

use crate::args::CliArgsProvider;
use crate::error::OutOfBounds;
use crate::ext::QcProvider;
use crate::utils::debug;
use crate::Error::PyDowncastError;
use crate::{ext, Error};
use crate::ext::types::circuit_generator::GenCircuit;

pub struct SimpleQcProvider {
    args: CliArgsProvider,

    py_instance: Option<PyObject>,
}

impl SimpleQcProvider {
    pub fn new(args: &CliArgsProvider) -> Self {
        Self {
            args: args.clone(),
            py_instance: None,
        }
    }
}

#[async_trait]
impl QcProvider for SimpleQcProvider {
    async fn connect(&mut self) -> Result<(), Error> {
        Python::with_gil(|py| {
            let code = std::fs::read_to_string(&format!("{}/simple.py", &self.args.python_dir))?;
            let module = PyModule::from_code(py, &code, "", "")?;
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
        let res = Python::with_gil(|py| -> Result<_, Error> {
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
                        let val: i32 = val.extract().unwrap();

                        if val > highest.1 {
                            highest = (key.to_string(), val);
                        }
                    }
                    format!("{}: {}", highest.0, highest.1)
                })
                .collect();
            Ok(res)
        })?;

        Python::with_gil(|py| -> Result<_, Error> {
            self.py_instance
                .as_ref()
                .context(OutOfBounds)?
                .call_method0(py, "clear_circuits")?;
            Ok(())
        })?;

        Ok(res)
    }

    async fn meta_info(&self) -> Result<MetaInfo, Error> {
        Python::with_gil(|py| -> Result<_, Error> {
            let res = self
                .py_instance
                .as_ref()
                .context(OutOfBounds)?
                .call_method0(py, "get_meta_info")?;
            let res = res.cast_as::<PyDict>(py).map_err(|_| PyDowncastError).unwrap();

            let time: f64 = res.get_item("time").unwrap().extract()?;
            let time = Duration::from_secs_f64(time);

            Ok(MetaInfo::builder().time(time).build())
        })
    }
}
