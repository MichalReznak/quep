//! # Outputer interface:
//! class Outputer:
//!     def check_constraints(self) -> bool:
//!         return True
//!
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
use tokio::time::Duration;

use crate::args::CliArgsOutput;
use crate::error::Constraint;
use crate::ext::outputer::OutValue;
use crate::ext::Outputer;
use crate::{CliArgs, Error};

pub struct PythonOutputer {
    py_instance: PyObject,
}

impl PythonOutputer {
    #[throws]
    pub fn from_args(_args: &CliArgsOutput) -> Self {
        // TODO should add some type of path to file
        let py_instance = Python::with_gil(|py| {
            let code = std::fs::read_to_string("./python/ext/outputer.py")?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Outputer")?.into();
            qiskit.call0(py)
        })?;

        Self { py_instance }
    }
}

#[async_trait]
impl Outputer for PythonOutputer {
    fn check_constraints(&self, _args: &CliArgs) -> Result<(), Error> {
        Python::with_gil(|py| {
            if let Ok(method) = self.py_instance.getattr(py, "check_constraints") {
                if !method.call0(py)?.extract::<bool>(py)? {
                    Constraint {
                        reason: "TODO".to_string(),
                    }
                    .fail()?;
                }
            }
            Ok(())
        })
    }

    async fn output_table(
        &self,
        values: Vec<Vec<OutValue>>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<Option<String>, Error> {
        Python::with_gil(|py| {
            let durations =
                durations.map(|e| e.into_iter().map(|e| e.as_millis()).collect::<Vec<_>>());
            // TODO add durations and runtime
            let res = self.py_instance.call_method1(
                py,
                "output_table",
                (values, durations, runtime.as_millis()),
            )?;

            Ok(if res.is_none(py) {
                None
            }
            else {
                Some(res.to_string())
            })
        })
    }

    async fn output_volume(
        &self,
        values: Vec<OutValue>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<Option<String>, Error> {
        Python::with_gil(|py| {
            let durations =
                durations.map(|e| e.into_iter().map(|e| e.as_millis()).collect::<Vec<_>>());
            // TODO add durations and runtime
            let res = self.py_instance.call_method1(
                py,
                "output_volume",
                (values, durations, runtime.as_millis()),
            )?;

            Ok(if res.is_none(py) {
                None
            }
            else {
                Some(res.to_string())
            })
        })
    }

    async fn output_linear(
        &self,
        values: Vec<OutValue>,
        durations: Option<Vec<Duration>>,
        width: i32,
        runtime: Duration,
    ) -> Result<Option<String>, Error> {
        Python::with_gil(|py| {
            let durations =
                durations.map(|e| e.into_iter().map(|e| e.as_millis()).collect::<Vec<_>>());
            // TODO add durations and runtime
            let res = self.py_instance.call_method1(
                py,
                "output_linear",
                (values, durations, runtime.as_millis(), width),
            )?;

            Ok(if res.is_none(py) {
                None
            }
            else {
                Some(res.to_string())
            })
        })
    }
}
