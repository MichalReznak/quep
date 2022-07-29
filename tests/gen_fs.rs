use anyhow::Error;
use fehler::throws;
use quep_core::args::types::{
    CircuitBenchType, CircuitType, LangSchemaType, OrchestratorType, OutputType, ProviderType,
};

use crate::common::Config;

mod common;

#[throws]
#[tokio::test]
async fn fs_a_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_one_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_one_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_one_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_one_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_one_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_one_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_serial_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_serial_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_serial_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_serial_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_serial_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_serial_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_one_serial_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_one_serial_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_one_serial_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_one_serial_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_one_serial_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_one_serial_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_collect_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_collect_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_one_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_one_collect_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_one_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_one_collect_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_one_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_one_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_one_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_one_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_one_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_one_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_one_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_one_serial_collect_linear() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Linear)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

//************************* Single *****************************//
#[throws]
#[tokio::test]
async fn fs_a_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

//***************** Init one *****************//
#[throws]
#[tokio::test]
async fn fs_a_one_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_one_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_one_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_one_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_one_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_one_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

//***************** Serial *****************//
#[throws]
#[tokio::test]
async fn fs_a_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_serial_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_serial_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_one_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_one_serial_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_one_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_one_serial_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_one_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_one_serial_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(false)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

//***************** Collect *****************//
#[throws]
#[tokio::test]
async fn fs_a_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_collect_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_collect_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_one_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_one_collect_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_one_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_one_collect_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_one_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_one_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Text)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(false)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_a_one_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_b_one_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Mirror)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_c_one_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_d_one_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Noisy)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::Cycle)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_e_one_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}

#[throws]
#[tokio::test]
async fn fs_f_one_serial_collect_single() {
    Config::builder()
        .prov(ProviderType::Simple)
        .cir(CircuitType::Fs)
        .cir_bench(CircuitBenchType::None)
        .cir_one(true)
        .out(OutputType::Serial)
        .ls(LangSchemaType::OpenQasm)
        .orch(OrchestratorType::Single)
        .orch_collect(true)
        .orch_size2(1)
        .build()
        .run()
        .await?;
}
