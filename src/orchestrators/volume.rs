use async_trait::async_trait;
use derive_more::Constructor;
use regex::Regex;
use snafu::OptionExt;

use crate::chooser::Chooser;
use crate::error::RegexCapture;
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

/// Always increase depth and width by one in each iteration
#[derive(Constructor)]
pub struct VolumeOrchestrator;

#[async_trait]
impl Orchestrator for VolumeOrchestrator {
    async fn run(&self, chooser: &Chooser, width: i32, _: i32) -> Result<(), crate::Error> {
        let mut result = vec![];
        let mut durations = vec![];

        let re = Regex::new(r"(\d+): (?P<val>\d+)")?;

        // generate test suite -> CircuitGenerator
        let generator = chooser.get_circuit_generator()?;
        let outputer = chooser.get_outputer()?;

        // connect to the provider -> QcProvider
        let mut provider = chooser.get_provider()?;
        provider.connect().await?;

        // TODO for now it generates empty for not computed ones
        for i in 0..width {
            if let Some(circuit) = generator.generate(i, i).await? {
                // start measuring -> MeasureTool
                // run -> Executor
                provider.start_measure();
                let res = provider.run(circuit).await?;
                durations.push(provider.stop_measure());
                result.push(res.clone());

                let c = re.captures(&res).context(RegexCapture)?;
                if c["val"].parse::<f64>()? <= 1024.0 * (2.0 / 3.0) {
                    break;
                }
            }
            else {
                break;
            }
        }

        // get measured results
        // output -> Outputer
        outputer.output_volume(result, durations).await?;

        Ok(())
    }
}
