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

/// Linear iteration from 0 to MAX
#[derive(Constructor)]
pub struct LinearOrchestrator;

#[async_trait]
impl Orchestrator for LinearOrchestrator {
    async fn run(
        &self,
        chooser: &Chooser,
        i: i32,
        depth: i32,
        iter: i32,
    ) -> Result<(), crate::Error> {
        let mut result = vec![];
        let mut durations = vec![];

        let re = Regex::new(r"(?P<result>\d+): (?P<val>\d+)")?;

        // generate test suite -> CircuitGenerator
        let generator = chooser.get_circuit_generator()?;
        let outputer = chooser.get_outputer()?;

        // connect to the provider -> QcProvider
        let mut provider = chooser.get_provider()?;
        provider.connect().await?;

        // TODO fix this
        // It runs dummy circuit to make the speed measurement more precise
        if let Some(circuit) = generator.generate(0, 0).await? {
            provider.set_circuit(circuit.clone()).await?;
            provider.start_measure();
            provider.run().await?;
            provider.stop_measure();
        }

        for j in 0..i {
            // TODO somehow better allow to define circuit width
            // (or if it should increase width instead of depth?)
            if let Some(circuit) = generator.generate(depth - 1, j).await? {
                // start measuring -> MeasureTool
                // run -> Executor

                let mut time = Duration::from_micros(0);
                let mut val = Value::builder().result("".to_string()).correct(0).build();
                for _ in 0..iter {
                    provider.set_circuit(circuit.clone()).await?;

                    provider.start_measure();
                    let res = provider.run().await?;
                    time += provider.stop_measure();

                    let c = re.captures(&res).context(RegexCapture)?;
                    val.result = c["result"].parse::<String>().unwrap_infallible();
                    val.correct = c["val"].parse::<i32>()?;
                }

                durations.push(Duration::from_millis((time.as_millis() as u64) / (iter as u64))); // TODO
                result.push(val.clone());

                if (val.correct as f64) <= 1024.0 * (2.0 / 3.0) {
                    break;
                }
            }
            else {
                break;
            }
        }

        // get measured results
        // output -> Outputer
        outputer.output_linear(result, durations, depth).await?;

        Ok(())
    }
}
