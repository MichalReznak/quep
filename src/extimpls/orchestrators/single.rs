use std::time::Duration;

use async_trait::async_trait;
use fehler::throws;
use regex::Regex;
use snafu::OptionExt;
use tokio::time::Instant;
use unwrap_infallible::UnwrapInfallible;

use crate::args::CliArgsOrch;
use crate::chooser::Chooser;
use crate::error::RegexCapture;
use crate::ext::outputer::OutValue;
use crate::ext::{CircuitGenerator, LangSchema, Orchestrator, Outputer, QcProvider};
use crate::Error;

/// Does a single run of some specific size
pub struct SingleOrchestrator {
    args: CliArgsOrch,
}

impl SingleOrchestrator {
    #[throws]
    pub fn from_args(args: &CliArgsOrch) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl Orchestrator for SingleOrchestrator {
    async fn run(&self, chooser: &Chooser, mirror: bool) -> Result<Option<String>, Error> {
        let i = self.args.size;
        let j = self.args.size_2;
        let iter = self.args.iter;

        let mut result = vec![];
        let mut durations = vec![];

        // generate test suite -> CircuitGenerator
        let mut generator = chooser.get_circuit_generator()?;
        let outputer = chooser.get_outputer()?;
        let mut lang_schema = chooser.get_lang_schema()?;

        let re = Regex::new(r"(?P<result>\d+): (?P<val>\d+)")?;

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
        if self.args.preheat && let Some(circuit) = generator.generate(&lang_schema, 1, 1, 0).await? {
            let circuit = lang_schema.as_string(circuit.clone()).await?;
            provider.append_circuit(circuit.clone()).await?;
            provider.run().await?;

            println!("Pre-heat run done");
        }

        let runtime = Instant::now();

        if self.args.collect {
            for ii in 0..iter {
                if let Some(c) = generator.generate(&lang_schema, i, j, ii).await? {
                    let c = lang_schema.as_string(c.clone()).await?;
                    provider.append_circuit(c.clone()).await?;

                    if !mirror {
                        simulator.as_mut().unwrap().append_circuit(c.clone()).await?;
                    }
                }
                else {
                    break;
                }
            }

            let res = provider.run().await?;

            let mut val =
                OutValue::builder().result("".to_string()).correct(0).is_correct(false).build();
            for r in res {
                let c = re.captures(&r).context(RegexCapture).unwrap();
                val.result = c["result"].parse::<String>().unwrap_infallible();
                val.correct += c["val"].parse::<i32>().unwrap();
            }
            val.correct /= iter;

            val.is_correct = if !mirror {
                let res = simulator.as_mut().unwrap().run().await?;

                let mut sim_val =
                    OutValue::builder().result("".to_string()).correct(0).is_correct(false).build();
                for r in res {
                    let c = re.captures(&r).context(RegexCapture).unwrap();
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

            // get measured results
            // output -> Outputer
            outputer.output_table(vec![vec![val]], None, Instant::now() - runtime).await
        }
        else {
            let mut sr = vec![];
            let mut time = Duration::from_micros(0);
            let mut val =
                OutValue::builder().result("".to_string()).correct(0).is_correct(false).build();
            let mut sim_val =
                OutValue::builder().result("".to_string()).correct(0).is_correct(false).build();

            for ii in 0..iter {
                if let Some(circuit) = generator.generate(&lang_schema, i, j, ii).await? {
                    // TODO if I do a multiple iterations and one falls below limit, how to
                    // solve this?
                    let circuit = lang_schema.as_string(circuit.clone()).await?;
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

            durations.push(Duration::from_millis((time.as_millis() as u64) / (iter as u64)));
            sr.push(val.clone());

            result.push(sr.clone());

            // get measured results
            // output -> Outputer
            outputer.output_table(result, Some(durations), Instant::now() - runtime).await
        }
    }
}