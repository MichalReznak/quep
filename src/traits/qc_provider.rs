//! Quantum computer qc_providers
//!
//! Set of qc_providers that implement common interface to communicate
//! with a set or a single computer
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use tokio::time::Duration;

use crate::qc_providers::IbmqQcProvider;
#[cfg(feature = "qiskit")]
use crate::qc_providers::{NoisyQcProvider, QiskitQcProvider};
use crate::Error;

#[cfg(feature = "qiskit")]
#[enum_dispatch]
pub enum QcProviderDyn {
    IbmqQcProvider,
    QiskitQcProvider,
    NoisyQcProvider,
}

#[cfg(not(feature = "qiskit"))]
#[enum_dispatch]
pub enum QcProviderDyn {
    IbmqQcProvider,
}

#[async_trait]
#[enum_dispatch(QcProviderDyn)]
pub trait QcProvider {
    async fn connect(&self) -> Result<(), Error>;

    async fn run(&self, circuit: String) -> Result<String, Error>;

    fn start_measure(&mut self);

    fn stop_measure(&mut self) -> Duration;
}
