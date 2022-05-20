use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize)]
pub enum ProviderType {
    Ibmq,
    Simple,
    Noisy,
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize)]
pub enum OutputType {
    Text,
    Serial,
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize)]
pub enum CircuitType {
    Basic,
    Fs,
    Volume,
    Mirror,
    RandMirror,
    Base,
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize)]
pub enum OrchestratorType {
    Lattice,
    Linear, // For FS
    Single, // Basic
    Volume, // QuantumVolume
}

#[derive(Debug, EnumString, Clone, Copy, Serialize, Deserialize)]
pub enum OutputSerType {
    Json,
    Xml,
    Yaml,
    Toml,
    Ron,
}
