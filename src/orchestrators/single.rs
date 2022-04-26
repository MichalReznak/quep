use async_trait::async_trait;
use derive_more::Constructor;

use crate::chooser::Chooser;
use crate::traits::{CircuitGenerator, Orchestrator, Outputer, QcProvider};
use crate::ARGS;

/// Does a single run of some specific size
#[derive(Constructor)]
pub struct SingleOrchestrator;

#[async_trait]
impl Orchestrator for SingleOrchestrator {
    async fn run(&self) -> Result<(), crate::Error> {
        let mut result = vec![];
        let mut durations = vec![];

        // generate test suite -> CircuitGenerator
        let generator = Chooser::get_circuit_generator()?;
        let outputer = Chooser::get_outputer()?;

        // connect to the provider -> QcProvider
        let mut provider = Chooser::get_provider()?;
        provider.connect().await?;

        let mut sr = vec![];
        if let Some(circuit) = generator.generate(ARGS.orch_size, ARGS.orch_size_2).await? {
            // start measuring -> MeasureTool
            // run -> Executor
            provider.start_measure();
            let res = provider.run(circuit).await?;
            sr.push(res.clone());
            durations.push(provider.stop_measure());
        }

        result.push(sr.clone());

        // get measured results
        // output -> Outputer
        outputer.output(result, durations).await?;

        Ok(())
    }
}
