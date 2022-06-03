use std::time::Duration;

use async_trait::async_trait;
use itertools::Itertools;
use regex::Regex;
use snafu::OptionExt;
use tokio::time::Instant;
use unwrap_infallible::UnwrapInfallible;

use crate::args::CliArgsOrch;
use crate::chooser::Chooser;
use crate::error::RegexCapture;
use crate::traits::outputer::Value;
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

/// Always increase depth and width by one in each iteration
pub struct VolumeOrchestrator {
    args: CliArgsOrch,
}

impl VolumeOrchestrator {
    pub fn new(args: &CliArgsOrch) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl Orchestrator for VolumeOrchestrator {
    async fn run(&self, chooser: &Chooser) -> Result<(), crate::Error> {
        let width = self.args.size;
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
        if let Some(circuit) = generator.generate(0, 0, 0).await? {
            provider.set_circuit(circuit.clone()).await?;
            provider.start_measure();
            provider.run().await?;
            provider.stop_measure();
            provider.clear_circuits()?;
        }

        println!("Dummy run done");
        let runtime = Instant::now();

        if self.args.collect {
            // TODO add iterations
            'main: for i in 0..width {
                for ii in 0..iter {
                    if let Some(c) = generator.generate(i, i, ii).await? {
                        provider.append_circuit(c.clone()).await?;
                    }
                    else {
                        break 'main;
                    }
                }
            }

            let mut time = Duration::from_micros(0);
            provider.start_measure();
            let res = provider.run_all().await?;
            time += provider.stop_measure();

            let result = res
                .into_iter()
                .chunks(iter as usize)
                .into_iter()
                .map(|res| {
                    let mut val = Value::builder().result("".to_string()).correct(0).build();
                    for r in res {
                        let c = re.captures(&r).context(RegexCapture).unwrap();
                        val.result = c["result"].parse::<String>().unwrap_infallible();
                        val.correct += c["val"].parse::<i32>().unwrap();
                    }
                    val.correct /= iter;
                    val
                })
                .collect();

            outputer.output_volume(result, None, Instant::now() - runtime).await?;
        }
        else {
            // TODO for now it generates empty for not computed ones
            'main2: for i in 0..width {
                let mut time = Duration::from_micros(0);
                let mut val = Value::builder().result("".to_string()).correct(0).build();
                for ii in 0..iter {
                    if let Some(circuit) = generator.generate(i, i, ii).await? {
                        // TODO if I do a multiple iterations and one falls below limit, how to
                        // solve this?
                        provider.set_circuit(circuit.clone()).await?;

                        provider.start_measure();
                        let res = provider.run().await?;
                        time += provider.stop_measure();

                        let c = re.captures(&res).context(RegexCapture)?;
                        // TODO check if result is the same
                        val.result = c["result"].parse::<String>().unwrap_infallible();
                        val.correct += c["val"].parse::<i32>()?;
                    }
                    else {
                        break 'main2;
                    }
                }
                val.correct /= iter;

                durations.push(Duration::from_millis((time.as_millis() as u64) / (iter as u64))); // TODO
                result.push(val.clone());

                if val.correct as f64 <= 1024.0 * (2.0 / 3.0) {
                    break;
                }
            }

            outputer
                .output_volume(result, Some(durations), Instant::now() - runtime)
                .await?;
        }
        Ok(())
    }
}
