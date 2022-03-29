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

use derive_more::Constructor;
use fehler::throws;

use crate::traits::QcProvider;
use crate::traits::{CircuitGenerator, Outputer};
use crate::chooser::Chooser;

mod args;

mod error;
mod chooser;
mod circuit_generators;
mod outputers;
mod qc_providers;

pub mod traits;

pub use error::Error;
pub use args::ARGS;


#[derive(Constructor)]
pub struct Quep;

impl Quep {
    #[throws]
    pub async fn run() {
        let mut i = 0;
        loop {
            // generate test suite -> CircuitGenerator
            let generator = Chooser::get_circuit_generator()?;
            if let Some(circuit) = generator.generate(i).await? {
                i += 1;

                // connect to the provider -> QcProvider
                let mut provider = Chooser::get_provider()?;
                provider.connect().await?;

                // start measuring -> MeasureTool
                // run -> Executor
                provider.start_measure();
                let result = provider.run(circuit).await?;
                let duration = provider.stop_measure();

                // get measured results
                // output -> Outputer
                let outputer = Chooser::get_outputer()?;
                outputer.output(result, duration).await?;
            }
            else {
                break;
            }
        }
    }
}
