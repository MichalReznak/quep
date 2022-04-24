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
use regex::Regex;
use snafu::OptionExt;

use crate::chooser::Chooser;
use crate::error::{OutOfBounds, RegexCapture};
use crate::traits::{CircuitGenerator, Outputer, QcProvider};
mod args;
mod chooser;
mod circuit_generators;
mod error;
mod outputers;
mod qc_providers;

pub mod pyvenv;
pub mod traits;

pub use args::ARGS;
pub use error::Error;

pub struct Quep;

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
        let mut result = vec![];
        let mut durations = vec![];

        let re = Regex::new(r"(\d+): (?P<val>\d+)")?;

        // generate test suite -> CircuitGenerator
        let generator = Chooser::get_circuit_generator()?;
        let outputer = Chooser::get_outputer()?;

        // connect to the provider -> QcProvider
        let mut provider = Chooser::get_provider()?;
        provider.connect().await?;

        'main: for i in 0..ARGS.size {
            let mut sr = vec![];

            for j in 0..ARGS.size {
                if let Some(circuit) = generator.generate(i, j).await? {
                    // start measuring -> MeasureTool
                    // run -> Executor
                    provider.start_measure();
                    let res = provider.run(circuit).await?;
                    sr.push(res.clone());
                    durations.push(provider.stop_measure());

                    let c = re.captures(&res).context(RegexCapture)?;
                    if c["val"].parse::<f64>()? <= 1024.0 * (2.0 / 3.0) {
                        break;
                    }
                }
                else {
                    result.push(sr.clone());
                    break 'main;
                }
            }

            result.push(sr.clone());
            let c = re.captures(&sr.get(0).context(OutOfBounds)?).context(RegexCapture)?;
            if c["val"].parse::<f64>()? <= 1024.0 * (2.0 / 3.0) && sr.len() == 1 {
                break;
            }
        }

        // get measured results
        // output -> Outputer
        outputer.output(result, durations).await?;
    }
}
