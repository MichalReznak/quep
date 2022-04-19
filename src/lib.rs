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

use std::path::{Path, PathBuf};
use fehler::throws;
use regex::Regex;

use crate::chooser::Chooser;
use crate::traits::{CircuitGenerator, Outputer, QcProvider};

use pyo3::prelude::*;

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
    pub async fn new() -> Self {
        // check if python dir is available
        let venv_dir = format!("{}/.venv", ARGS.python_dir);
        let pip = format!("{}/.venv/Scripts/pip", ARGS.python_dir);
        let req = format!("{}/requirements.txt", ARGS.python_dir);

        if !Path::new(&venv_dir).exists() {
            // install .venv
            println!("Installing virtualenv...");
            Command::new("python")
                .args(["-m", "venv", &venv_dir])
                .spawn()
                .unwrap()
                .wait().await.unwrap();
        }

        if !Path::new(&format!("{}/cmake", &venv_dir)).exists() {
            // Check if qiskit exists
            // run in venv pip install
            println!("Installing qiskit...");
            Command::new(&pip)
                .args(["install", "-r", &req])
                .spawn()
                .unwrap().wait().await.unwrap();
        }

        // set correct paths
        Python::with_gil(|py| {
            Python::run(py, &unindent::unindent(r#"
                import sys
                sys.path.append('./python/.venv/lib/site-packages')
                sys.path.append('./python/.venv')
            "#), None, None).unwrap();
        });

        println!("Done");
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
        let re = Regex::new(r"(\d+): (?P<val>\d+)").unwrap();

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

                    let c = re.captures(&res).unwrap();
                    if c["val"].parse::<f64>().unwrap() <= 1024.0 * (2.0 / 3.0) {
                        break;
                    }
                }
                else {
                    break 'main;
                }
            }

            result.push(sr.clone());
            let c = re.captures(&sr.get(0).unwrap()).unwrap();
            if c["val"].parse::<f64>().unwrap() <= 1024.0 * (2.0 / 3.0) && sr.len() == 1 {
                break;
            }
        }

        // get measured results
        // output -> Outputer
        let outputer = Chooser::get_outputer()?;
        outputer.output(result, durations).await?;
    }
}
