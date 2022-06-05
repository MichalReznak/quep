use std::collections::HashMap;

use fehler::throws;
use serde::{Deserialize, Deserializer, Serialize};

use super::types::{CircuitType, OrchestratorType, OutputSerType, OutputType, ProviderType};
use crate::args::types::CircuitSchemaType;
use crate::utils::dir;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CliArgsConfig {
    #[serde(default)]
    pub provider: ProviderConfig,

    #[serde(default)]
    pub circuit: CircuitConfig,

    #[serde(default)]
    pub orch: OrchConfig,

    #[serde(default)]
    pub output: OutputConfig,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfig {
    pub t: Option<ProviderType>,

    #[serde(default)]
    #[serde(deserialize_with = "parse_from_os_str")]
    pub python_dir: Option<String>,

    pub account_id: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CircuitConfig {
    pub t: Option<CircuitType>,
    pub schema: Option<CircuitSchemaType>,
    pub rand: Option<bool>,
    pub parse: Option<bool>,
    pub source: Option<String>,
    pub inverse_gates: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OutputConfig {
    pub t: Option<OutputType>,
    pub ser: Option<OutputSerType>,
    pub pretty: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrchConfig {
    pub t: Option<OrchestratorType>,
    pub size: Option<i32>,
    pub size_2: Option<i32>,
    pub iter: Option<i32>,
    pub collect: Option<bool>,

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
