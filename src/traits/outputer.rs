//! Outputers
//!
//! Outputs results in any way supported
//! There will be some text based for sure

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use tokio::time::Duration;

use crate::outputers::TextOutputer;
use crate::Error;

#[enum_dispatch]
pub enum OutputerDyn {
    TextOutputer,
}

#[async_trait]
#[enum_dispatch(OutputerDyn)]
pub trait Outputer {
    async fn output(&self, value: String, duration: Duration) -> Result<(), Error>;
}
