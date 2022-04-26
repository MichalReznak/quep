use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum ProviderType {
    Ibmq,
    Qiskit,
    Noisy,
}

#[derive(Debug, EnumString)]
pub enum OutputType {
    Text,
    Serial,
}

#[derive(Debug, EnumString)]
pub enum CircuitType {
    Basic,
    Fs,
    Volume,
    Mirror,
    RandMirror,
}

#[derive(Debug, EnumString)]
pub enum OrchestratorType {
    Lattice,
    Linear, // For FS
    Single, // Basic
    Volume, // QuantumVolume
}

#[derive(Debug, EnumString)]
pub enum OutputSerType {
    Json,
    Xml,
    Yaml,
    Toml,
    Ron,
}
