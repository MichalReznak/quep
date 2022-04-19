use std::fs;
use std::path::{Path, PathBuf};
use clap::Parser;
use lazy_static::lazy_static;
use crate::args::types::{CircuitType, OutputType, ProviderType};

pub mod types;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(long, env = "QUEP_PROVIDER")]
    pub provider: ProviderType,

    #[clap(long, env = "QUEP_OUTPUT")]
    pub output: OutputType,

    #[clap(long, env = "QUEP_CIRCUIT")]
    pub circuit: CircuitType,

    #[clap(long, env = "QUEP_SIZE")]
    pub size: i32,

    #[clap(long, env = "QUEP_PYTHON_DIR", parse(try_from_str = parse_python_dir), default_value = "./python")]
    pub python_dir: String,
}

// TODO use try_from_os_str
fn parse_python_dir(val: &str) -> Result<String, String> {
    Ok(dunce::canonicalize(val).unwrap().to_str().unwrap().to_owned())
}

lazy_static! {
    pub static ref ARGS: CliArgs = {
        dotenv::dotenv().ok();
        CliArgs::parse()
    };
}
