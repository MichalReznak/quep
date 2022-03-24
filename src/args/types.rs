use strum_macros::EnumString;

#[derive(EnumString)]
pub enum ProviderType {
    Ibmq,
    Qiskit,
}

#[derive(EnumString)]
pub enum OutputType {
    Text,
}

#[derive(EnumString)]
pub enum CircuitType {
    Basic,
    Fs,
}
