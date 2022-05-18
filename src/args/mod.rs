pub use config::CliArgsConfig;
pub use env::CliArgsEnv;
use typed_builder::TypedBuilder;

use crate::args::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};

pub mod config;
pub mod env;
pub mod types;

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgs {
    pub provider: ProviderType,
    pub output: OutputType,
    pub output_ser: OutputSerType,
    pub circuit: CircuitType,
    pub orch: OrchestratorType,
    pub orch_data: String,
    pub orch_iter: i32,
    pub orch_size: i32,

    // TODO make it better
    // This is to define width and depth separately in some orchestrators
    pub orch_size_2: i32,
    pub python_dir: String,
}
