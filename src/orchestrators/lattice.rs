use std::time::Duration;

use async_trait::async_trait;
use derive_more::Constructor;
use regex::Regex;
use snafu::OptionExt;

use unwrap_infallible::UnwrapInfallible;

use crate::chooser::Chooser;
use crate::error::{OutOfBounds, RegexCapture};
use crate::traits::outputer::Value;
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

/// Iterates in all combination for 2D array
#[derive(Constructor)]
pub struct LatticeOrchestrator;

#[async_trait]
impl Orchestrator for LatticeOrchestrator {
    async fn run(&self, chooser: &Chooser, i: i32, j: i32, iter: i32) -> Result<(), crate::Error> {
        let mut result = vec![];
        let mut durations = vec![];

        let re = Regex::new(r"(?P<result>\d+): (?P<val>\d+)")?;

        // generate test suite -> CircuitGenerator
        let generator = chooser.get_circuit_generator()?;
        let outputer = chooser.get_outputer()?;

        // connect to the provider -> QcProvider
        let mut provider = chooser.get_provider()?;
        provider.connect().await?;

        'main: for i in 0..i {
            let mut sr = vec![];

            for j in 0..j {
                if let Some(circuit) = generator.generate(i, j).await? {
                    // start measuring -> MeasureTool
                    // run -> Executor

                    let mut time = Duration::from_micros(0);
                    let mut val = Value::builder().result("".to_string()).correct(0).build();
                    for _ in 0..iter {
                        // TODO if I do a multiple iterations and one falls below limit, how to
                        // solve this?
                        provider.start_measure();
                        let res = provider.run(circuit.clone()).await?;

                        let c = re.captures(&res).context(RegexCapture)?;
                        val.result = c["result"].parse::<String>().unwrap_infallible();
                        val.correct = c["val"].parse::<i32>()?;
                        time += provider.stop_measure();
                    }
                    durations
                        .push(Duration::from_millis((time.as_millis() as u64) / (iter as u64))); // TODO
                    sr.push(val.clone());

                    if (val.correct as f64) <= 1024.0 * (2.0 / 3.0) {
                        break;
                    }
                }
                else {
                    result.push(sr.clone());
                    break 'main;
                }
            }

            result.push(sr.clone());
            let c = sr.get(0).context(OutOfBounds)?.correct;
            if (c as f64) <= 1024.0 * (2.0 / 3.0) && sr.len() == 1 {
                break;
            }
        }

        // get measured results
        // output -> Outputer
        outputer.output_table(result, durations).await?;

        Ok(())
    }
}
