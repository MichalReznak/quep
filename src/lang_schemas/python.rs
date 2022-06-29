use async_trait::async_trait;

use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::LangGate;
use crate::ext::LangSchema;
use crate::lang_schemas::LangCircuit;
use crate::Error;

#[derive(Default)]
pub struct PythonSchema {
    pub gates: Vec<LangGate>,
}

impl PythonSchema {
    pub fn new() -> Self {
        Self { gates: vec![] }
    }
}

#[async_trait]
impl LangSchema for PythonSchema {
    async fn parse_file(&self, _path: &str) -> Result<Vec<LangGate>, Error> {
        unimplemented!();
    }

    async fn as_string(&mut self, _circ: LangCircuit) -> Result<GenCircuit, Error> {
        unimplemented!();
    }
}
