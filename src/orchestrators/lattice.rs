use std::time::Duration;

use async_trait::async_trait;
use regex::internal::Inst;
use regex::Regex;
use snafu::OptionExt;
use tokio::time::Instant;
use unwrap_infallible::UnwrapInfallible;

use crate::args::CliArgsOrch;
use crate::chooser::Chooser;
use crate::error::{OutOfBounds, RegexCapture};
use crate::traits::outputer::Value;
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

/// Iterates in all combination for 2D array
pub struct LatticeOrchestrator {
    args: CliArgsOrch,
}

impl LatticeOrchestrator {
    pub fn new(args: &CliArgsOrch) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl Orchestrator for LatticeOrchestrator {
    async fn run(&self, chooser: &Chooser) -> Result<(), crate::Error> {
        let i = self.args.size;
        let j = self.args.size_2;
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

        // TODO should be as close to computation as possible
        let runtime = Instant::now();

        if self.args.collect {
            // TODO add iterations
            'main: for i in 0..i {
                for j in 0..j {
                    if let Some(c) = generator.generate(i, j, 0 /* TODO iter */, false).await? {
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

            for ii in 0..i {
                let mut sr = vec![];

                for jj in 0..j {
                    let ci = (ii * j) + jj;
                    let c = res.get(ci as usize).unwrap();

                    let mut val = Value::builder().result("".to_string()).correct(0).build();
                    let c = re.captures(&c).context(RegexCapture).unwrap();
                    val.result = c["result"].parse::<String>().unwrap_infallible();
                    val.correct = c["val"].parse::<i32>().unwrap();
                    sr.push(val);
                }

                result.push(sr);
            }

            outputer.output_table(result, None, Instant::now() - runtime).await?;
        }
        else {
            'main: for i in 0..i {
                let mut sr = vec![];

                for j in 0..j {
                    let mut time = Duration::from_micros(0);
                    let mut val = Value::builder().result("".to_string()).correct(0).build();
                    for ii in 0..iter {
                        if let Some(circuit) = generator.generate(i, j, ii, false).await? {
                            // TODO if I do a multiple iterations and one falls below limit, how to
                            // solve this?
                            provider.set_circuit(circuit.clone()).await?;

                            provider.start_measure();
                            let res = provider.run().await?;
                            time += provider.stop_measure();

                            // TODO value is always overwritten in all orch
                            let c = re.captures(&res).context(RegexCapture)?;
                            val.result = c["result"].parse::<String>().unwrap_infallible();
                            val.correct = c["val"].parse::<i32>()?;
                        }
                        else {
                            result.push(sr.clone());
                            break 'main;
                        }
                    }

                    durations
                        .push(Duration::from_millis((time.as_millis() as u64) / (iter as u64))); // TODO
                    sr.push(val.clone());

                    if (val.correct as f64) <= 1024.0 * (2.0 / 3.0) {
                        break;
                    }
                }

                result.push(sr.clone());
                let c = sr.get(0).context(OutOfBounds)?.correct;
                if (c as f64) <= 1024.0 * (2.0 / 3.0) && sr.len() == 1 {
                    break;
                }
            }

            outputer.output_table(result, Some(durations), Instant::now() - runtime).await?;
        }

        Ok(())
    }
}
