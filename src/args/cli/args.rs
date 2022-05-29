use typed_builder::TypedBuilder;

use super::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsProvider {
    pub t: ProviderType,

    /// Python script files location
    pub python_dir: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsOutput {
    pub t: OutputType,

    /// Serialize format
    pub ser: OutputSerType,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsCircuit {
    pub t: CircuitType,

    /// Randomize circuit generation of the same size
    pub rand: bool,

    /// Base: Parse gates to primitive ones
    pub parse: bool,

    /// Base: Source file
    pub source: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsOrch {
    pub t: OrchestratorType,

    /// Fs: location of the .qasm files
    pub data: String,

    /// Number of iterations of the same size
    pub iter: i32,

    /// Depth of the max circuits
    pub size: i32,

    // TODO make it better
    // This is to define width and depth separately in some orchestrators
    /// Width of the circuits
    pub size_2: i32,

    /// Collect all circuits first and then run them
    pub collect: bool,
}