//! Language schema Lexers
//!
//! Used for Quantum language parsing

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::lang_schemas::{LangCircuit, OpenQasmSchema, QiskitSchema};
use crate::Error;
use crate::ext::types::circuit_generator::GenCircuit;

#[enum_dispatch]
pub enum LangSchemaDyn {
    OpenQasmSchema,
    QiskitSchema,
}

#[async_trait]
#[enum_dispatch(LangSchemaDyn)]
pub trait LangSchema {
    // TODO rename
    /// Outputs circuit as string
    async fn as_string(&mut self, circ: LangCircuit) -> Result<GenCircuit, Error>;
}
