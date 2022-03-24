//! Circuit generators
//!
//! Generate set of circuits based on some rules
//! Can define some exceptions?
//! Could be:
//! * Structured mirror circuits
//! * Random mirror circuits
//! * Cycle benchmarking
//! * Etc

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::circuit_generators::BasicCircuitGenerator;
use crate::Error;

#[enum_dispatch]
pub enum CircuitGeneratorDyn {
    BasicCircuitGenerator,
}

#[async_trait]
#[enum_dispatch(CircuitGeneratorDyn)]
pub trait CircuitGenerator {
    // TODO

    async fn generate(&self) -> Result<String, Error>;
}
