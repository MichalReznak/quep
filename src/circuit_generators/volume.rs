use async_trait::async_trait;
use derive_more::Constructor;

use crate::traits::CircuitGenerator;
use crate::Error;

#[derive(Constructor)]
pub struct VolumeCircuitGenerator;

#[async_trait]
impl CircuitGenerator for VolumeCircuitGenerator {
    async fn generate(&self, _i: i32) -> Result<Option<String>, Error> {
        Ok(None)
    }
}
