use anyhow::Error;
use fehler::throws;
use quep::args::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use quep::CliArgs;

mod common;

#[throws]
#[tokio::test]
async fn mirror() {
    let args = CliArgs::builder()
        .provider(ProviderType::Noisy)
        .output(OutputType::Text)
        .output_ser(OutputSerType::Json)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Single)
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn mirror_serial_json() {
    let args = CliArgs::builder()
        .provider(ProviderType::Noisy)
        .output(OutputType::Serial)
        .output_ser(OutputSerType::Json)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Single)
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

// TODO all serials

#[throws]
#[tokio::test]
async fn rand_mirror() {
    let args = CliArgs::builder()
        .provider(ProviderType::Noisy)
        .output(OutputType::Text)
        .output_ser(OutputSerType::Json)
        .circuit(CircuitType::RandMirror)
        .orch(OrchestratorType::Single)
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn fs() {
    let args = CliArgs::builder()
        .provider(ProviderType::Noisy)
        .output(OutputType::Text)
        .output_ser(OutputSerType::Json)
        .circuit(CircuitType::Fs)
        .orch(OrchestratorType::Single)
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn basic() {
    let args = CliArgs::builder()
        .provider(ProviderType::Noisy)
        .output(OutputType::Text)
        .output_ser(OutputSerType::Json)
        .circuit(CircuitType::Basic)
        .orch(OrchestratorType::Single)
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn volume() {
    let args = CliArgs::builder()
        .provider(ProviderType::Noisy)
        .output(OutputType::Text)
        .output_ser(OutputSerType::Json)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Single)
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .build();
    quep::Quep::new(args).await?.run().await?;
}
