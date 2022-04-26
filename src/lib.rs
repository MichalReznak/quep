#![feature(async_closure)]
#![feature(try_blocks)]
#![feature(box_syntax)]
#![feature(negative_impls)]
#![feature(trait_alias)]
#![feature(proc_macro_hygiene)]
#![feature(generators)]
#![feature(stmt_expr_attributes)]
#![feature(extend_one)]
#![feature(core_intrinsics)]
#![feature(result_flattening)]
#![feature(string_remove_matches)]

use fehler::throws;
use log::info;

use crate::chooser::Chooser;

mod args;
mod chooser;
mod circuit_generators;
mod error;
mod orchestrators;
mod outputers;
mod qc_providers;

pub mod pyvenv;
pub mod traits;

pub use args::ARGS;
pub use error::Error;

use crate::traits::Orchestrator;

pub struct Quep;

// TODO check if set combination is correct or not

impl Quep {
    #[cfg(feature = "qiskit")]
    #[throws]
    pub async fn new() -> Self {
        pyvenv::PyVenv::init().await?;
        info!("Done");
        Self
    }

    #[cfg(not(feature = "qiskit"))]
    pub fn new() {
        unimplemented!();
    }

    #[throws]
    pub async fn run(self) {
        let orch = Chooser::get_orchestrator()?;
        orch.run().await?;
    }
}
