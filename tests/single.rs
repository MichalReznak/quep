use anyhow::Error;
use fehler::throws;
use quep::args::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use quep::CliArgs;

mod common;

#[throws]
#[tokio::test]
async fn a0() {
    let args = CliArgs::builder()
        .provider(ProviderType::Noisy)
        .output(OutputType::Text)
        .output_ser(OutputSerType::Json)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Lattice)
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .build();
    quep::Quep::with_args(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn a1() {
    let args = CliArgs::builder()
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .output_ser(OutputSerType::Json)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Lattice)
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .build();
    quep::Quep::with_args(args).await?.run().await?;
}
