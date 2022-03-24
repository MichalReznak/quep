use async_trait::async_trait;
use derive_more::Constructor;

use crate::traits::CircuitGenerator;
use crate::Error;

#[derive(Constructor)]
pub struct BasicCircuitGenerator;

#[async_trait]
impl CircuitGenerator for BasicCircuitGenerator {
    async fn generate(&self) -> Result<String, Error> {
        Ok("generated".to_string())
    }
}
