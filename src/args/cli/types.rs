use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    Ibmq,
    Simple,
    Noisy,
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum OutputType {
    Text,
    Serial,
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum CircuitType {
    Basic,
    Fs,
    Volume,
    Struct,
    Rand,
    Base,
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum OrchestratorType {
    Lattice,
    Linear, // For FS
    Single, // Basic
    Volume, // QuantumVolume
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum OutputSerType {
    Json,
    Xml,
    Yaml,
    Toml,
    Ron,
}

#[derive(Debug, EnumString, Display, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum CircuitSchemaType {
    OpenQasm,
    Qiskit,
}

#[derive(Debug, EnumString, Display, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum CircuitBenchType {
    Mirror,
    Cycle,
}
