use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use pyo3::pyclass;

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    Ibmq,
    Simple,
    Noisy,
    Python,
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum OutputType {
    Text,
    Serial,
    Python,
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum CircuitType {
    Basic,
    Fs,
    Volume,
    Struct,
    Rand,
    Base,
    Python,
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum OrchestratorType {
    Lattice,
    Linear, // For FS
    Single, // Basic
    Volume, // QuantumVolume
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum OutputSerType {
    Json,
    Xml,
    Yaml,
    Toml,
    Ron,
}

#[derive(Debug, EnumString, Display, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
#[pyclass]
pub enum CircuitSchemaType {
    OpenQasm,
    Qiskit,
    Python,
}

#[derive(Debug, EnumString, Display, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum CircuitBenchType {
    Mirror,
    Cycle,
    None,
}
