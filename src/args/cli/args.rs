use std::collections::HashMap;

use typed_builder::TypedBuilder;

use super::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use crate::args::types::CircuitSchemaType;

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsProvider {
    pub t: ProviderType,

    /// Python script files location
    pub python_dir: String,

    /// IBMQ: Account ID
    pub account_id: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsOutput {
    pub t: OutputType,

    /// Serialize format
    pub ser: OutputSerType,

    /// Pretty print the output
    pub pretty: bool,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsCircuit {
    pub t: CircuitType,

    /// Openqasm and other TODO
    pub schema: CircuitSchemaType,

    /// Randomize circuit generation of the same size
    pub rand: bool,

    /// Base: Parse gates to primitive ones
    pub parse: bool,

    /// Base: Source file
    pub source: String,

    /// Base: Inverse gates
    pub inverse_gates: HashMap<String, String>,
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

    /// Do a dummy run
    pub preheat: bool,

    /// Mirror the circuits
    pub mirror: bool,
}
