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

use fehler::throws;

mod args;
pub use args::ARGS;

mod circuit_generators;
mod qc_providers;

mod error;
pub use error::Error;

#[throws]
pub fn hello() -> String {
    "Hello world!".to_string()
}


pub struct Programm;

impl Programm {
    pub fn run() {
        // use ARGS
        // connect to the provider -> QcProvider
        // generate test suite -> CircuitGenerator
        // start measuring -> MeasureTool
        // run -> Executor
        // get measured results
        // output -> Outputer
    }
}
