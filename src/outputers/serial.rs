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
use crate::traits::outputer::Value;
use crate::traits::Outputer;
use crate::Error;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, TypedBuilder, Debug)]
struct Output {
    records: Vec<Record>,
    #[builder(setter(into))]
    runtime_ms: i32,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, TypedBuilder, Debug)]
struct Record {
    width: i32,
    depth: i32,
    #[builder(setter(into))]
    output: String,
    #[builder(setter(into))]
    time_ms: Option<i32>, // TODO issue for toml, json5 (not implemented) should be u128
    result: i32,
    correct: bool,
}

pub struct SerialOutputer {
    args: CliArgsOutput,
}

impl SerialOutputer {
    pub fn new(args: &CliArgsOutput) -> Self {
        Self { args: args.clone() }
    }
}

// TODO can be any serialized format

#[throws]
fn serialize(t: OutputSerType, out: &Output) -> String {
    use OutputSerType::*;
    match t {
        Json => serde_json::to_string_pretty(out)?,
        Xml => quick_xml::se::to_string(out)?,
        Yaml => serde_yaml::to_string(out)?,
        Toml => toml::to_string_pretty(out)?,
        Ron => ron::to_string(out)?,
    }
}

#[async_trait]
impl Outputer for SerialOutputer {
    async fn output_table(
        &self,
        values: Vec<Vec<Value>>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<(), Error> {
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
                    .correct((col.correct as f64) > 1024.0 * (2.0 / 3.0))
                    .time_ms(time_ms)
                    .build();

                table.push(record);
            }
        }

        let table = Output::builder().records(table).runtime_ms(runtime.as_millis() as i32).build();
        let res = serialize(self.args.ser, &table)?;

        println!("\nResult:");
        println!("{res}");
        Ok(())
    }

    async fn output_volume(
        &self,
        values: Vec<Value>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<(), Error> {
        let include_durs = matches!(durations, Some(_));
        // TODO this hack is not pretty
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
                .correct(val.correct as f64 > 1024.0 * (2.0 / 3.0))
                .time_ms(time_ms)
                .build();
            table.push(record);
        }

        let table = Output::builder().records(table).runtime_ms(runtime.as_millis() as i32).build();
        let res = serialize(self.args.ser, &table)?;

        println!("\nResult:");
        println!("{res}");

        println!("\nQuantum Volume: {}", len);
        Ok(())
    }

    async fn output_linear(
        &self,
        values: Vec<Value>,
        durations: Option<Vec<Duration>>,
        width: i32,
        runtime: Duration,
    ) -> Result<(), Error> {
        let include_durs = matches!(durations, Some(_));
        // TODO this hack is not pretty
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
                .correct(val.correct as f64 > 1024.0 * (2.0 / 3.0))
                .time_ms(time_ms)
                .build();
            table.push(record);
        }

        let table = Output::builder().records(table).runtime_ms(runtime.as_millis() as i32).build();
        let res = serialize(self.args.ser, &table)?;

        println!("\nResult:");
        println!("{res}");
        Ok(())
    }
}
