pub mod config;
pub mod env;
pub mod types;

use clap::Parser;
pub use config::CliArgsConfig;
pub use env::CliArgsEnv;
use fehler::throws;
use load_file::load_str;
use typed_builder::TypedBuilder;
use types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};

use crate::{dir, Error};

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgs {
    pub provider: ProviderType,

    pub output: OutputType,
    /// Serialize format
    pub output_ser: OutputSerType,

    pub circuit: CircuitType,
    /// Randomize circuit generation of the same size
    pub circuit_rand: bool,
    /// Base: Parse gates to primitive ones
    pub circuit_parse: bool,

    pub orch: OrchestratorType,
    /// Fs: location of the .qasm files
    pub orch_data: String,
    /// Number of iterations of the same size
    pub orch_iter: i32,
    /// Depth of the max circuits
    pub orch_size: i32,

    // TODO make it better
    // This is to define width and depth separately in some orchestrators
    /// Width of the circuits
    pub orch_size_2: i32,

    /// Python script files location
    pub python_dir: String,
}

impl CliArgs {
    #[throws]
    pub fn parse_with_config(config_path: &str) -> CliArgs {
        dotenv::dotenv().ok();
        let clap = CliArgsEnv::parse();

        // TODO define correct combinations
        // parse config file, json for now
        let config = load_str!(&config_path); // TODO panics on error, relative dir
        let config = json5::from_str::<CliArgsConfig>(config)?;

        let orch_data_dir = dir("./data")?;
        let python_dir = dir("./python")?;

        // TODO better?
        // if not set use it
        CliArgs::builder()
            .provider(clap.provider.unwrap_or_else(|| ProviderType::Simple))
            .output(clap.output.or_else(|| config.output.t).unwrap_or_else(|| OutputType::Text))
            .output_ser(
                clap.output_ser
                    .or_else(|| config.output.ser)
                    .unwrap_or_else(|| OutputSerType::Json),
            )
            .circuit(
                clap.circuit.or_else(|| config.circuit.t).unwrap_or_else(|| CircuitType::Basic),
            )
            .circuit_rand(
                clap.circuit_rand.or_else(|| config.circuit.rand).unwrap_or_else(|| false),
            )
            .circuit_parse(
                clap.circuit_parse.or_else(|| config.circuit.parse).unwrap_or_else(|| false),
            )
            .orch(clap.orch.or_else(|| config.orch.t).unwrap_or_else(|| OrchestratorType::Single))
            .orch_data(clap.orch_data.or_else(|| config.orch.data).unwrap_or_else(|| orch_data_dir))
            .orch_iter(clap.orch_iter.or_else(|| config.orch.iter).unwrap_or_else(|| 1))
            .orch_size(clap.orch_size.or_else(|| config.orch.size).unwrap_or_else(|| 1))
            .orch_size_2(clap.orch_size_2.or_else(|| config.orch.size_2).unwrap_or_else(|| 1))
            .python_dir(clap.python_dir.or_else(|| config.python_dir).unwrap_or_else(|| python_dir))
            .build()
    }
}
