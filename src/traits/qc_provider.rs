//! Quantum computer providers
//!
//! Set of providers that implement common interface to communicate
//! with a set or a single computer
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::qc_providers::IbmqQcProvider;
use crate::Error;

#[enum_dispatch]
pub enum QcProviderDyn {
    IbmqQcProvider,
}

#[async_trait]
#[enum_dispatch(QcProviderDyn)]
pub trait QcProvider {
    async fn connect(&self) -> Result<(), Error>;

    async fn run(&self, circuit: String) -> Result<String, Error>;
}
