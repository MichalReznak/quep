//! Language schema Lexers
//!
//! Used for Quantum language parsing

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::LangGate;
use crate::lang_schemas::{LangCircuit, OpenQasmSchema, QiskitSchema};
use crate::{CliArgs, Error};

#[enum_dispatch]
pub enum LangSchemaDyn {
    OpenQasmSchema,
    QiskitSchema,
}

#[async_trait]
#[enum_dispatch(LangSchemaDyn)]
pub trait LangSchema {
    /// Check whether arguments are correct
    fn check_constraints(&self, _args: &CliArgs) -> Result<(), Error> {
        Ok(())
    }

    /// Returns list of gates from the source file
    fn get_gates(&self) -> Vec<LangGate>;

    /// Parse file from the fs path
    async fn parse_file(&mut self, path: &str) -> Result<(), Error>;

    /// Outputs circuit as string
    async fn as_string(&mut self, circ: LangCircuit) -> Result<GenCircuit, Error>;
}
