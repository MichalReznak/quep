//! Quantum computer qc_providers
//!
//! Set of qc_providers that implement common interface to communicate
//! with a set or a single computer
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use tokio::time::Duration;

use crate::qc_providers::{IbmqQcProvider, NoisyQcProvider, SimpleQcProvider};
use crate::Error;

#[enum_dispatch]
pub enum QcProviderDyn {
    IbmqQcProvider,
    SimpleQcProvider,
    NoisyQcProvider,
}

#[async_trait]
#[enum_dispatch(QcProviderDyn)]
pub trait QcProvider {
    async fn connect(&mut self) -> Result<(), Error>;

    async fn set_circuit(&mut self, circuit: String) -> Result<(), Error>;
    async fn append_circuit(&mut self, circuit: String) -> Result<(), Error>;

    async fn run(&self) -> Result<String, Error>;
    async fn run_all(&self) -> Result<Vec<String>, Error>;

    fn start_measure(&mut self);

    fn stop_measure(&mut self) -> Duration;
}
