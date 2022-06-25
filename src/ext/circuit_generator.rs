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

use super::types::circuit_generator::GenCircuit;
use crate::circuit_generators::{
    BaseCircuitGenerator, BasicCircuitGenerator, FsCircuitGenerator, RandCircuitGenerator,
    StructCircuitGenerator, VolumeCircuitGenerator,
};
use crate::{CliArgs, Error};

#[enum_dispatch]
pub enum CircuitGeneratorDyn {
    BasicCircuitGenerator,
    FsCircuitGenerator,
    VolumeCircuitGenerator,
    StructCircuitGenerator,
    RandCircuitGenerator,
    BaseCircuitGenerator,
}

#[async_trait]
#[enum_dispatch(CircuitGeneratorDyn)]
pub trait CircuitGenerator {
    /// Check whether arguments are correct
    fn check_constraints(&self, _args: &CliArgs) -> Result<(), Error> {
        Ok(())
    }

    async fn generate(&mut self, i: i32, j: i32, rand: i32) -> Result<Option<GenCircuit>, Error>;
}
