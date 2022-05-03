//! Outputers
//!
//! Outputs results in any way supported
//! There will be some text based for sure

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use tokio::time::Duration;

use crate::outputers::{SerialOutputer, TextOutputer};
use crate::Error;

#[enum_dispatch]
pub enum OutputerDyn {
    TextOutputer,
    SerialOutputer,
}

#[async_trait]
#[enum_dispatch(OutputerDyn)]
pub trait Outputer {
    async fn output_table(
        &self,
        value: Vec<Vec<String>>,
        duration: Vec<Duration>,
    ) -> Result<(), Error>;

    async fn output_volume(
        &self,
        values: Vec<String>,
        duration: Vec<Duration>,
    ) -> Result<(), Error>;

    async fn output_linear(
        &self,
        values: Vec<String>,
        duration: Vec<Duration>,
        width: i32,
    ) -> Result<(), Error>;
}
