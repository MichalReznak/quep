use std::collections::HashMap;

use pyo3::pyclass;
use typed_builder::TypedBuilder;

use super::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use crate::args::types::{CircuitBenchType, LangSchemaType};

#[pyclass]
#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsProvider {
    pub t: ProviderType,

    /// Path to Custom provider
    pub path: String,

    /// Python script files location
    pub python_dir: String,

    /// IBMQ: Account ID
    pub account_id: String,

    /// IBMQ: Machine name
    pub machine_name: String,
}

#[pyclass]
#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsOutput {
    pub t: OutputType,

    /// Path to Custom Outputer
    pub path: String,

    /// Serialize format
    pub ser: OutputSerType,

    /// Pretty print the output
    pub pretty: bool,
}

#[pyclass]
#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsCircuit {
    pub t: CircuitType,

    /// Path to Custom Circuit Generator
    pub path: String,

    /// Circuit benchmark type
    pub bench: CircuitBenchType,

    /// Initializes all qubits to |1>
    pub init_one: bool,

    /// Randomized qubits for two qubit gates
    pub shuffle: bool,

    /// Randomize circuit generation of the same size
    pub rand: bool,

    /// Base: Source file
    pub source: String,

    /// Base: Custom additional gates
    pub gates: HashMap<String, String>,
}

#[pyclass]
#[derive(Debug, Clone, TypedBuilder)]
pub struct CliArgsLangSchema {
    /// Used language schema
    pub t: LangSchemaType,

    /// Path to Custom Language Schema
    pub path: String,
}

#[pyclass]
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
