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

use crate::chooser::Chooser;
use crate::traits::{CircuitGenerator, Outputer, QcProvider};

mod args;

mod chooser;
mod circuit_generators;
mod error;
mod outputers;
mod qc_providers;

pub mod traits;

pub use args::ARGS;
pub use error::Error;

#[derive(Constructor)]
pub struct Quep;

impl Quep {
    #[throws]
    pub async fn run() {
        let mut result = vec![];
        let mut durations = vec![];

        let a = ARGS.size;

        'main: for i in 0..=a {
            let mut sr = vec![];

            for j in 0..=a {
                // generate test suite -> CircuitGenerator
                let generator = Chooser::get_circuit_generator()?;
                if let Some(circuit) = generator.generate(i, j).await? {
                    // connect to the provider -> QcProvider
                    let mut provider = Chooser::get_provider()?;
                    provider.connect().await?;

                    // start measuring -> MeasureTool
                    // run -> Executor
                    provider.start_measure();
                    sr.push(provider.run(circuit).await?);
                    durations.push(provider.stop_measure());
                }
                else {
                    break 'main;
                }
            }

            result.push(sr);
        }

        // get measured results
        // output -> Outputer
        let outputer = Chooser::get_outputer()?;
        outputer.output(result, durations).await?;
    }
}
