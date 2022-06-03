use async_trait::async_trait;

use crate::ext::LangSchema;
use crate::Error;

#[allow(dead_code)]
pub struct OpenQasmSchema {}

#[allow(dead_code)]
impl OpenQasmSchema {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl LangSchema for OpenQasmSchema {
    async fn parse(&mut self, _file: String) -> Result<(), Error> {
        todo!()
    }
}
