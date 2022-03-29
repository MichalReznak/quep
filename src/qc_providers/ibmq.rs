use async_trait::async_trait;
use derive_more::Constructor;
use tokio::time::Duration;

use crate::traits::QcProvider;
use crate::Error;

#[derive(Constructor)]
pub struct IbmqQcProvider;

#[async_trait]
impl QcProvider for IbmqQcProvider {
    async fn connect(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn run(&self, circuit: String) -> Result<String, Error> {
        Ok(circuit)
    }

    fn start_measure(&mut self) {
        unimplemented!()
    }

    fn stop_measure(&mut self) -> Duration {
        unimplemented!()
    }
}
