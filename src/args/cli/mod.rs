pub mod args;
pub mod config;
pub mod env;
pub mod types;

pub use args::*;
use clap::Parser;
pub use config::CliArgsConfig;
pub use env::CliArgsEnv;
use fehler::throws;
use typed_builder::TypedBuilder;
use types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};

use crate::{dir, Error};

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgs {
    pub provider: CliArgsProvider,
    pub output: CliArgsOutput,
    pub circuit: CliArgsCircuit,
    pub orch: CliArgsOrch,
}

impl CliArgs {
    #[throws]
    pub fn parse_with_config(config_path: &str) -> CliArgs {
        dotenv::dotenv().ok();
        let clap = CliArgsEnv::parse();

        // TODO define correct combinations
        // parse config file, json for now
        let config = std::fs::read_to_string(&config_path)?;
        let config = json5::from_str::<CliArgsConfig>(&config)?;

        let orch_data_dir = dir("./data")?;
        let python_dir = dir("./python")?;
        let circuit_source = dir("./base.template.qasm")?;

        let provider = CliArgsProvider::builder()
            .t(clap.provider.or(config.provider.t).unwrap_or(ProviderType::Simple))
            .python_dir(
                clap.provider_python_dir.or(config.provider.python_dir).unwrap_or(python_dir),
            )
            .account_id(
                clap.provider_account_id
                    .or(config.provider.account_id)
                    .unwrap_or("".to_string()),
            )
            .build();

        let output = CliArgsOutput::builder()
            .t(clap.output.or(config.output.t).unwrap_or(OutputType::Text))
            .ser(clap.output_ser.or(config.output.ser).unwrap_or(OutputSerType::Json))
            .pretty(clap.output_pretty.or(config.output.pretty).unwrap_or(true))
            .build();

        let circuit = CliArgsCircuit::builder()
            .t(clap.circuit.or(config.circuit.t).unwrap_or(CircuitType::Basic))
            .rand(clap.circuit_rand.or(config.circuit.rand).unwrap_or(false))
            .parse(clap.circuit_parse.or(config.circuit.parse).unwrap_or(false))
            .source(clap.circuit_source.or(config.circuit.source).unwrap_or(circuit_source))
            .build();

        let orch = CliArgsOrch::builder()
            .t(clap.orch.or(config.orch.t).unwrap_or(OrchestratorType::Single))
            .data(clap.orch_data.or(config.orch.data).unwrap_or(orch_data_dir))
            .iter(clap.orch_iter.or(config.orch.iter).unwrap_or(1))
            .size(clap.orch_size.or(config.orch.size).unwrap_or(1))
            .size_2(clap.orch_size_2.or(config.orch.size_2).unwrap_or(1))
            .collect(clap.orch_collect.or(config.orch.collect).unwrap_or(false))
            .build();

        // TODO better?
        CliArgs::builder()
            .provider(provider)
            .output(output)
            .circuit(circuit)
            .orch(orch)
            .build()
    }
}
