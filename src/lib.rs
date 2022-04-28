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

use chooser::Chooser;
use clap::Parser;
use fehler::throws;
use log::info;
use traits::Orchestrator;

mod chooser;
mod circuit_generators;
mod error;
mod orchestrators;
mod outputers;
mod qc_providers;

pub mod args;
pub mod pyvenv;
pub mod traits;

pub use args::CliArgs;
pub use error::Error;

pub struct Quep {
    args: CliArgs,
}

// TODO check if set combination is correct or not

impl Quep {
    #[cfg(feature = "qiskit")]
    #[throws]
    pub async fn new() -> Self {
        dotenv::dotenv().ok();
        let args = CliArgs::parse();

        pyvenv::PyVenv::init(&args.python_dir).await?;

        info!("Done");
        Self { args }
    }

    #[throws]
    pub async fn with_args(args: CliArgs) -> Self {
        pyvenv::PyVenv::init(&args.python_dir).await?;
        info!("Done");
        Self { args }
    }

    #[cfg(not(feature = "qiskit"))]
    pub fn new() {
        unimplemented!();
    }

    #[throws]
    pub async fn run(self) {
        let chooser = Chooser::new(self.args.clone());
        let orch = chooser.get_orchestrator()?;
        orch.run(&chooser, self.args.orch_size, self.args.orch_size_2).await?;
    }
}
