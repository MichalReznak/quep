use std::collections::HashMap;

use anyhow::{Context, Error};
use collection_literals::collection;
use fehler::throws;
use quep_core::args::types::{
    CircuitBenchType, CircuitType, LangSchemaType, OrchestratorType, OutputSerType, OutputType,
    ProviderType,
};
use quep_core::args::{
    CliArgsCircuit, CliArgsLangSchema, CliArgsOrch, CliArgsOutput, CliArgsProvider,
};
use quep_core::{CliArgs, Quep};
use typed_builder::TypedBuilder;


const ACCOUNT_ID: &str = "9ee04b444ed1c767fcd01b66027a391d8df5938df51dd27e6eaaed0a45f5da67c19dcfb2f2f46dcff893c3a54d054b4b392e1a54618d8cfea9d70d9f3378ea51";
const MACHINE_NAME: &str = "ibmq_quito";

#[throws]
fn get_dir(s: &str) -> String {
    dunce::canonicalize(s)?.to_str().context("Cannot to string")?.to_owned()
}

#[derive(TypedBuilder)]
pub struct Config {
    pub prov: ProviderType,

    pub cir: CircuitType,
    pub cir_bench: CircuitBenchType,
    #[builder(default, setter(strip_option))]
    pub cir_one: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub cir_rand: Option<bool>,

    pub out: OutputType,

    pub ls: LangSchemaType,

    pub orch: OrchestratorType,
    #[builder(default, setter(strip_option))]
    pub orch_from: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub orch_from2: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub orch_size: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub orch_size2: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub orch_collect: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub orch_preheat: Option<bool>,
}

impl Config {
    #[throws]
    pub async fn run(self) {
        let c = self;

        let prov = CliArgsProvider::builder()
            .t(c.prov)
            .python_dir(get_dir(".")?)
            .path(get_dir("./python_ext/qc_provider.py")?)
            .account_id(ACCOUNT_ID.to_string())
            .machine_name(MACHINE_NAME.to_string())
            .build();

        let circ = CliArgsCircuit::builder()
            .t(c.cir)
            .bench(c.cir_bench)
            .path(get_dir("./python_ext/circuit_generator.py")?)
            .init_one(c.cir_one.unwrap_or(false))
            .rand(c.cir_rand.unwrap_or(true))
            .source(get_dir("./templates/example.qasm")?)
            .gates(collection! { HashMap<String, String>; })
            .build();

        let out = CliArgsOutput::builder()
            .t(c.out)
            .ser(OutputSerType::Json)
            .pretty(true)
            .path(get_dir("./python_ext/outputer.py")?)
            .build();

        let ls = CliArgsLangSchema::builder()
            .t(c.ls)
            .path(get_dir("./python_ext/lang_schema.py")?)
            .build();

        let orch = CliArgsOrch::builder()
            .t(c.orch)
            .from_size(c.orch_from.unwrap_or(1))
            .from_size_2(c.orch_from2.unwrap_or(1))
            .size(c.orch_size.unwrap_or(4))
            .size_2(c.orch_size2.unwrap_or(4))
            .iter(4)
            .data(get_dir("./data")?)
            .collect(c.orch_collect.unwrap_or(false))
            .preheat(c.orch_preheat.unwrap_or(false))
            .build();

        let args = CliArgs::builder()
            .provider(prov)
            .circuit(circ)
            .output(out)
            .lang_schema(ls)
            .orch(orch)
            .build();

        Quep::new(args).await?.run().await?;
    }
}
