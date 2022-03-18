#![feature(async_closure)]
#![feature(try_blocks)]
#![feature(box_syntax)]
#![feature(negative_impls)]
#![feature(trait_alias)]
#![feature(proc_macro_hygiene)]
#![feature(generators)]
#![feature(stmt_expr_attributes)]
#![feature(extend_one)]
#![feature(core_intrinsics)]
#![feature(result_flattening)]

use crate::traits::{QcProvider, CircuitGenerator, Outputer};
use derive_more::Constructor;
use fehler::throws;

mod args;
pub use args::ARGS;

mod chooser;
mod circuit_generators;
mod outputers;
mod qc_providers;

pub mod traits;

mod error;
pub use error::Error;

use crate::chooser::Chooser;

#[derive(Constructor)]
pub struct Programm;

impl Programm {
    #[throws]
    pub async fn run() {
        println!("Hello world!");

        // use ARGS

        // generate test suite -> CircuitGenerator
        let generator = Chooser::get_circuit_generator()?;
        let circuit = generator.generate().await?;
        

        // connect to the provider -> QcProvider
        let provider = Chooser::get_provider()?;
        provider.connect().await?;

        
        // start measuring -> MeasureTool
        // run -> Executor
        let result = provider.run(circuit).await?;
        
        // get measured results
        // output -> Outputer
        let outputer = Chooser::get_outputer()?;
        outputer.output(result).await?;
    }
}
