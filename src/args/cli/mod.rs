pub mod args;
pub mod config;
pub mod env;
pub mod types;

use std::collections::HashMap;

pub use args::*;
use clap::Parser;
use collection_literals::collection;
pub use config::CliArgsConfig;
pub use env::CliArgsEnv;
use fehler::throws;
use pyo3::pyclass;
use typed_builder::TypedBuilder;
use types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};

use crate::args::types::{CircuitBenchType, LangSchemaType};
use crate::config::{
    CliArgsCircuitConfig, CliArgsLangSchemaConfig, CliArgsOrchConfig, CliArgsOutputConfig,
    CliArgsProviderConfig,
};
use crate::error::Constraint;
use crate::{dir, Error};

#[pyclass]
#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgs {
    pub provider: CliArgsProvider,
    pub output: CliArgsOutput,
    pub circuit: CliArgsCircuit,
    pub lang_schema: CliArgsLangSchema,
    pub orch: CliArgsOrch,
}

#[throws]
pub fn parse_provider(clap: &CliArgsEnv, config: CliArgsProviderConfig) -> CliArgsProvider {
    let path = || dir("./qc_provider.py").unwrap_or_else(|_| "".to_string());
    let python_dir = || dir(".").unwrap_or_else(|_| "".to_string());

    CliArgsProvider::builder()
        .t(clap.provider.or(config.t).unwrap_or(ProviderType::Simple))
        .path(clap.provider_path.clone().or(config.path).unwrap_or_else(path))
        .python_dir(
            clap.provider_python_dir
                .clone()
                .or(config.python_dir)
                .unwrap_or_else(python_dir),
        )
        .account_id(
            clap.provider_account_id
                .clone()
                .or(config.account_id)
                .unwrap_or_else(|| "".to_string()),
        )
        .build()
}

#[throws]
pub fn parse_output(clap: &CliArgsEnv, config: CliArgsOutputConfig) -> CliArgsOutput {
    let path = || dir("./outputer.py").unwrap_or_else(|_| "".to_string());

    CliArgsOutput::builder()
        .t(clap.output.or(config.t).unwrap_or(OutputType::Text))
        .path(clap.circuit_path.clone().or(config.path).unwrap_or_else(path))
        .ser(clap.output_ser.or(config.ser).unwrap_or(OutputSerType::Json))
        .pretty(clap.output_pretty.or(config.pretty).unwrap_or(true))
        .build()
}

#[throws]
pub fn parse_circuit(clap: &CliArgsEnv, config: CliArgsCircuitConfig) -> CliArgsCircuit {
    // TODO all dir calls should not fail when file does not exist
    let path = || dir("./circuit_generator.py").unwrap_or_else(|_| "".to_string());
    let source = || dir("./templates/example.qasm").unwrap_or_else(|_| "".to_string());

    CliArgsCircuit::builder()
        .t(clap.circuit.or(config.t).unwrap_or(CircuitType::Struct))
        .path(clap.circuit_path.clone().or(config.path).unwrap_or_else(path))
        .bench(clap.circuit_bench.or(config.bench).unwrap_or(CircuitBenchType::Mirror))
        .init_one(clap.circuit_init_one.or(config.init_one).unwrap_or(false))
        .rand(clap.circuit_rand.or(config.rand).unwrap_or(false))
        .parse(clap.circuit_parse.or(config.parse).unwrap_or(false))
        .source(clap.circuit_source.clone().or(config.source).unwrap_or_else(source))
        .inverse_gates(clap.circuit_inverse_gates.clone().or(config.inverse_gates).unwrap_or(
            collection! {
                HashMap<String, String>;
                "s".to_string() => "sdg".to_string(),
                "t".to_string() => "tdg".to_string(),
            },
        ))
        .build()
}

#[throws]
pub fn parse_lang_schema(clap: &CliArgsEnv, config: CliArgsLangSchemaConfig) -> CliArgsLangSchema {
    let path = || dir("./lang_schema.py").unwrap_or_else(|_| "".to_string());

    CliArgsLangSchema::builder()
        .t(clap.lang_schema.or(config.t).unwrap_or(LangSchemaType::OpenQasm))
        .path(clap.lang_schema_path.clone().or(config.path).unwrap_or_else(path))
        .build()
}

#[throws]
pub fn parse_orch(clap: &CliArgsEnv, config: CliArgsOrchConfig) -> CliArgsOrch {
    let data = || dir("./data").unwrap_or_else(|_| "".to_string());

    CliArgsOrch::builder()
        .t(clap.orch.or(config.t).unwrap_or(OrchestratorType::Single))
        .data(clap.orch_data.clone().or(config.data).unwrap_or_else(data))
        .iter(clap.orch_iter.or(config.iter).unwrap_or(1))
        .size(clap.orch_size.or(config.size).unwrap_or(1))
        .from_size(clap.orch_from_size.or(config.from_size).unwrap_or(1))
        .size_2(clap.orch_size_2.or(config.size_2).unwrap_or(1))
        .from_size_2(clap.orch_from_size_2.or(config.from_size_2).unwrap_or(1))
        .collect(clap.orch_collect.or(config.collect).unwrap_or(false))
        .preheat(clap.orch_preheat.or(config.preheat).unwrap_or(true))
        .build()
}

#[throws]
fn check_constraints(args: &CliArgs) {
    if args.orch.size <= 0
        || args.orch.size_2 <= 0
        || args.orch.from_size <= 0
        || args.orch.from_size_2 <= 0
    {
        Constraint {
            reason: "size/size2/fromSize/fromSize2 needs to be positive number".to_string(),
        }
        .fail()?;
    }
}

impl CliArgs {
    #[throws]
    pub fn parse() -> CliArgs {
        dotenv::dotenv().ok();
        let clap = CliArgsEnv::parse();
        let config = CliArgsConfig::default();

        let res = CliArgs::builder()
            .provider(parse_provider(&clap, config.provider)?)
            .output(parse_output(&clap, config.output)?)
            .circuit(parse_circuit(&clap, config.circuit)?)
            .lang_schema(parse_lang_schema(&clap, config.lang_schema)?)
            .orch(parse_orch(&clap, config.orch)?)
            .build();
        check_constraints(&res)?;
        res
    }

    #[throws]
    pub fn parse_with_config_no_env(config: CliArgsConfig) -> CliArgs {
        let clap = CliArgsEnv::default();

        let res = CliArgs::builder()
            .provider(parse_provider(&clap, config.provider)?)
            .output(parse_output(&clap, config.output)?)
            .circuit(parse_circuit(&clap, config.circuit)?)
            .lang_schema(parse_lang_schema(&clap, config.lang_schema)?)
            .orch(parse_orch(&clap, config.orch)?)
            .build();
        check_constraints(&res)?;
        res
    }

    #[throws]
    pub fn parse_with_config(config_path: &str) -> CliArgs {
        dotenv::dotenv().ok();
        let clap = CliArgsEnv::parse();

        // parse config file, json for now
        let config = std::fs::read_to_string(&config_path)?;
        let config = json5::from_str::<CliArgsConfig>(&config)?;

        let res = CliArgs::builder()
            .provider(parse_provider(&clap, config.provider)?)
            .output(parse_output(&clap, config.output)?)
            .circuit(parse_circuit(&clap, config.circuit)?)
            .lang_schema(parse_lang_schema(&clap, config.lang_schema)?)
            .orch(parse_orch(&clap, config.orch)?)
            .build();

        check_constraints(&res)?;
        res
    }
}
