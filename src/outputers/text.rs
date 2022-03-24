use async_trait::async_trait;
use derive_more::Constructor;
use tokio::time::Duration;

use crate::traits::Outputer;

#[derive(Constructor)]
pub struct TextOutputer;

#[async_trait]
impl Outputer for TextOutputer {
    async fn output(&self, value: String, duration: Duration) -> Result<(), crate::Error> {
        let duration = duration.as_millis();
        println!("{value}");
        println!("\nRuntime: {duration} ns");
        Ok(())
    }
}
