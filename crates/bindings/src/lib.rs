use std::str::FromStr;

use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use quep_core::args::config::{
    CliArgsCircuitConfig, CliArgsLangSchemaConfig, CliArgsOrchConfig, CliArgsOutputConfig,
    CliArgsProviderConfig,
};
use quep_core::args::types::{
    CircuitBenchType, CircuitType, LangSchemaType, OrchestratorType, OutputSerType, OutputType,
    ProviderType,
};
use quep_core::args::CliArgsConfig;
use quep_core::CliArgs;

#[pyclass]
#[derive(Clone, Debug)]
struct QuepyConfig {
    pub provider: Option<ProviderType>,
    pub provider_path: Option<String>,
    pub provider_python_dir: Option<String>,
    pub provider_account_id: Option<String>,
    pub provider_machine_name: Option<String>,

    pub output: Option<OutputType>,
    pub output_path: Option<String>,
    pub output_ser: Option<OutputSerType>,
    pub output_pretty: Option<bool>,

    pub circuit: Option<CircuitType>,
    pub circuit_path: Option<String>,
    pub circuit_bench: Option<CircuitBenchType>,
    pub circuit_init_one: Option<bool>,
    pub circuit_rand: Option<bool>,
    pub circuit_source: Option<String>,

    pub lang_schema: Option<LangSchemaType>,
    pub lang_schema_path: Option<String>,

    pub orch: Option<OrchestratorType>,
    pub orch_data: Option<String>,
    pub orch_iter: Option<i32>,
    pub orch_from_size: Option<i32>,
    pub orch_from_size_2: Option<i32>,
    pub orch_size: Option<i32>,
    pub orch_size_2: Option<i32>,
    pub orch_collect: Option<bool>,
    pub orch_preheat: Option<bool>,
}

#[pymethods]
impl QuepyConfig {
    #[new]
    fn new(
        provider: Option<String>,
        provider_path: Option<String>,
        provider_python_dir: Option<String>,
        provider_account_id: Option<String>,
        provider_machine_name: Option<String>,

        output: Option<String>,
        output_path: Option<String>,
        output_ser: Option<String>,
        output_pretty: Option<bool>,

        circuit: Option<String>,
        circuit_path: Option<String>,
        circuit_bench: Option<String>,
        circuit_init_one: Option<bool>,
        circuit_rand: Option<bool>,
        circuit_source: Option<String>,

        lang_schema: Option<String>,
        lang_schema_path: Option<String>,

        orch: Option<String>,
        orch_data: Option<String>,
        orch_iter: Option<i32>,
        orch_from_size: Option<i32>,
        orch_from_size_2: Option<i32>,
        orch_size: Option<i32>,
        orch_size_2: Option<i32>,
        orch_collect: Option<bool>,
        orch_preheat: Option<bool>,
    ) -> PyResult<Self> {
        Ok(Self {
            provider: provider.map(|e| ProviderType::from_str(&e).unwrap()),
            provider_path,
            provider_python_dir,
            provider_account_id,
            provider_machine_name,

            output: output.map(|e| OutputType::from_str(&e).unwrap()),
            output_path,
            output_ser: output_ser.map(|e| OutputSerType::from_str(&e).unwrap()),
            output_pretty,

            circuit: circuit.map(|e| CircuitType::from_str(&e).unwrap()),
            circuit_path,
            circuit_bench: circuit_bench.map(|e| CircuitBenchType::from_str(&e).unwrap()),
            circuit_init_one,
            circuit_rand,
            circuit_source,

            lang_schema: lang_schema.map(|e| LangSchemaType::from_str(&e).unwrap()),
            lang_schema_path,

            orch: orch.map(|e| OrchestratorType::from_str(&e).unwrap()),
            orch_data,
            orch_iter,
            orch_size,
            orch_size_2,
            orch_collect,
            orch_preheat,
            orch_from_size,
            orch_from_size_2,
        })
    }
}

impl From<QuepyConfig> for CliArgsConfig {
    fn from(qc: QuepyConfig) -> Self {
        Self {
            provider: CliArgsProviderConfig {
                t: qc.provider,
                path: qc.provider_path,
                python_dir: qc.provider_python_dir,
                account_id: qc.provider_account_id,
                machine_name: qc.provider_machine_name,
            },
            circuit: CliArgsCircuitConfig {
                t: qc.circuit,
                path: qc.circuit_path,
                bench: qc.circuit_bench,
                init_one: qc.circuit_init_one,
                rand: qc.circuit_rand,
                source: qc.circuit_source,
                gates: None,
            },
            lang_schema: CliArgsLangSchemaConfig {
                t: qc.lang_schema,
                path: qc.lang_schema_path,
            },
            orch: CliArgsOrchConfig {
                t: qc.orch,
                from_size: qc.orch_from_size,
                from_size_2: qc.orch_from_size_2,
                size: qc.orch_size,
                size_2: qc.orch_size_2,
                iter: qc.orch_iter,
                collect: qc.orch_collect,
                preheat: qc.orch_preheat,
                data: qc.orch_data,
            },
            output: CliArgsOutputConfig {
                t: qc.output,
                path: qc.output_path,
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
        Ok(Python::with_gil(|_| res))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn quepy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QuepyConfig>()?;

    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
