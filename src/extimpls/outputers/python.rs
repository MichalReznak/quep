// # Outputer interface:
// class Outputer:
//     def __init__(self, args):
//         pass
//
//     def check_constraints(self, config) -> bool:
//         return {'correct': False, 'reason': 'Some reason'}
//
//     def output_table(self, values: [[dict[str, any]]],
//         durations: [int], runtime: int) -> str:
//         # Format table in any way
//         return ''
//
//     def output_volume(self, values: [dict[str, any]],
//         durations: [int], runtime: int) -> str:
//         # Format volume in any way
//         return ''
//
//     def output_linear(self, values: [dict[str, any]],
//     durations: [int], runtime: int, width: int) -> str:
//         # Format linear in any way
//         return ''

use async_trait::async_trait;
use fehler::throws;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tokio::time::Duration;

use crate::args::CliArgsOutput;
use crate::error::Constraint;
use crate::ext::outputer::OutValue;
use crate::ext::Outputer;
use crate::Error::PyDowncastError;
use crate::{CliArgs, Error};

pub struct PythonOutputer {
    py_instance: PyObject,
}

impl PythonOutputer {
    #[throws]
    pub fn from_args(args: &CliArgsOutput) -> Self {
        let py_instance = Python::with_gil(|py| {
            let code = std::fs::read_to_string(&args.path)?;
            let module = PyModule::from_code(py, &code, "", "")?;
            let qiskit: Py<PyAny> = module.getattr("Outputer")?.into();
            qiskit.call1(py, (args.clone(),))
        })?;

        Self { py_instance }
    }
}

#[async_trait]
impl Outputer for PythonOutputer {
    fn check_constraints(&self, args: &CliArgs) -> Result<(), Error> {
        Python::with_gil(|py| {
            if let Ok(method) = self.py_instance.getattr(py, "check_constraints") {
                let res = method.call1(py, (args.clone(),))?;
                let res = res.cast_as::<PyDict>(py).map_err(|_| PyDowncastError).unwrap();
                if !res.get_item("correct").unwrap().extract::<bool>()? {
                    Constraint {
                        reason: res.get_item("reason").unwrap().to_string(),
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
        from: i32,
        from2: i32,
    ) -> Result<Option<String>, Error> {
        Python::with_gil(|py| {
            let durations =
                durations.map(|e| e.into_iter().map(|e| e.as_millis()).collect::<Vec<_>>());
            let res = self.py_instance.call_method1(
                py,
                "output_table",
                (values, durations, runtime.as_millis(), from, from2),
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
        from: i32,
    ) -> Result<Option<String>, Error> {
        Python::with_gil(|py| {
            let durations =
                durations.map(|e| e.into_iter().map(|e| e.as_millis()).collect::<Vec<_>>());
            let res = self.py_instance.call_method1(
                py,
                "output_volume",
                (values, durations, runtime.as_millis(), from),
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
        from: i32,
    ) -> Result<Option<String>, Error> {
        Python::with_gil(|py| {
            let durations =
                durations.map(|e| e.into_iter().map(|e| e.as_millis()).collect::<Vec<_>>());
            let res = self.py_instance.call_method1(
                py,
                "output_linear",
                (values, durations, width, runtime.as_millis(), from),
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
