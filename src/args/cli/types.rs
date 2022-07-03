use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[pyclass]
#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    Ibmq,
    Simple,
    Noisy,
    Python,
}

#[pyclass]
#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum OutputType {
    Text,
    Serial,
    Python,
}

#[pyclass]
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

#[pyclass]
#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum OrchestratorType {
    Lattice,
    Linear, // For FS
    Single, // Basic
    Volume, // QuantumVolume
}

#[pyclass]
#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum OutputSerType {
    Json,
    Xml,
    Yaml,
    Toml,
    Ron,
}

#[pyclass]
#[derive(Debug, EnumString, Display, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum LangSchemaType {
    OpenQasm,
    Qiskit,
    Python,
}

#[pyclass]
#[derive(Debug, EnumString, Display, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum CircuitBenchType {
    Mirror,
    Cycle,
    None,
}
