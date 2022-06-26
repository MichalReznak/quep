use std::time::Duration;

use async_trait::async_trait;
use regex::Regex;
use snafu::OptionExt;
use tokio::time::Instant;
use unwrap_infallible::UnwrapInfallible;

use crate::args::CliArgsOrch;
use crate::chooser::Chooser;
use crate::error::{OutOfBounds, RegexCapture};
use crate::ext::outputer::OutValue;
use crate::ext::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

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
    async fn run(&self, chooser: &Chooser, mirror: bool) -> Result<String, crate::Error> {
        let from_i = self.args.from_size;
        let from_j = self.args.from_size_2;
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

        let mut simulator = if !mirror {
            let mut a = chooser.get_simulator()?;
            a.connect().await?;
            Some(a)
        }
        else {
            None
        };

        // It runs dummy circuit to make the speed measurement more precise
        if self.args.preheat && let Some(circuit) = generator.generate(1, 1, 0).await? {
            provider.append_circuit(circuit.clone()).await?;
            provider.run().await?;

            println!("Pre-heat run done");
        }

        let runtime = Instant::now();

        if self.args.collect {
            'main: for i in 1..=i {
                for j in 1..=j {
                    for ii in 0..iter {
                        if let Some(c) = generator.generate(i, j, ii).await? {
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
            }

            let res = provider.run().await?;

            let sim_res = if !mirror {
                simulator.as_mut().unwrap().run().await?
            }
            else {
                vec![]
            };

            for ii in 0..i {
                let mut sr = vec![];

                for jj in 0..j {
                    let ci = ((ii * j) + jj) * iter;
                    let res = res.get((ci as usize)..(ci as usize + (iter as usize))).unwrap();

                    let mut val = OutValue::builder()
                        .result("".to_string())
                        .correct(0)
                        .is_correct(false)
                        .build();

                    // Skip first N iterations if defined
                    // TODO this can be done smarter
                    if ii < from_i - 1 || jj < from_j - 1 {
                        sr.push(val.clone());
                        continue;
                    }

                    for r in res {
                        let c = re.captures(r).context(RegexCapture).unwrap();
                        val.result = c["result"].parse::<String>().unwrap_infallible();
                        val.correct += c["val"].parse::<i32>().unwrap();
                    }
                    val.correct /= iter;

                    val.is_correct = if !mirror {
                        let sim_res =
                            sim_res.get((ci as usize)..(ci as usize + (iter as usize))).unwrap();

                        let mut sim_val = OutValue::builder()
                            .result("".to_string())
                            .correct(0)
                            .is_correct(false)
                            .build();
                        for r in sim_res.iter() {
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

                    sr.push(val);
                }

                result.push(sr);
            }

            outputer.output_table(result, None, Instant::now() - runtime).await
        }
        else {
            'main2: for i in 1..=i {
                let mut sr = vec![];

                for j in 1..=j {
                    let mut time = Duration::from_micros(0);
                    let mut val = OutValue::builder()
                        .result("".to_string())
                        .correct(0)
                        .is_correct(false)
                        .build();
                    let mut sim_val = OutValue::builder()
                        .result("".to_string())
                        .correct(0)
                        .is_correct(false)
                        .build();

                    // Skip first N iterations if defined
                    // TODO this can be done smarter
                    if i < from_i || j < from_j {
                        durations.push(Duration::from_millis(0));
                        sr.push(val.clone());
                        continue;
                    }

                    for ii in 0..iter {
                        if let Some(circuit) = generator.generate(i, j, ii).await? {
                            // TODO if I do a multiple iterations and one falls below limit, how to
                            // solve this?
                            provider.append_circuit(circuit.clone()).await?;

                            let res = provider.run().await?.get(0).unwrap().to_string();
                            time += provider.meta_info().await?.time;

                            // TODO value is always overwritten in all orch
                            let c = re.captures(&res).context(RegexCapture)?;
                            // TODO check if result is the same
                            val.result = c["result"].parse::<String>().unwrap_infallible();
                            val.correct += c["val"].parse::<i32>()?;

                            sim_val.result = c["result"].parse::<String>().unwrap_infallible();
                            sim_val.correct += c["val"].parse::<i32>()?;
                        }
                        else {
                            result.push(sr.clone());
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

                    durations
                        .push(Duration::from_millis((time.as_millis() as u64) / (iter as u64)));
                    sr.push(val.clone());

                    if !val.is_correct {
                        break;
                    }
                }

                result.push(sr.clone());
                let c = sr.get(0).context(OutOfBounds)?.is_correct;
                if c && sr.len() == 1 {
                    break;
                }
            }

            outputer.output_table(result, Some(durations), Instant::now() - runtime).await
        }
    }
}
