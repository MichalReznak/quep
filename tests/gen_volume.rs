use anyhow::Error;
use fehler::throws;
use quep_core::args::types::{
    CircuitBenchType, CircuitType, LangSchemaType, OrchestratorType, OutputType, ProviderType,
};

use crate::common::Config;

mod common;

#[throws]
#[tokio::test]
async fn volume_a() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_serial() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_serial() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_serial() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_serial() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_serial() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_serial() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_qiskit() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_qiskit() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_qiskit() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_qiskit() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_qiskit() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_qiskit() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_serial_qiskit() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_serial_qiskit() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_serial_qiskit() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_serial_qiskit() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_serial_qiskit() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_serial_qiskit() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_serial_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_serial_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_serial_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_serial_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_serial_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_serial_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_serial_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_serial_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_serial_qiskit_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_serial_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_serial_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_serial_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_serial_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_serial_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_serial_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_serial_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_serial_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_serial_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_serial_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_serial_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_serial_qiskit_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Volume)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_serial_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_serial_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_serial_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_serial_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_serial_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_serial_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_a_serial_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_b_serial_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_c_serial_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_d_serial_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_e_serial_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn volume_f_serial_qiskit_single_collect() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Volume)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::Qiskit)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .build()
        .run()
        .await?;
}
