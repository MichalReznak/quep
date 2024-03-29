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
#![feature(iter_intersperse)]
#![feature(let_chains)]

use chooser::Chooser;
use ext::Orchestrator;
use fehler::throws;

mod chooser;
mod error;
mod extimpls;
mod globals;
pub use extimpls::*;

pub mod args;
pub mod ext;
pub mod pyvenv;
pub mod utils;

pub use args::CliArgs;
pub use error::Error;
pub use globals::*;

use crate::args::config;
use crate::args::types::{CircuitBenchType, CircuitType, LangSchemaType, OutputType, ProviderType};
use crate::utils::dir;

pub struct Quep {
    args: CliArgs,
}

impl Quep {
    #[throws]
    pub async fn new(args: CliArgs) -> Self {
        // Use python only when needed
        use ProviderType::*;

        if matches!(args.provider.t, Simple | Ibmq | Noisy | Python)
            || matches!(args.circuit.bench, CircuitBenchType::None)
            || matches!(args.output.t, OutputType::Python)
            || matches!(args.provider.t, ProviderType::Python)
            || matches!(args.circuit.t, CircuitType::Python)
            || matches!(args.lang_schema.t, LangSchemaType::Python)
        {
            pyvenv::PyVenv::init(&args.provider.python_dir).await?;
            println!("Done");
        }
        Self { args }
    }

    #[throws]
    pub async fn from_env() -> Self {
        let args = if let Ok(config_path) = dir("./quep.json5") {
            println!("Config path: {config_path}");

            CliArgs::parse_with_config(&config_path)?
        }
        else {
            println!("No config file found");
            CliArgs::parse()?
        };
        println!("{args:#?}");

        Self::new(args).await?
    }

    #[throws]
    pub async fn run(self) -> Option<String> {
        let chooser = Chooser::new(self.args.clone());
        let orch = chooser.get_orchestrator()?;
        let mirror = !matches!(self.args.circuit.bench, CircuitBenchType::None);

        orch.check_constraints(&self.args)?;
        orch.run(&chooser, mirror).await?
    }
}
