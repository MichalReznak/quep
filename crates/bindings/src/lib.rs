use std::str::FromStr;

use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use quep_core::args::config::{
    CliArgsCircuitConfig, CliArgsOrchConfig, CliArgsOutputConfig, CliArgsProviderConfig,
};
use quep_core::args::types::{
    CircuitSchemaType, CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType,
};
use quep_core::args::CliArgsConfig;
use quep_core::CliArgs;

#[pyclass]
#[derive(Clone, Debug)]
struct QuepyConfig {
    pub provider: Option<ProviderType>,
    pub provider_python_dir: Option<String>,
    pub provider_account_id: Option<String>,
    pub output: Option<OutputType>,
    pub output_ser: Option<OutputSerType>,
    pub output_pretty: Option<bool>,
    pub circuit: Option<CircuitType>,
    pub circuit_schema: Option<CircuitSchemaType>,
    pub circuit_rand: Option<bool>,
    pub circuit_parse: Option<bool>,
    pub circuit_source: Option<String>,
    pub orch: Option<OrchestratorType>,
    pub orch_data: Option<String>,
    pub orch_iter: Option<i32>,
    pub orch_size: Option<i32>,
    pub orch_size_2: Option<i32>,
    pub orch_collect: Option<bool>,
    pub orch_preheat: Option<bool>,
    pub orch_mirror: Option<bool>,
}

#[pymethods]
impl QuepyConfig {
    // TODO
    #[new]
    fn new(
        provider: Option<String>,
        provider_python_dir: Option<String>,
        provider_account_id: Option<String>,

        output: Option<String>,
        output_ser: Option<String>,
        output_pretty: Option<bool>,

        circuit: Option<String>,
        circuit_schema: Option<String>,
        circuit_rand: Option<bool>,
        circuit_parse: Option<bool>,
        circuit_source: Option<String>,

        orch: Option<String>,
        orch_data: Option<String>,
        orch_iter: Option<i32>,
        orch_size: Option<i32>,
        orch_size_2: Option<i32>,
        orch_collect: Option<bool>,
        orch_preheat: Option<bool>,
        orch_mirror: Option<bool>,
    ) -> PyResult<Self> {
        Ok(Self {
            provider: provider.map(|e| ProviderType::from_str(&e).unwrap()),
            provider_python_dir,
            provider_account_id,
            output: output.map(|e| OutputType::from_str(&e).unwrap()),
            output_ser: output_ser.map(|e| OutputSerType::from_str(&e).unwrap()),
            output_pretty,
            circuit: circuit.map(|e| CircuitType::from_str(&e).unwrap()),
            circuit_schema: circuit_schema.map(|e| CircuitSchemaType::from_str(&e).unwrap()),
            circuit_rand,
            circuit_parse,
            circuit_source,
            orch: orch.map(|e| OrchestratorType::from_str(&e).unwrap()),
            orch_data,
            orch_iter,
            orch_size,
            orch_size_2,
            orch_collect,
            orch_preheat,
            orch_mirror,
        })
    }
}

impl From<QuepyConfig> for CliArgsConfig {
    fn from(qc: QuepyConfig) -> Self {
        Self {
            provider: CliArgsProviderConfig {
                t: qc.provider,
                python_dir: qc.provider_python_dir,
                account_id: qc.provider_account_id,
            },
            circuit: CliArgsCircuitConfig {
                t: qc.circuit,
                schema: qc.circuit_schema,
                rand: qc.circuit_rand,
                parse: qc.circuit_parse,
                source: qc.circuit_source,
                inverse_gates: None,
            },
            orch: CliArgsOrchConfig {
                t: qc.orch,
                size: qc.orch_size,
                size_2: qc.orch_size_2,
                iter: qc.orch_iter,
                collect: qc.orch_collect,
                preheat: qc.orch_preheat,
                mirror: qc.orch_mirror,
                data: qc.orch_data,
            },
            output: CliArgsOutputConfig {
                t: qc.output,
                ser: qc.output_ser,
                pretty: qc.output_pretty,
            },
        }
    }
}

/// Run new Quep instance and use configuration from arg
#[pyfunction]
fn run(py: Python<'_>, config: QuepyConfig) -> PyResult<&PyAny> {
    future_into_py(py, async move {
        let args = CliArgs::parse_with_config_no_env(config.into()).unwrap();
        let res = quep_core::Quep::new(args).await.unwrap().run().await.unwrap();
        Ok(Python::with_gil(|_py| res))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn quepy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QuepyConfig>()?;

    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
