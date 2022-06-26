use std::collections::HashMap;

use typed_builder::TypedBuilder;

use super::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use crate::args::types::{CircuitBenchType, CircuitSchemaType};

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

    // TODO text
    pub bench: CircuitBenchType,

    /// Openqasm and other TODO
    pub schema: CircuitSchemaType,

    /// Increase by one and expect inverse result (Struct, Rand)
    pub interleave: bool,

    // TODO text
    pub init_one: bool,

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

    /// Start depth of the max circuits
    pub from_size: i32,

    /// Start width of the circuits
    pub from_size_2: i32,

    /// Depth of the max circuits
    pub size: i32,

    /// Width of the circuits
    pub size_2: i32,

    /// Collect all circuits first and then run them
    pub collect: bool,

    /// Do a dummy run
    pub preheat: bool,
}
