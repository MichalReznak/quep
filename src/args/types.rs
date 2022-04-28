use strum_macros::EnumString;

#[derive(Debug, EnumString, Clone, Copy)]
pub enum ProviderType {
    Ibmq,
    Qiskit,
    Noisy,
}

#[derive(Debug, EnumString, Clone, Copy)]
pub enum OutputType {
    Text,
    Serial,
}

#[derive(Debug, EnumString, Clone, Copy)]
pub enum CircuitType {
    Basic,
    Fs,
    Volume,
    Mirror,
    RandMirror,
}

#[derive(Debug, EnumString, Clone, Copy)]
pub enum OrchestratorType {
    Lattice,
    Linear, // For FS
    Single, // Basic
    Volume, // QuantumVolume
}

#[derive(Debug, EnumString, Clone, Copy)]
pub enum OutputSerType {
    Json,
    Xml,
    Yaml,
    Toml,
    Ron,
}
