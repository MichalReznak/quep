use anyhow::Error;
use fehler::throws;
use quep_core::args::types::{CircuitType, OrchestratorType, OutputType, ProviderType};
use quep_core::Quep;

use crate::common::{get_args, Config};

mod common;

#[throws]
#[tokio::test]
async fn lattice_a() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Lattice)
            .out(OutputType::Serial)
            .prov(ProviderType::Noisy)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn lattice_b() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Lattice)
            .out(OutputType::Text)
            .prov(ProviderType::Noisy)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn lattice_c() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Lattice)
            .out(OutputType::Serial)
            .prov(ProviderType::Simple)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn lattice_d() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Lattice)
            .out(OutputType::Text)
            .prov(ProviderType::Simple)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn single_a() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Single)
            .out(OutputType::Serial)
            .prov(ProviderType::Noisy)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn single_b() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Single)
            .out(OutputType::Text)
            .prov(ProviderType::Noisy)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn single_c() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Single)
            .out(OutputType::Serial)
            .prov(ProviderType::Simple)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn single_d() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Single)
            .out(OutputType::Text)
            .prov(ProviderType::Simple)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn linear_a() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Linear)
            .out(OutputType::Serial)
            .prov(ProviderType::Noisy)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn linear_b() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Linear)
            .out(OutputType::Text)
            .prov(ProviderType::Noisy)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn linear_c() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Linear)
            .out(OutputType::Serial)
            .prov(ProviderType::Simple)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}

#[throws]
#[tokio::test]
async fn linear_d() {
    let args = get_args(
        Config::builder()
            .cir(CircuitType::Rand)
            .orch(OrchestratorType::Linear)
            .out(OutputType::Text)
            .prov(ProviderType::Simple)
            .build(),
    )?;
    Quep::new(args).await?.run().await?;
}
