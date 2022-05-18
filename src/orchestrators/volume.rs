use std::time::Duration;

use async_trait::async_trait;
use derive_more::Constructor;
use regex::Regex;
use snafu::OptionExt;
use unwrap_infallible::UnwrapInfallible;

use crate::chooser::Chooser;
use crate::error::RegexCapture;
use crate::traits::outputer::Value;
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

/// Always increase depth and width by one in each iteration
#[derive(Constructor)]
pub struct VolumeOrchestrator;

#[async_trait]
impl Orchestrator for VolumeOrchestrator {
    async fn run(
        &self,
        chooser: &Chooser,
        width: i32,
        _: i32,
        iter: i32,
        rand: bool,
    ) -> Result<(), crate::Error> {
        let mut result = vec![];
        let mut durations = vec![];

        let re = Regex::new(r"(?P<result>\d+): (?P<val>\d+)")?;

        // generate test suite -> CircuitGenerator
        let mut generator = chooser.get_circuit_generator()?;
        let outputer = chooser.get_outputer()?;

        // connect to the provider -> QcProvider
        let mut provider = chooser.get_provider()?;
        provider.connect().await?;

        // TODO fix this
        // It runs dummy circuit to make the speed measurement more precise
        if let Some(circuit) = generator.generate(0, 0, 0).await? {
            provider.set_circuit(circuit.clone()).await?;
            provider.start_measure();
            provider.run().await?;
            provider.stop_measure();
        }

        // TODO for now it generates empty for not computed ones
        for i in 0..width {
            let mut time = Duration::from_micros(0);
            let mut val = Value::builder().result("".to_string()).correct(0).build();
            for ii in 0..iter {
                let rand_i = if rand { ii } else { 0 };
                if let Some(circuit) = generator.generate(i, i, rand_i).await? {
                    // TODO if I do a multiple iterations and one falls below limit, how to
                    // solve this?
                    provider.set_circuit(circuit.clone()).await?;

                    provider.start_measure();
                    let res = provider.run().await?;
                    time += provider.stop_measure();

                    let c = re.captures(&res).context(RegexCapture)?;
                    val.result = c["result"].parse::<String>().unwrap_infallible();
                    val.correct = c["val"].parse::<i32>()?;
                }
                else {
                    break;
                }

                durations.push(Duration::from_millis((time.as_millis() as u64) / (iter as u64))); // TODO
                result.push(val.clone());

                if val.correct as f64 <= 1024.0 * (2.0 / 3.0) {
                    break;
                }
            }
        }

        // get measured results
        // output -> Outputer
        outputer.output_volume(result, durations).await?;

        Ok(())
    }
}
