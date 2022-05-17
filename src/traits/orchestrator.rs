//! Orchestrators
//!
//! This module synchronizes all other parts of the application
//! to do whatever needed

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::orchestrators::{
    LatticeOrchestrator, LinearOrchestrator, SingleOrchestrator, VolumeOrchestrator,
};
use crate::{Chooser, Error};

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
    async fn run(&self, chooser: &Chooser, i: i32, j: i32, iter: i32) -> Result<(), Error>;
}
