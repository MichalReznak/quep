use async_trait::async_trait;
use derive_more::Constructor;

use crate::traits::Outputer;

#[derive(Constructor)]
pub struct TextOutputer;

#[async_trait]
impl Outputer for TextOutputer {
    async fn output(&self, value: String) -> Result<(), crate::Error> {
        println!("{value}");
        Ok(())
    }
}
