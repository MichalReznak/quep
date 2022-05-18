use std::ffi::OsStr;

use clap::Parser;
use fehler::throws;
use snafu::OptionExt;

use crate::args::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
pub use crate::config::CliArgsConfig;
use crate::error::Utf16;
use crate::Error;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgsEnv {
    #[clap(long, env = "QUEP_PROVIDER")]
    pub provider: Option<ProviderType>,

    #[clap(long, env = "QUEP_OUTPUT")]
    pub output: Option<OutputType>,

    #[clap(long, env = "QUEP_OUTPUT_SER")]
    pub output_ser: Option<OutputSerType>,

    #[clap(long, env = "QUEP_CIRCUIT")]
    pub circuit: Option<CircuitType>,

    #[clap(long, env = "QUEP_CIRCUIT_RAND")]
    pub circuit_rand: Option<bool>,

    #[clap(long, env = "QUEP_ORCH")]
    pub orch: Option<OrchestratorType>,

    #[clap(long, env = "QUEP_ORCH_DATA", parse(try_from_os_str = parse_from_os_str))]
    pub orch_data: Option<String>,

    #[clap(long, env = "QUEP_ORCH_ITER")]
    pub orch_iter: Option<i32>,

    // TODO better default
    #[clap(long, env = "QUEP_ORCH_SIZE")] // default_value = "2147483647"
    pub orch_size: Option<i32>,

    // TODO make it better
    // This is to define width and depth separately in some orchestrators
    #[clap(long, env = "QUEP_ORCH_SIZE_2")]
    pub orch_size_2: Option<i32>,

    // TODO default_value does not work
    #[clap(long, env = "QUEP_PYTHON_DIR", parse(try_from_os_str = parse_from_os_str))]
    pub python_dir: Option<String>,

    // TODO just for testing only
    #[clap(long)] // default_value = "1"
    pub test_threads: Option<i32>,

    #[clap(short, takes_value = false)]
    pub q: Option<bool>,
}

#[throws]
fn parse_from_os_str(val: &OsStr) -> String {
    dunce::canonicalize(val)?.to_str().context(Utf16)?.to_owned()
}
