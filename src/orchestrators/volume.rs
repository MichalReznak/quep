use async_trait::async_trait;
use derive_more::Constructor;
use regex::Regex;
use snafu::OptionExt;

use crate::chooser::Chooser;
use crate::error::{OutOfBounds, RegexCapture};
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};
use crate::ARGS;

/// Always increase depth and width by one in each iteration
#[derive(Constructor)]
pub struct VolumeOrchestrator;

#[async_trait]
impl Orchestrator for VolumeOrchestrator {
    async fn run(&self) -> Result<(), crate::Error> {
        let mut result = vec![];
        let mut durations = vec![];

        let re = Regex::new(r"(\d+): (?P<val>\d+)")?;

        // generate test suite -> CircuitGenerator
        let generator = Chooser::get_circuit_generator()?;
        let outputer = Chooser::get_outputer()?;

        // connect to the provider -> QcProvider
        let mut provider = Chooser::get_provider()?;
        provider.connect().await?;

        // TODO for now it generates empty for not computed ones
        'main: for i in 0..ARGS.orch_size {
            let mut sr = vec![];

            for j in 0..ARGS.orch_size {
                if i != j {
                    sr.push("0: 0".to_string());
                    continue;
                }

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

        Ok(())
    }
}
