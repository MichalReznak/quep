use std::ffi::OsStr;

use clap::Parser;
use fehler::throws;
use snafu::OptionExt;
use typed_builder::TypedBuilder;

use crate::args::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use crate::error::Utf16;
use crate::Error;

pub mod types;

#[derive(Parser, Debug, Clone, TypedBuilder)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(long, env = "QUEP_PROVIDER")]
    pub provider: ProviderType,

    #[clap(long, env = "QUEP_OUTPUT")]
    pub output: OutputType,

    #[clap(long, env = "QUEP_OUTPUT_SER")]
    pub output_ser: OutputSerType,

    #[clap(long, env = "QUEP_CIRCUIT")]
    pub circuit: CircuitType,

    #[clap(long, env = "QUEP_ORCH")]
    pub orch: OrchestratorType,

    #[clap(long, env = "QUEP_ORCH_DATA", parse(try_from_os_str = parse_from_os_str))]
    pub orch_data: String,

    // TODO better default
    #[clap(long, env = "QUEP_ORCH_SIZE", default_value = "2147483647")]
    pub orch_size: i32,

    // TODO make it better
    // This is to define width and depth separately in some orchestrators
    #[clap(long, env = "QUEP_ORCH_SIZE_2")]
    pub orch_size_2: i32,

    // TODO default_value does not work
    #[clap(long, env = "QUEP_PYTHON_DIR", parse(try_from_os_str = parse_from_os_str))]
    pub python_dir: String,
}

#[throws]
fn parse_from_os_str(val: &OsStr) -> String {
    dunce::canonicalize(val)?.to_str().context(Utf16)?.to_owned()
}
