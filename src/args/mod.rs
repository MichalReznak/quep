use std::ffi::OsStr;

use clap::Parser;
use fehler::throws;
use lazy_static::lazy_static;
use snafu::OptionExt;

use crate::args::types::{CircuitType, OutputSerType, OutputType, ProviderType};
use crate::error::Utf16;
use crate::Error;

pub mod types;

#[derive(Parser, Debug)]
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

    #[clap(long, env = "QUEP_SIZE")]
    pub size: i32,

    // TODO default_value does not work
    #[clap(long, env = "QUEP_PYTHON_DIR", parse(try_from_os_str = parse_python_dir))]
    pub python_dir: String,
}

#[throws]
fn parse_python_dir(val: &OsStr) -> String {
    dunce::canonicalize(val)?.to_str().context(Utf16)?.to_owned()
}

lazy_static! {
    pub static ref ARGS: CliArgs = {
        dotenv::dotenv().ok();
        CliArgs::parse()
    };
}
