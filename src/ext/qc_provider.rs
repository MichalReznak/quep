//! Quantum computer qc_providers
//!
//! Set of qc_providers that implement common interface to communicate
//! with a set or a single computer
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use super::types::MetaInfo;
use crate::ext::types::circuit_generator::GenCircuit;
use crate::qc_providers::{IbmqQcProvider, NoisyQcProvider, SimpleQcProvider};
use crate::{CliArgs, Error};

#[enum_dispatch]
pub enum QcProviderDyn {
    IbmqQcProvider,
    SimpleQcProvider,
    NoisyQcProvider,
}

#[async_trait]
#[enum_dispatch(QcProviderDyn)]
pub trait QcProvider {
    /// Check whether arguments are correct
    fn check_constraints(&self, _args: &CliArgs) -> Result<(), Error> {
        Ok(())
    }

    /// Authorize provider
    async fn connect(&mut self) -> Result<(), Error>;

    /// Add circuit to the queue
    async fn append_circuit(&mut self, circuit: GenCircuit) -> Result<(), Error>;

    /// Run all circuits from queue and remove them
    async fn run(&self) -> Result<Vec<String>, Error>;

    /// Get all meta information about the last run
    async fn meta_info(&self) -> Result<MetaInfo, Error>;
}
