use std::time::Duration;

use async_trait::async_trait;
use regex::Regex;
use snafu::OptionExt;
use tokio::time::Instant;
use unwrap_infallible::UnwrapInfallible;

use crate::args::CliArgsOrch;
use crate::chooser::Chooser;
use crate::error::RegexCapture;
use crate::traits::outputer::Value;
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};

/// Does a single run of some specific size
pub struct SingleOrchestrator {
    args: CliArgsOrch,
}

impl SingleOrchestrator {
    pub fn new(args: &CliArgsOrch) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl Orchestrator for SingleOrchestrator {
    async fn run(&self, chooser: &Chooser) -> Result<(), crate::Error> {
        let i = self.args.size;
        let j = self.args.size_2;
        let iter = self.args.iter;

        let mut result = vec![];
        let mut durations = vec![];

        // generate test suite -> CircuitGenerator
        let mut generator = chooser.get_circuit_generator()?;
        let outputer = chooser.get_outputer()?;

        let re = Regex::new(r"(?P<result>\d+): (?P<val>\d+)")?;

        // connect to the provider -> QcProvider
        let mut provider = chooser.get_provider()?;
        provider.connect().await?;

        // It runs dummy circuit to make the speed measurement more precise
        // if let Some(circuit) = generator.generate(0, 0, 0, false).await? {
        //     provider.set_circuit(circuit.clone()).await?;
        //     provider.start_measure();
        //     provider.run().await?;
        //     provider.stop_measure();
        // }

        println!("Dummy run done");
        let runtime = Instant::now();

        let mut sr = vec![];

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

                let c = re.captures(&res).context(RegexCapture)?;
                val.result = c["result"].parse::<String>().unwrap_infallible();
                val.correct = c["val"].parse::<i32>()?;
            }
        }
        durations.push(Duration::from_millis((time.as_millis() as u64) / (iter as u64))); // TODO
        sr.push(val.clone());

        result.push(sr.clone());

        // get measured results
        // output -> Outputer
        outputer.output_table(result, Some(durations), Instant::now() - runtime).await?; // TODO

        Ok(())
    }
}
