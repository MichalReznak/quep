///! Example usage
// import asyncio
// import quepy
//
// async def main():
//     c = quepy.QuepyConfig(a=42)
//     print(await quepy.run(c))
//
// asyncio.run(main())

use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use quep_core::args::CliArgsConfig;
use quep_core::args::types::OutputType;
use quep_core::CliArgs;

#[pyclass]
#[derive(Copy, Clone)]
struct QuepyConfig {
    pub a: i32,
    // TODO
}

#[pymethods]
impl QuepyConfig {
    #[new]
    fn new(a: i32) -> Self {
        Self {
            a,
        }
    }
}


/// Run new Quep instance and use configuration from arg
#[pyfunction]
fn run(py: Python<'_>, config: QuepyConfig) -> PyResult<&PyAny> {
    future_into_py(py, async move {
        let mut args = CliArgs::parse_with_config_no_env(CliArgsConfig::default()).unwrap();
        args.output.t = OutputType::Serial;

        let res = quep_core::Quep::new(args).await.unwrap().run().await.unwrap();
        Ok(Python::with_gil(|py| res))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn quepy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QuepyConfig>()?;

    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
