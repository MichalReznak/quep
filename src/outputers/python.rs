//! Add interface description

use async_trait::async_trait;
use tokio::time::Duration;

use crate::args::CliArgsOutput;
use crate::ext::outputer::OutValue;
use crate::ext::Outputer;
use crate::Error;

pub struct PythonOutputer {
    // args: CliArgsOutput,
}

impl PythonOutputer {
    pub fn new(_args: &CliArgsOutput) -> Self {
        // args: args.clone()
        Self {}
    }
}

#[async_trait]
impl Outputer for PythonOutputer {
    async fn output_table(
        &self,
        _values: Vec<Vec<OutValue>>,
        _durations: Option<Vec<Duration>>,
        _runtime: Duration,
    ) -> Result<String, Error> {
        todo!();
    }

    async fn output_volume(
        &self,
        _values: Vec<OutValue>,
        _durations: Option<Vec<Duration>>,
        _runtime: Duration,
    ) -> Result<String, Error> {
        todo!();
    }

    async fn output_linear(
        &self,
        _values: Vec<OutValue>,
        _durations: Option<Vec<Duration>>,
        _width: i32,
        _runtime: Duration,
    ) -> Result<String, Error> {
        todo!();
    }
}
