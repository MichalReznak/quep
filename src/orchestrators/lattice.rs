use async_trait::async_trait;
use derive_more::Constructor;
use regex::Regex;
use snafu::OptionExt;

use crate::chooser::Chooser;
use crate::error::{OutOfBounds, RegexCapture};
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

/// Iterates in all combination for 2D array
#[derive(Constructor)]
pub struct LatticeOrchestrator;

#[async_trait]
impl Orchestrator for LatticeOrchestrator {
    async fn run(&self, chooser: &Chooser, i: i32, j: i32) -> Result<(), crate::Error> {
        let mut result = vec![];
        let mut durations = vec![];

        let re = Regex::new(r"(\d+): (?P<val>\d+)")?;

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
        outputer.output_table(result, durations).await?;

        Ok(())
    }
}
