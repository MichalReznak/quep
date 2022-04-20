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

use std::path::Path;

use fehler::throws;
use log::info;
use pyo3::prelude::*;
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

pub mod traits;

pub use args::ARGS;
pub use error::Error;
use tokio::process::Command;

pub struct Quep;

impl Quep {
    #[cfg(feature = "qiskit")]
    #[throws]
    pub async fn new() -> Self {
        // check if python dir is available
        let venv_dir = format!("{}/.venv", ARGS.python_dir);
        let pip = format!("{}/.venv/Scripts/pip", ARGS.python_dir);
        let req = format!("{}/requirements.txt", ARGS.python_dir);

        if !Path::new(&venv_dir).exists() {
            // install .venv
            info!("Installing virtualenv...");
            Command::new("python").args(["-m", "venv", &venv_dir]).spawn()?.wait().await?;
        }

        // Check if qiskit exists
        if !Path::new(&format!("{}/cmake", &venv_dir)).exists() {
            // run in venv pip install
            info!("Installing qiskit...");
            Command::new(&pip).args(["install", "-r", &req]).spawn()?.wait().await?;
        }

        // set correct paths
        Python::with_gil(|py| -> Result<_, Error> {
            Ok(Python::run(
                py,
                &unindent::unindent(&format!(
                    r#"
                        import sys
                        sys.path.append('{}/.venv/lib/site-packages')
                        sys.path.append('{}/.venv')
                    "#,
                    &ARGS.python_dir, &ARGS.python_dir
                )),
                None,
                None,
            )?)
        })?;

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

        let a = ARGS.size;
        let re = Regex::new(r"(\d+): (?P<val>\d+)")?;

        'main: for i in 0..a {
            let mut sr = vec![];

            for j in 0..a {
                // generate test suite -> CircuitGenerator
                let generator = Chooser::get_circuit_generator()?;
                if let Some(circuit) = generator.generate(i, j).await? {
                    // connect to the provider -> QcProvider
                    let mut provider = Chooser::get_provider()?;
                    provider.connect().await?;

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
        let outputer = Chooser::get_outputer()?;
        outputer.output(result, durations).await?;
    }
}
