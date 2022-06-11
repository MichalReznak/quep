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
use typed_builder::TypedBuilder;
use types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};

use crate::args::types::CircuitSchemaType;
use crate::config::{CircuitConfig, OrchConfig, OutputConfig, ProviderConfig};
use crate::{dir, Error};

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgs {
    pub provider: CliArgsProvider,
    pub output: CliArgsOutput,
    pub circuit: CliArgsCircuit,
    pub orch: CliArgsOrch,
}

#[throws]
pub fn parse_provider(clap: &CliArgsEnv, config: ProviderConfig) -> CliArgsProvider {
    let python_dir = dir("./python")?;
    let provider_type = clap.provider.or(config.t).unwrap_or(ProviderType::Simple);

    CliArgsProvider::builder()
        .t(provider_type)
        .python_dir(clap.provider_python_dir.clone().or(config.python_dir).unwrap_or(python_dir))
        .account_id(clap.provider_account_id.clone().or(config.account_id).unwrap_or_else(|| {
            if provider_type == ProviderType::Ibmq {
                panic!("IBM Q needs account ID");
            }

            "".to_string()
        }))
        .build()
}

#[throws]
pub fn parse_output(clap: &CliArgsEnv, config: OutputConfig) -> CliArgsOutput {
    CliArgsOutput::builder()
        .t(clap.output.or(config.t).unwrap_or(OutputType::Text))
        .ser(clap.output_ser.or(config.ser).unwrap_or(OutputSerType::Json))
        .pretty(clap.output_pretty.or(config.pretty).unwrap_or(true))
        .build()
}

#[throws]
pub fn parse_circuit(clap: &CliArgsEnv, config: CircuitConfig) -> CliArgsCircuit {
    let circuit_source = dir("./base.template.qasm")?;

    CliArgsCircuit::builder()
        .t(clap.circuit.or(config.t).unwrap_or(CircuitType::Basic))
        .schema(clap.circuit_schema.or(config.schema).unwrap_or(CircuitSchemaType::OpenQasm))
        .rand(clap.circuit_rand.or(config.rand).unwrap_or(false))
        .parse(clap.circuit_parse.or(config.parse).unwrap_or(false))
        .source(clap.circuit_source.clone().or(config.source).unwrap_or(circuit_source))
        .inverse_gates(config.inverse_gates.unwrap_or(collection! {
            HashMap<String, String>;
            "s".to_string() => "sdg".to_string(),
            "t".to_string() => "tdg".to_string(),
        }))
        .build()
}

#[throws]
pub fn parse_orch(clap: &CliArgsEnv, config: OrchConfig) -> CliArgsOrch {
    let orch_data_dir = dir("./data")?;

    CliArgsOrch::builder()
        .t(clap.orch.or(config.t).unwrap_or(OrchestratorType::Single))
        .data(clap.orch_data.clone().or(config.data).unwrap_or(orch_data_dir))
        .iter(clap.orch_iter.or(config.iter).unwrap_or(1))
        .size(clap.orch_size.or(config.size).unwrap_or(1))
        .size_2(clap.orch_size_2.or(config.size_2).unwrap_or(1))
        .collect(clap.orch_collect.or(config.collect).unwrap_or(false))
        .preheat(clap.orch_preheat.or(config.preheat).unwrap_or(true))
        .build()
}

impl CliArgs {
    #[throws]
    pub fn parse() -> CliArgs {
        dotenv::dotenv().ok();
        let clap = CliArgsEnv::parse();
        let config = CliArgsConfig::default();

        CliArgs::builder()
            .provider(parse_provider(&clap, config.provider)?)
            .output(parse_output(&clap, config.output)?)
            .circuit(parse_circuit(&clap, config.circuit)?)
            .orch(parse_orch(&clap, config.orch)?)
            .build()
    }

    #[throws]
    pub fn parse_with_config(config_path: &str) -> CliArgs {
        dotenv::dotenv().ok();
        let clap = CliArgsEnv::parse();

        // parse config file, json for now
        let config = std::fs::read_to_string(&config_path)?;
        let config = json5::from_str::<CliArgsConfig>(&config)?;

        CliArgs::builder()
            .provider(parse_provider(&clap, config.provider)?)
            .output(parse_output(&clap, config.output)?)
            .circuit(parse_circuit(&clap, config.circuit)?)
            .orch(parse_orch(&clap, config.orch)?)
            .build()
    }
}
