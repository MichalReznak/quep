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
#![feature(new_uninit)]

use chooser::Chooser;
use ext::Orchestrator;
use fehler::throws;

mod chooser;
mod circuit_generators;
mod error;
mod lang_schemas;
mod orchestrators;
mod outputers;
mod qc_providers;

pub mod args;
pub mod ext;
pub mod pyvenv;
pub mod utils;

pub use args::CliArgs;
pub use error::Error;

use crate::args::config;
use crate::utils::dir;

pub struct Quep {
    args: CliArgs,
}

impl Quep {
    #[throws]
    pub async fn new(args: CliArgs) -> Self {
        pyvenv::PyVenv::init(&args.provider.python_dir).await?;
        println!("Done");
        Self { args }
    }

    #[throws]
    pub async fn from_env() -> Self {
        let config_path = dir("./quep.json5")?;
        println!("Config path: {config_path}");

        let args = CliArgs::parse_with_config(&config_path)?;
        println!("{args:#?}");

        pyvenv::PyVenv::init(&args.provider.python_dir).await?;

        println!("Done");
        Self { args }
    }

    #[throws]
    pub async fn run(self) {
        let chooser = Chooser::new(self.args.clone());
        let orch = chooser.get_orchestrator()?;
        orch.run(&chooser).await?;
    }
}
