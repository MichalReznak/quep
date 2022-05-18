use fehler::throws;
use serde::{Deserialize, Deserializer, Serialize};

use crate::args::types::{CircuitType, OrchestratorType, OutputSerType};
use crate::utils::dir;
use crate::{OutputType, ProviderType};

// TODO all should be option
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliArgsConfig {
    pub provider: Option<ProviderType>,

    #[serde(default)]
    pub circuit: CircuitConfig,

    #[serde(default)]
    pub orch: OrchConfig,

    #[serde(default)]
    pub output: OutputConfig,

    #[serde(default)]
    #[serde(deserialize_with = "parse_from_os_str")]
    pub python_dir: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CircuitConfig {
    pub t: Option<CircuitType>,
    pub rand: Option<bool>,
    // TODO add pretty flag
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OutputConfig {
    pub t: Option<OutputType>,
    pub ser: Option<OutputSerType>,
    // TODO add pretty flag
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrchConfig {
    pub t: Option<OrchestratorType>,
    pub size: Option<i32>,
    pub size_2: Option<i32>,
    pub iter: Option<i32>,

    #[serde(default)]
    #[serde(deserialize_with = "parse_from_os_str")]
    pub data: Option<String>,
}

#[throws(D::Error)]
fn parse_from_os_str<'de, D: Deserializer<'de>>(d: D) -> Option<String> {
    match String::deserialize(d) {
        Ok(buf) => Some(dir(&buf).unwrap()),
        Err(_) => None,
    }
}
