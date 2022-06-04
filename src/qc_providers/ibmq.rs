use async_trait::async_trait;
use log::info;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::Python;
use snafu::OptionExt;
use tokio::time::Duration;

use crate::args::CliArgsProvider;
use crate::error::OutOfBounds;
use crate::ext::types::MetaInfo;
use crate::ext::QcProvider;
use crate::utils::debug;
use crate::Error;
use crate::Error::PyDowncastError;

pub struct IbmqQcProvider {
    args: CliArgsProvider,

    py_instance: Option<PyObject>,
}

impl IbmqQcProvider {
    pub fn new(args: &CliArgsProvider) -> Self {
        Self {
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
            let qiskit = qiskit.call1(py, (&self.args.account_id,))?;

            qiskit.call_method0(py, "auth")?;
            self.py_instance = Some(qiskit);
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
                        info!("{key:#?}, {val:#?}");
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
