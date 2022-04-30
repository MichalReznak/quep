use anyhow::Error;
use fehler::throws;
use quep::args::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use quep::CliArgs;

mod common;

// Orch: Volume
#[throws]
#[tokio::test]
async fn a() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn b() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn c() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Fs)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn d() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Fs)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn e() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Basic)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn f() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Basic)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn g() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn h() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn i() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::RandMirror)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn j() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::RandMirror)
        .orch(OrchestratorType::Volume)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

// Orch: Single
#[throws]
#[tokio::test]
async fn aa() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ab() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ac() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Fs)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ad() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Fs)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ae() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Basic)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn af() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Basic)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ag() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ah() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ai() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::RandMirror)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn aj() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::RandMirror)
        .orch(OrchestratorType::Single)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

// Orch: Lattice
#[throws]
#[tokio::test]
async fn ba() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn bb() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn bc() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Fs)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn bd() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Fs)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn be() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Basic)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn bf() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Basic)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn bg() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn bh() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn bi() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::RandMirror)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn bj() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::RandMirror)
        .orch(OrchestratorType::Lattice)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

// Orch: Linear
#[throws]
#[tokio::test]
async fn ca() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn cb() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn cc() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Fs)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn cd() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Fs)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ce() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Basic)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn cf() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Basic)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn cg() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ch() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::Mirror)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn ci() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Serial)
        .circuit(CircuitType::RandMirror)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn cj() {
    let args = CliArgs::builder()
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .output_ser(OutputSerType::Json)
        .provider(ProviderType::Qiskit)
        .output(OutputType::Text)
        .circuit(CircuitType::RandMirror)
        .orch(OrchestratorType::Linear)
        .build();
    quep::Quep::new(args).await?.run().await?;
}
