//! Language schema Lexers
//!
//! Used for Quantum language parsing

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::LangGate;
use crate::lang_schemas::{LangCircuit, OpenQasmSchema, QiskitSchema};
use crate::Error;

#[enum_dispatch]
pub enum LangSchemaDyn {
    OpenQasmSchema,
    QiskitSchema,
}

#[async_trait]
#[enum_dispatch(LangSchemaDyn)]
pub trait LangSchema {
    fn get_gates(&self) -> Vec<LangGate>;

    async fn parse_file(&mut self, path: &str) -> Result<(), Error>;

    // TODO rename
    /// Outputs circuit as string
    async fn as_string(&mut self, circ: LangCircuit) -> Result<GenCircuit, Error>;
}
