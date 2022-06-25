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
use crate::ext::outputer::Value;
use crate::ext::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

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
    async fn run(&self, chooser: &Chooser, mirror: bool) -> Result<String, crate::Error> {
        let from_i = self.args.from_size;
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

        let mut simulator = if !mirror {
            let mut a = chooser.get_simulator()?;
            a.connect().await?;
            Some(a)
        }
        else {
            None
        };

        // TODO fix this
        // It runs dummy circuit to make the speed measurement more precise
        if self.args.preheat {
            if let Some(circuit) = generator.generate(1, 1, 0).await? {
                provider.append_circuit(circuit.clone()).await?;
                provider.run().await?;
            }
        }

        println!("Dummy run done");
        let runtime = Instant::now();

        if self.args.collect {
            'main: for j in 1..=i {
                for ii in 0..iter {
                    if let Some(c) = generator.generate(depth, j, ii).await? {
                        provider.append_circuit(c.clone()).await?;

                        if !mirror {
                            simulator.as_mut().unwrap().append_circuit(c.clone()).await?;
                        }
                    }
                    else {
                        break 'main;
                    }
                }
            }

            let res = provider.run().await?;

            let sim_res = if !mirror {
                simulator.as_mut().unwrap().run().await?
            }
            else {
                vec![]
            };

            let result = res
                .into_iter()
                .chunks(iter as usize)
                .into_iter()
                .enumerate()
                .map(|(i, res)| {
                    let mut val = Value::builder()
                        .result("".to_string())
                        .correct(0)
                        .is_correct(false)
                        .build();

                    // Skip first N iterations if defined
                    // TODO this can be done smarter
                    if i < (from_i - 1) as usize {
                        return val;
                    }

                    for r in res {
                        let c = re.captures(&r).context(RegexCapture).unwrap();
                        val.result = c["result"].parse::<String>().unwrap_infallible();
                        val.correct += c["val"].parse::<i32>().unwrap();
                    }
                    val.correct /= iter;

                    val.is_correct = if !mirror {
                        let ci = i * (iter as usize);
                        let res =
                            sim_res.get((ci as usize)..(ci as usize + (iter as usize))).unwrap();
                        println!("{res:?}");

                        let mut sim_val = Value::builder()
                            .result("".to_string())
                            .correct(0)
                            .is_correct(false)
                            .build();
                        for r in res.iter() {
                            let c = re.captures(r).context(RegexCapture).unwrap();
                            sim_val.result = c["result"].parse::<String>().unwrap_infallible();
                            sim_val.correct += c["val"].parse::<i32>().unwrap();
                        }
                        sim_val.correct /= iter;
                        println!("{sim_val:#?}");

                        let d = (sim_val.correct as f64) * (1.0 / 3.0);
                        sim_val.result == val.result && (sim_val.correct - val.correct) as f64 <= d
                    }
                    else {
                        (val.correct as f64) > 1024.0 * (2.0 / 3.0)
                    };

                    val
                })
                .collect();

            outputer.output_linear(result, None, depth, Instant::now() - runtime).await
        }
        else {
            'main2: for j in 1..=i {
                let mut time = Duration::from_micros(0);
                let mut val =
                    Value::builder().result("".to_string()).correct(0).is_correct(false).build();
                let mut sim_val =
                    Value::builder().result("".to_string()).correct(0).is_correct(false).build();

                // Skip first N iterations if defined
                // TODO this can be done smarter
                if j < from_i {
                    durations.push(Duration::from_millis(0));
                    result.push(val.clone());
                    continue;
                }

                for ii in 0..iter {
                    // TODO somehow better allow to define circuit width
                    // (or if it should increase width instead of depth?)
                    if let Some(circuit) = generator.generate(depth, j, ii).await? {
                        provider.append_circuit(circuit.clone()).await?;

                        let res = provider.run().await?.get(0).unwrap().to_string();
                        time += provider.meta_info().await?.time;

                        let c = re.captures(&res).context(RegexCapture)?;
                        // TODO check if result is the same
                        val.result = c["result"].parse::<String>().unwrap_infallible();
                        val.correct += c["val"].parse::<i32>()?;

                        sim_val.result = c["result"].parse::<String>().unwrap_infallible();
                        sim_val.correct += c["val"].parse::<i32>()?;
                    }
                    else {
                        break 'main2;
                    }
                }
                val.correct /= iter;
                sim_val.correct /= iter;

                val.is_correct = if !mirror {
                    let d = (sim_val.correct as f64) * (1.0 / 3.0);
                    sim_val.result == val.result && (sim_val.correct - val.correct) as f64 <= d
                }
                else {
                    (val.correct as f64) > 1024.0 * (2.0 / 3.0)
                };

                durations.push(Duration::from_millis((time.as_millis() as u64) / (iter as u64))); // TODO
                result.push(val.clone());

                if (val.correct as f64) <= 1024.0 * (2.0 / 3.0) {
                    break;
                }
            }

            outputer
                .output_linear(result, Some(durations), depth, Instant::now() - runtime)
                .await
        }
    }
}
