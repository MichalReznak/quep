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
#![feature(string_remove_matches)]

use chooser::Chooser;
use clap::Parser;
use fehler::throws;
use load_file::load_str;
use log::info;
use traits::Orchestrator;

mod chooser;
mod circuit_generators;
mod error;
mod orchestrators;
mod outputers;
mod qc_providers;

pub mod args;
pub mod pyvenv;
pub mod traits;
pub mod utils;

pub use args::CliArgs;
pub use error::Error;

use crate::args::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use crate::args::{config, CliArgsConfig, CliArgsEnv};
use crate::utils::dir;

pub struct Quep {
    args: CliArgs,
}

impl Quep {
    #[throws]
    pub async fn new(args: CliArgs) -> Self {
        pyvenv::PyVenv::init(&args.python_dir).await?;
        info!("Done");
        Self { args }
    }

    #[throws]
    pub async fn from_env() -> Self {
        dotenv::dotenv().ok();
        let clap = CliArgsEnv::parse();

        // TODO define correct combinations
        // parse config file, json for now
        let config_path = dir("./quep.json5")?;
        println!("Config path: {config_path}");
        let config = load_str!(&config_path); // TODO panics on error, relative dir
        let config = json5::from_str::<CliArgsConfig>(config)?;

        let orch_data_dir = dir("./data")?;
        let python_dir = dir("./python")?;

        // TODO better?
        // if not set use it
        let args = CliArgs::builder()
            .provider(clap.provider.unwrap_or_else(|| ProviderType::Simple))
            .output(clap.output.or_else(|| config.output.t).unwrap_or_else(|| OutputType::Text))
            .output_ser(
                clap.output_ser
                    .or_else(|| config.output.ser)
                    .unwrap_or_else(|| OutputSerType::Json),
            )
            .circuit(
                clap.circuit.or_else(|| config.circuit.t).unwrap_or_else(|| CircuitType::Basic),
            )
            .circuit_rand(
                clap.circuit_rand.or_else(|| config.circuit.rand).unwrap_or_else(|| false),
            )
            .orch(clap.orch.or_else(|| config.orch.t).unwrap_or_else(|| OrchestratorType::Single))
            .orch_data(clap.orch_data.or_else(|| config.orch.data).unwrap_or_else(|| orch_data_dir))
            .orch_iter(clap.orch_iter.or_else(|| config.orch.iter).unwrap_or_else(|| 1))
            .orch_size(clap.orch_size.or_else(|| config.orch.size).unwrap_or_else(|| 1))
            .orch_size_2(clap.orch_size_2.or_else(|| config.orch.size_2).unwrap_or_else(|| 1))
            .python_dir(clap.python_dir.or_else(|| config.python_dir).unwrap_or_else(|| python_dir))
            .build();

        println!("{args:#?}");

        pyvenv::PyVenv::init(&args.python_dir).await?;

        info!("Done");
        Self { args }
    }

    #[throws]
    pub async fn run(self) {
        let chooser = Chooser::new(self.args.clone());
        let orch = chooser.get_orchestrator()?;
        orch.run(
            &chooser,
            self.args.orch_size,
            self.args.orch_size_2,
            self.args.orch_iter,
            self.args.circuit_rand,
        )
        .await?;
    }
}
