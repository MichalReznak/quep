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
            .t(clap.provider.unwrap_or_else(|| ProviderType::Simple))
            .python_dir(
                clap.provider_python_dir
                    .or_else(|| config.provider.python_dir)
                    .unwrap_or_else(|| python_dir),
            )
            .build();

        let output = CliArgsOutput::builder()
            .t(clap.output.or_else(|| config.output.t).unwrap_or_else(|| OutputType::Text))
            .ser(
                clap.output_ser
                    .or_else(|| config.output.ser)
                    .unwrap_or_else(|| OutputSerType::Json),
            )
            .build();

        let circuit = CliArgsCircuit::builder()
            .t(clap.circuit.or_else(|| config.circuit.t).unwrap_or_else(|| CircuitType::Basic))
            .rand(clap.circuit_rand.or_else(|| config.circuit.rand).unwrap_or_else(|| false))
            .parse(clap.circuit_parse.or_else(|| config.circuit.parse).unwrap_or_else(|| false))
            .source(
                clap.circuit_source
                    .or_else(|| config.circuit.source)
                    .unwrap_or_else(|| circuit_source),
            )
            .build();

        let orch = CliArgsOrch::builder()
            .t(clap.orch.or_else(|| config.orch.t).unwrap_or_else(|| OrchestratorType::Single))
            .data(clap.orch_data.or_else(|| config.orch.data).unwrap_or_else(|| orch_data_dir))
            .iter(clap.orch_iter.or_else(|| config.orch.iter).unwrap_or_else(|| 1))
            .size(clap.orch_size.or_else(|| config.orch.size).unwrap_or_else(|| 1))
            .size_2(clap.orch_size_2.or_else(|| config.orch.size_2).unwrap_or_else(|| 1))
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
