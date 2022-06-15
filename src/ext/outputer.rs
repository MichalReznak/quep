//! Outputers
//!
//! Outputs results in any way supported
//! There will be some text based for sure

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use tokio::time::Duration;
use typed_builder::TypedBuilder;

use crate::outputers::{SerialOutputer, TextOutputer};
use crate::Error;

// TODO rename
#[derive(Debug, Clone, TypedBuilder)]
pub struct Value {
    pub result: String,   // result bit-string
    pub correct: i32,     // number of correct shots
    pub is_correct: bool, // is correct?
}

#[enum_dispatch]
pub enum OutputerDyn {
    TextOutputer,
    SerialOutputer,
}

#[async_trait]
#[enum_dispatch(OutputerDyn)]
pub trait Outputer {
    /// Create a lattice or results
    async fn output_table(
        &self,
        value: Vec<Vec<Value>>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<String, Error>;

    /// Output with increasing width and depth always by one
    async fn output_volume(
        &self,
        values: Vec<Value>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<String, Error>;

    /// Output as a linear list of values
    async fn output_linear(
        &self,
        values: Vec<Value>,
        durations: Option<Vec<Duration>>,
        width: i32,
        runtime: Duration,
    ) -> Result<String, Error>;
}
