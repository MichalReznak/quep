//! # Outputer interface:
//! class Outputer:
//!     def output_table(self, values: [[dict[str, any]]], durations: [int],
//! runtime: int) -> str:         # Format table in any way
//!         return ""
//!
//!     def output_volume(self, values: [dict[str, any]], durations: [int],
//! runtime: int) -> str:         # Format volume in any way
//!         return ""
//!
//!     def output_linear(self, values: [dict[str, any]], durations: [int],
//! runtime: int, width: int) -> str:         # Format linear in any way
//!         return ""

use async_trait::async_trait;
use fehler::throws;
use pyo3::prelude::*;
use pythonize::pythonize;
use tokio::time::Duration;

use crate::args::CliArgsOutput;
use crate::error::Constraint;
use crate::ext::outputer::OutValue;
use crate::ext::Outputer;
use crate::Error;
pub struct PythonOutputer {
    py_instance: PyObject,
}

impl PythonOutputer {
    #[throws]
    pub fn from_args(_args: &CliArgsOutput) -> Self {
        // TODO should add some type of path to file
        let py_instance = Python::with_gil(|py| {
            let code = std::fs::read_to_string("./outputer.py")?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Outputer")?.into();
            qiskit.call0(py)
        })?;

        Self { py_instance }
    }
}

#[async_trait]
impl Outputer for PythonOutputer {
    async fn output_table(
        &self,
        values: Vec<Vec<OutValue>>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<String, Error> {
        Python::with_gil(|py| {
            let values = pythonize(py, &values)?;
            let durations =
                durations.map(|e| e.into_iter().map(|e| e.as_millis()).collect::<Vec<_>>());
            // TODO add durations and runtime
            let res = self.py_instance.call_method1(
                py,
                "output_table",
                (values, durations, runtime.as_millis()),
            )?;
            if res.is_none(py) {
                Constraint {
                    reason: "Shouldn't be a constraint" // TODO
                        .to_string(),
                }
                .fail()?;
            }

            Ok(res.to_string()) // TODO not sure if should be to_string
        })
    }

    async fn output_volume(
        &self,
        values: Vec<OutValue>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<String, Error> {
        Python::with_gil(|py| {
            let values = pythonize(py, &values)?;
            let durations =
                durations.map(|e| e.into_iter().map(|e| e.as_millis()).collect::<Vec<_>>());
            // TODO add durations and runtime
            let res = self.py_instance.call_method1(
                py,
                "output_volume",
                (values, durations, runtime.as_millis()),
            )?;
            if res.is_none(py) {
                Constraint {
                    reason: "Shouldn't be a constraint" // TODO
                        .to_string(),
                }
                .fail()?;
            }

            Ok(res.to_string()) // TODO not sure if should be to_string
        })
    }

    async fn output_linear(
        &self,
        values: Vec<OutValue>,
        durations: Option<Vec<Duration>>,
        width: i32,
        runtime: Duration,
    ) -> Result<String, Error> {
        Python::with_gil(|py| {
            let values = pythonize(py, &values)?;
            let durations =
                durations.map(|e| e.into_iter().map(|e| e.as_millis()).collect::<Vec<_>>());
            // TODO add durations and runtime
            let res = self.py_instance.call_method1(
                py,
                "output_linear",
                (values, durations, runtime.as_millis(), width),
            )?;
            if res.is_none(py) {
                Constraint {
                    reason: "Shouldn't be a constraint" // TODO
                        .to_string(),
                }
                .fail()?;
            }

            Ok(res.to_string()) // TODO not sure if should be to_string
        })
    }
}
