use std::time::Duration;

use async_trait::async_trait;
use regex::Regex;
use snafu::OptionExt;
use unwrap_infallible::UnwrapInfallible;

use crate::args::CliArgsOrch;
use crate::chooser::Chooser;
use crate::error::RegexCapture;
use crate::traits::outputer::Value;
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

/// Linear iteration from 0 to MAX
pub struct LinearOrchestrator {
    args: CliArgsOrch,
}

impl LinearOrchestrator {
    pub fn new(args: &CliArgsOrch) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl Orchestrator for LinearOrchestrator {
    async fn run(&self, chooser: &Chooser) -> Result<(), crate::Error> {
        let i = self.args.size;
        let depth = self.args.size_2;
        let iter = self.args.iter;

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
        if let Some(circuit) = generator.generate(0, 0, 0, false).await? {
            provider.set_circuit(circuit.clone()).await?;
            provider.start_measure();
            provider.run().await?;
            provider.stop_measure();
        }

        println!("Dummy run done");

        if self.args.collect {
            // TODO add iterations
            'main: for j in 0..i {
                if let Some(c) = generator.generate(depth - 1, j, 1 /* TODO */, false).await? {
                    provider.append_circuit(c.clone()).await?;
                }
                else {
                    break 'main;
                }
            }

            let mut time = Duration::from_micros(0);
            provider.start_measure();
            let res = provider.run_all().await?;
            time += provider.stop_measure();

            result = res
                .into_iter()
                .map(|res| {
                    let mut val = Value::builder().result("".to_string()).correct(0).build();
                    let c = re.captures(&res).context(RegexCapture).unwrap();
                    val.result = c["result"].parse::<String>().unwrap_infallible();
                    val.correct = c["val"].parse::<i32>().unwrap();
                    val
                })
                .collect();

            // TODO
            durations =
                std::iter::repeat(Duration::from_millis((time.as_millis() as u64) / (iter as u64)))
                    .take(i as usize)
                    .collect();
        }
        else {
            for j in 0..i {
                for ii in 0..iter {
                    let mut time = Duration::from_micros(0);
                    let mut val = Value::builder().result("".to_string()).correct(0).build();

                    // TODO somehow better allow to define circuit width
                    // (or if it should increase width instead of depth?)
                    if let Some(circuit) = generator.generate(depth - 1, j, ii, false).await? {
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

                    durations
                        .push(Duration::from_millis((time.as_millis() as u64) / (iter as u64))); // TODO
                    result.push(val.clone());

                    if (val.correct as f64) <= 1024.0 * (2.0 / 3.0) {
                        break;
                    }
                }
            }
        }

        // get measured results
        // output -> Outputer
        outputer
            .output_linear(result, Some(durations), depth, Duration::from_millis(0))
            .await?; // TODO
        Ok(())
    }
}
