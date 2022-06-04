//! Language schema Lexers
//!
//! Used for Quantum language parsing

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::lang_schemas::{LangCircuit, OpenQasmSchema};
use crate::Error;

#[enum_dispatch]
pub enum LangSchemaDyn {
    OpenQasmSchema,
}

#[async_trait]
#[enum_dispatch(LangSchemaDyn)]
pub trait LangSchema {
    // TODO rename
    /// Outputs circuit as string
    async fn as_string(&mut self, circ: LangCircuit) -> Result<String, Error>;
}
