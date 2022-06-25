//! Orchestrators
//!
//! This module synchronizes all other parts of the application
//! to do whatever needed

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::orchestrators::{
    LatticeOrchestrator, LinearOrchestrator, SingleOrchestrator, VolumeOrchestrator,
};
use crate::{Chooser, CliArgs, Error};

#[enum_dispatch]
pub enum OrchestratorDyn {
    LatticeOrchestrator,
    LinearOrchestrator,
    SingleOrchestrator,
    VolumeOrchestrator,
}

#[async_trait]
#[enum_dispatch(OrchestratorDyn)]
pub trait Orchestrator {
    /// Check whether arguments are correct
    fn check_constraints(&self, _args: &CliArgs) -> Result<(), Error> {
        Ok(())
    }

    /// Execute algorithm using the chosen extensions
    async fn run(&self, chooser: &Chooser, mirror: bool) -> Result<String, Error>;
}
