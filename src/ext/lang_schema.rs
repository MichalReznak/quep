//! Language schema Lexers
//!
//! Used for Quantum language parsing

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::ext::types::circuit_generator::GenCircuit;
use crate::ext::types::lang_schema::ParsedLangCircuit;
use crate::lang_schemas::{LangCircuit, OpenQasmSchema, PythonSchema, QiskitSchema};
use crate::{CliArgs, Error};

#[enum_dispatch]
pub enum LangSchemaDyn {
    OpenQasmSchema,
    QiskitSchema,
    PythonSchema,
}

#[async_trait]
#[enum_dispatch(LangSchemaDyn)]
pub trait LangSchema {
    /// Check whether arguments are correct
    fn check_constraints(&self, _args: &CliArgs) -> Result<(), Error> {
        Ok(())
    }

    /// Parse file from the fs path
    fn parse_file(&self, path: &str) -> Result<ParsedLangCircuit, Error>;

    /// Outputs circuit as string
    fn as_string(&mut self, circ: LangCircuit) -> Result<GenCircuit, Error>;
}
