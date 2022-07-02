use async_trait::async_trait;
use fehler::throws;
use num_traits::cast;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use snafu::OptionExt;
use tokio::time::Duration;
use typed_builder::TypedBuilder;

use crate::args::types::OutputSerType;
use crate::args::CliArgsOutput;
use crate::error::OutOfBounds;
use crate::ext::outputer::OutValue;
use crate::ext::Outputer;
use crate::Error;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, TypedBuilder, Debug)]
struct Output {
    records: Vec<Record>,
    #[builder(setter(into))]
    runtime_ms: i32,

    #[builder(default, setter(strip_option))]
    quantum_volume: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, TypedBuilder, Debug)]
struct Record {
    width: i32,
    depth: i32,
    #[builder(setter(into))]
    output: String,
    #[builder(setter(into))]
    time_ms: Option<i32>,
    result: i32,
    correct: bool,
}

pub struct SerialOutputer {
    args: CliArgsOutput,
}

impl SerialOutputer {
    #[throws]
    pub fn from_args(args: &CliArgsOutput) -> Self {
        Self { args: args.clone() }
    }
}

#[throws]
fn serialize(t: OutputSerType, out: &Output, pretty: bool) -> String {
    use OutputSerType::*;

    if pretty {
        match t {
            Json => serde_json::to_string_pretty(out)?,
            Xml => quick_xml::se::to_string(out)?,
            Yaml => serde_yaml::to_string(out)?,
            Toml => toml::to_string_pretty(out)?,
            Ron => ron::to_string(out)?,
        }
    }
    else {
        match t {
            Json => serde_json::to_string(out)?,
            Xml => quick_xml::se::to_string(out)?,
            Yaml => serde_yaml::to_string(out)?,
            Toml => toml::to_string(out)?,
            Ron => ron::to_string(out)?,
        }
    }
}

#[async_trait]
impl Outputer for SerialOutputer {
    async fn output_table(
        &self,
        values: Vec<Vec<OutValue>>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<Option<String>, Error> {
        let mut table = Vec::new();

        for (i, value) in values.iter().enumerate() {
            for (j, col) in value.iter().enumerate() {
                let time_ms = if let Some(durations) = &durations {
                    Some(durations.get(j).context(OutOfBounds)?.as_millis() as i32)
                }
                else {
                    None
                };

                let record = Record::builder()
                    .width(cast(i + 1).context(OutOfBounds)?)
                    .depth(cast(j + 1).context(OutOfBounds)?)
                    .output(&col.result)
                    .result(col.correct)
                    .correct(col.is_correct)
                    .time_ms(time_ms)
                    .build();

                table.push(record);
            }
        }

        let table = Output::builder().records(table).runtime_ms(runtime.as_millis() as i32).build();
        Ok(Some(serialize(self.args.ser, &table, self.args.pretty)?))
    }

    async fn output_volume(
        &self,
        values: Vec<OutValue>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<Option<String>, Error> {
        let include_durs = matches!(durations, Some(_));
        let durations = durations.unwrap_or_else(|| {
            vec![Duration::from_millis(0)].into_iter().cycle().take(values.len()).collect()
        });

        let len = values.len();
        let mut table = vec![];

        for (i, (val, dur)) in values.into_iter().zip(durations).enumerate() {
            let time_ms = if include_durs {
                Some(dur.as_millis() as i32)
            }
            else {
                None
            };

            let record = Record::builder()
                .width(cast(i + 1).context(OutOfBounds)?)
                .depth(cast(i + 1).context(OutOfBounds)?)
                .result(val.correct)
                .output(&val.result)
                .correct(val.is_correct)
                .time_ms(time_ms)
                .build();
            table.push(record);
        }

        let table = Output::builder()
            .records(table)
            .runtime_ms(runtime.as_millis() as i32)
            .quantum_volume(len.try_into()?)
            .build();
        Ok(Some(serialize(self.args.ser, &table, self.args.pretty)?))
    }

    async fn output_linear(
        &self,
        values: Vec<OutValue>,
        durations: Option<Vec<Duration>>,
        width: i32,
        runtime: Duration,
    ) -> Result<Option<String>, Error> {
        let include_durs = matches!(durations, Some(_));
        let durations = durations.unwrap_or_else(|| {
            vec![Duration::from_millis(0)].into_iter().cycle().take(values.len()).collect()
        });

        let mut table = vec![];

        for (i, (val, dur)) in values.into_iter().zip(durations).enumerate() {
            let time_ms = if include_durs {
                Some(dur.as_millis() as i32)
            }
            else {
                None
            };

            let record = Record::builder()
                .width(cast(i + 1).context(OutOfBounds)?)
                .depth(cast(width).context(OutOfBounds)?)
                .result(val.correct)
                .output(&val.result)
                .correct(val.is_correct)
                .time_ms(time_ms)
                .build();
            table.push(record);
        }

        let table = Output::builder().records(table).runtime_ms(runtime.as_millis() as i32).build();
        Ok(Some(serialize(self.args.ser, &table, self.args.pretty)?))
    }
}
