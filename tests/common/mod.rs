use anyhow::{Context, Error};
use app::args::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use app::args::{CliArgsCircuit, CliArgsOrch, CliArgsOutput, CliArgsProvider};
use app::CliArgs;
use fehler::throws;
use typed_builder::TypedBuilder;

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
        .build()
}

#[throws]
fn get_prov(t: ProviderType) -> CliArgsProvider {
    CliArgsProvider::builder().t(t).python_dir(get_dir("./python")?).build()
}

#[throws]
fn get_out(t: OutputType) -> CliArgsOutput {
    CliArgsOutput::builder().t(t).ser(OutputSerType::Json).build()
}

#[throws]
fn get_cir(t: CircuitType) -> CliArgsCircuit {
    CliArgsCircuit::builder()
        .t(t)
        .rand(true)
        .parse(false)
        .source(get_dir("./base.template.qasm")?)
        .build()
}
