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

use crate::circuit_generators::{
    BasicCircuitGenerator, FsCircuitGenerator, MirrorCircuitGenerator, RandMirrorCircuitGenerator,
    VolumeCircuitGenerator,
};
use crate::Error;

#[enum_dispatch]
pub enum CircuitGeneratorDyn {
    BasicCircuitGenerator,
    FsCircuitGenerator,
    VolumeCircuitGenerator,
    MirrorCircuitGenerator,
    RandMirrorCircuitGenerator,
}

#[async_trait]
#[enum_dispatch(CircuitGeneratorDyn)]
pub trait CircuitGenerator {
    async fn generate(&mut self, i: i32, j: i32, rand: i32) -> Result<Option<String>, Error>;
}
