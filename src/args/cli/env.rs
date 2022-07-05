use std::collections::HashMap;
use std::ffi::OsStr;

use clap::Parser;
use fehler::throws;
use regex::Regex;
use snafu::OptionExt;
use unwrap_infallible::UnwrapInfallible;

use super::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use crate::args::types::{CircuitBenchType, LangSchemaType};
pub use crate::config::CliArgsConfig;
use crate::error::{Error, Utf16};

#[derive(Parser, Debug, Clone, Default)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgsEnv {
    #[clap(long, env = "QUEP_PROVIDER")]
    pub provider: Option<ProviderType>,

    #[clap(long, env = "QUEP_PROVIDER_PYTHON_DIR", parse(try_from_os_str = parse_from_os_str))]
    pub provider_python_dir: Option<String>,

    #[clap(long, env = "QUEP_PROVIDER_ACCOUNT_ID")]
    pub provider_account_id: Option<String>,

    #[clap(long, env = "QUEP_PROVIDER_PATH", parse(try_from_os_str = parse_from_os_str))]
    pub provider_path: Option<String>,

    #[clap(long, env = "QUEP_PROVIDER_MACHINE_NAME")]
    pub provider_machine_name: Option<String>,

    #[clap(long, env = "QUEP_OUTPUT")]
    pub output: Option<OutputType>,

    #[clap(long, env = "QUEP_OUTPUT_PATH", parse(try_from_os_str = parse_from_os_str))]
    pub output_path: Option<String>,

    #[clap(long, env = "QUEP_OUTPUT_SER")]
    pub output_ser: Option<OutputSerType>,

    #[clap(long, env = "QUEP_CIRCUIT_PRETTY")]
    pub output_pretty: Option<bool>,

    #[clap(long, env = "QUEP_CIRCUIT")]
    pub circuit: Option<CircuitType>,

    #[clap(long, env = "QUEP_CIRCUIT_PATH", parse(try_from_os_str = parse_from_os_str))]
    pub circuit_path: Option<String>,

    #[clap(long, env = "QUEP_CIRCUIT_BENCH")]
    pub circuit_bench: Option<CircuitBenchType>,

    #[clap(long, env = "QUEP_CIRCUIT_INIT_ONE")]
    pub circuit_init_one: Option<bool>,

    #[clap(long, env = "QUEP_CIRCUIT_RAND")]
    pub circuit_rand: Option<bool>,

    #[clap(long, env = "QUEP_CIRCUIT_PARSE")]
    pub circuit_parse: Option<bool>,

    #[clap(long, env = "QUEP_CIRCUIT_SOURCE")]
    pub circuit_source: Option<String>,

    #[clap(long, env = "QUEP_CIRCUIT_INVERSE_GATES", parse(try_from_str = parse_to_map))]
    pub circuit_inverse_gates: Option<HashMap<String, String>>,

    #[clap(long, env = "QUEP_LANG_SCHEMA")]
    pub lang_schema: Option<LangSchemaType>,

    #[clap(long, env = "QUEP_LANG_SCHEMA_PATH", parse(try_from_os_str = parse_from_os_str))]
    pub lang_schema_path: Option<String>,

    #[clap(long, env = "QUEP_ORCH")]
    pub orch: Option<OrchestratorType>,

    #[clap(long, env = "QUEP_ORCH_DATA", parse(try_from_os_str = parse_from_os_str))]
    pub orch_data: Option<String>,

    #[clap(long, env = "QUEP_ORCH_ITER")]
    pub orch_iter: Option<i32>,

    #[clap(long, env = "QUEP_ORCH_FROM_SIZE")]
    pub orch_from_size: Option<i32>,

    #[clap(long, env = "QUEP_ORCH_FROM_SIZE_2")]
    pub orch_from_size_2: Option<i32>,

    #[clap(long, env = "QUEP_ORCH_SIZE")]
    pub orch_size: Option<i32>,

    #[clap(long, env = "QUEP_ORCH_SIZE_2")]
    pub orch_size_2: Option<i32>,

    #[clap(long, env = "QUEP_ORCH_COLLECT")]
    pub orch_collect: Option<bool>,

    #[clap(long, env = "QUEP_ORCH_PREHEAT")]
    pub orch_preheat: Option<bool>,

    // just for testing only
    #[clap(long)]
    pub test_threads: Option<i32>,

    #[clap(short, takes_value = false)]
    pub q: Option<bool>,
}

#[throws]
fn parse_from_os_str(val: &OsStr) -> String {
    dunce::canonicalize(val)?.to_str().context(Utf16)?.to_owned()
}

#[throws]
fn parse_to_map(val: &str) -> HashMap<String, String> {
    Regex::new(r"(?P<key>[a-zA-Z0-9]+):\s*(?P<val>[a-zA-Z0-9]+);?\s*")?
        .captures_iter(val)
        .map(|c| {
            (
                c["key"].parse::<String>().unwrap_infallible(),
                c["val"].parse::<String>().unwrap_infallible(),
            )
        })
        .collect()
}
