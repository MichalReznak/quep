use std::collections::HashMap;

use fehler::throws;
use serde::{Deserialize, Deserializer, Serialize};

use super::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use crate::args::types::{CircuitBenchType, LangSchemaType};
use crate::utils::dir;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CliArgsConfig {
    #[serde(default)]
    pub provider: CliArgsProviderConfig,

    #[serde(default)]
    pub circuit: CliArgsCircuitConfig,

    #[serde(default)]
    pub lang_schema: CliArgsLangSchemaConfig,

    #[serde(default)]
    pub orch: CliArgsOrchConfig,

    #[serde(default)]
    pub output: CliArgsOutputConfig,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CliArgsProviderConfig {
    pub t: Option<ProviderType>,

    #[serde(default)]
    #[serde(deserialize_with = "parse_from_os_str")]
    pub path: Option<String>,

    #[serde(default)]
    #[serde(deserialize_with = "parse_from_os_str")]
    pub python_dir: Option<String>,

    pub account_id: Option<String>,
    pub machine_name: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CliArgsCircuitConfig {
    pub t: Option<CircuitType>,

    #[serde(default)]
    #[serde(deserialize_with = "parse_from_os_str")]
    pub path: Option<String>,

    pub bench: Option<CircuitBenchType>,
    pub init_one: Option<bool>,
    pub rand: Option<bool>,
    pub source: Option<String>,
    pub gates: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CliArgsOutputConfig {
    pub t: Option<OutputType>,

    #[serde(default)]
    #[serde(deserialize_with = "parse_from_os_str")]
    pub path: Option<String>,

    pub ser: Option<OutputSerType>,
    pub pretty: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CliArgsLangSchemaConfig {
    pub t: Option<LangSchemaType>,

    #[serde(default)]
    #[serde(deserialize_with = "parse_from_os_str")]
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CliArgsOrchConfig {
    pub t: Option<OrchestratorType>,
    pub from_size: Option<i32>,
    pub from_size_2: Option<i32>,
    pub size: Option<i32>,
    pub size_2: Option<i32>,
    pub iter: Option<i32>,
    pub collect: Option<bool>,
    pub preheat: Option<bool>,

    #[serde(default)]
    #[serde(deserialize_with = "parse_from_os_str")]
    pub data: Option<String>,
}

#[throws(D::Error)]
fn parse_from_os_str<'de, D: Deserializer<'de>>(d: D) -> Option<String> {
    match String::deserialize(d) {
        Ok(buf) => Some(dir(&buf).unwrap()), // template error
        Err(_) => None,
    }
}
