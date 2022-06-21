use std::collections::HashMap;

use anyhow::{Context, Error};
use collection_literals::collection;
use fehler::throws;
use quep_core::args::types::{
    CircuitBenchType, CircuitSchemaType, CircuitType, OrchestratorType, OutputSerType, OutputType,
    ProviderType,
};
use quep_core::args::{CliArgsCircuit, CliArgsOrch, CliArgsOutput, CliArgsProvider};
use quep_core::CliArgs;
use typed_builder::TypedBuilder;
const ACCOUNT_ID: &str = "9ee04b444ed1c767fcd01b66027a391d8df5938df51dd27e6eaaed0a45f5da67c19dcfb2f2f46dcff893c3a54d054b4b392e1a54618d8cfea9d70d9f3378ea51";

#[derive(TypedBuilder)]
pub struct Config {
    pub orch: OrchestratorType,
    pub prov: ProviderType,
    pub out: OutputType,
    pub cir: CircuitType,
}

#[throws]
pub fn get_args(config: Config) -> CliArgs {
    CliArgs::builder()
        .orch(get_orch(config.orch)?)
        .provider(get_prov(config.prov)?)
        .output(get_out(config.out)?)
        .circuit(get_cir(config.cir)?)
        .build()
}

#[throws]
fn get_dir(s: &str) -> String {
    dunce::canonicalize(s)?.to_str().context("Cannot to string")?.to_owned()
}

#[throws]
fn get_orch(t: OrchestratorType) -> CliArgsOrch {
    CliArgsOrch::builder()
        .t(t)
        .size(4)
        .size_2(4)
        .iter(4)
        .data(get_dir("./data")?)
        .collect(false)
        .preheat(true)
        .mirror(true)
        .build()
}

#[throws]
fn get_prov(t: ProviderType) -> CliArgsProvider {
    CliArgsProvider::builder()
        .t(t)
        .python_dir(get_dir("./python")?)
        .account_id(ACCOUNT_ID.to_string())
        .build()
}

#[throws]
fn get_out(t: OutputType) -> CliArgsOutput {
    CliArgsOutput::builder().t(t).ser(OutputSerType::Json).pretty(true).build()
}

#[throws]
fn get_cir(t: CircuitType) -> CliArgsCircuit {
    CliArgsCircuit::builder()
        .t(t)
        .bench(CircuitBenchType::Mirror)
        .schema(CircuitSchemaType::OpenQasm)
        .rand(true)
        .parse(false)
        .source(get_dir("./base.template.qasm")?)
        .inverse_gates(collection! { HashMap<String, String>; })
        .build()
}
