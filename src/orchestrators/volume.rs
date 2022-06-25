use std::time::Duration;

use async_trait::async_trait;
use itertools::Itertools;
use regex::Regex;
use snafu::OptionExt;
use tokio::time::Instant;
use unwrap_infallible::UnwrapInfallible;

use crate::args::types::CircuitType;
use crate::args::CliArgsOrch;
use crate::chooser::Chooser;
use crate::error::{Constraint, RegexCapture};
use crate::ext::outputer::Value;
use crate::ext::{CircuitGenerator, Orchestrator, Outputer, QcProvider};
use crate::{CliArgs, Error};

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
    fn check_constraints(&self, args: &CliArgs) -> Result<(), Error> {
        if !matches!(args.circuit.t, CircuitType::Volume) {
            Constraint {
                reason: "Volume Orchestrator requires Volume circuit generator".to_string(),
            }
            .fail()?;
        }
        Ok(())
    }

    async fn run(&self, chooser: &Chooser, mirror: bool) -> Result<String, Error> {
        let from_i = self.args.from_size;
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
            // TODO add iterations
            'main: for i in 1..=width {
                for ii in 0..iter {
                    if let Some(c) = generator.generate(i, i, ii).await? {
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

                        let d = (sim_val.correct as f64) * (1.0 / 3.0);
                        sim_val.result == val.result && (sim_val.correct - val.correct) as f64 <= d
                    }
                    else {
                        (val.correct as f64) > 1024.0 * (2.0 / 3.0)
                    };

                    val
                })
                .collect();

            outputer.output_volume(result, None, Instant::now() - runtime).await
        }
        else {
            // TODO for now it generates empty for not computed ones
            'main2: for i in 1..=width {
                let mut time = Duration::from_micros(0);
                let mut val =
                    Value::builder().result("".to_string()).correct(0).is_correct(false).build();
                let mut sim_val =
                    Value::builder().result("".to_string()).correct(0).is_correct(false).build();

                // Skip first N iterations if defined
                // TODO this can be done smarter
                if i < from_i {
                    durations.push(Duration::from_millis(0));
                    result.push(val.clone());
                    continue;
                }

                for ii in 0..iter {
                    if let Some(circuit) = generator.generate(i, i, ii).await? {
                        // TODO if I do a multiple iterations and one falls below limit, how to
                        // solve this?
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

                if val.correct as f64 <= 1024.0 * (2.0 / 3.0) {
                    break;
                }
            }

            outputer.output_volume(result, Some(durations), Instant::now() - runtime).await
        }
    }
}
