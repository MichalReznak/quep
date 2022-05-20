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

/// Does a single run of some specific size
#[derive(Constructor)]
pub struct SingleOrchestrator;

#[async_trait]
impl Orchestrator for SingleOrchestrator {
    async fn run(
        &self,
        chooser: &Chooser,
        i: i32,
        j: i32,
        iter: i32,
        rand: bool,
    ) -> Result<(), crate::Error> {
        let mut result = vec![];
        let mut durations = vec![];

        // generate test suite -> CircuitGenerator
        let mut generator = chooser.get_circuit_generator()?;
        let outputer = chooser.get_outputer()?;

        let re = Regex::new(r"(?P<result>\d+): (?P<val>\d+)")?;

        // connect to the provider -> QcProvider
        let mut provider = chooser.get_provider()?;
        provider.connect().await?;

        // TODO enable
        // TODO fix this
        // It runs dummy circuit to make the speed measurement more precise
        // if let Some(circuit) = generator.generate(0, 0, 0).await? {
        //     provider.set_circuit(circuit.clone()).await?;
        //     provider.start_measure();
        //     provider.run().await?;
        //     provider.stop_measure();
        // }
        //
        // println!("Dummy run done");

        let mut sr = vec![];

        let mut time = Duration::from_micros(0);
        let mut val = Value::builder().result("".to_string()).correct(0).build();
        for ii in 0..iter {
            let rand_i = if rand { ii } else { 0 };

            if let Some(circuit) = generator.generate(i, j, rand_i).await? {
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
        outputer.output_table(result, durations).await?;

        Ok(())
    }
}
