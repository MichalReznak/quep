use async_trait::async_trait;

use fehler::throws;
use num_traits::cast;
use serde::{Deserialize, Serialize};
use snafu::OptionExt;
use tokio::time::Duration;
use typed_builder::TypedBuilder;

use crate::args::types::OutputSerType;
use crate::args::CliArgsOutput;
use crate::error::OutOfBounds;
use crate::traits::outputer::Value;
use crate::traits::Outputer;
use crate::Error;

#[derive(Serialize, Deserialize, TypedBuilder, Debug)]
struct Output {
    records: Vec<Record>,
}

#[derive(Serialize, Deserialize, TypedBuilder, Debug)]
struct Record {
    width: i32,
    depth: i32,
    #[builder(setter(into))]
    output: String,
    #[builder(setter(into))]
    time_ms: i32, // TODO issue for toml, json5 (not implemented) should be u128
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
        duration: Vec<Duration>,
    ) -> Result<(), Error> {
        let mut table = Vec::new();

        for (i, value) in values.iter().enumerate() {
            for (j, col) in value.iter().enumerate() {
                let record = Record::builder()
                    .width(cast(i + 1).context(OutOfBounds)?)
                    .depth(cast(j + 1).context(OutOfBounds)?)
                    .output(&col.result)
                    .result(col.correct)
                    .correct((col.correct as f64) > 1024.0 * (2.0 / 3.0))
                    .time_ms(duration.get(j).context(OutOfBounds)?.as_millis() as i32)
                    .build();

                table.push(record);
            }
        }

        let table = Output::builder().records(table).build();
        let res = serialize(self.args.ser, &table)?;

        println!("\nResult:");
        println!("{res}");
        Ok(())
    }

    async fn output_volume(
        &self,
        values: Vec<Value>,
        duration: Vec<Duration>,
    ) -> Result<(), Error> {
        let len = values.len();
        let mut table = vec![];

        for (i, (val, dur)) in values.into_iter().zip(duration).enumerate() {
            let record = Record::builder()
                .width(cast(i + 1).context(OutOfBounds)?)
                .depth(cast(i + 1).context(OutOfBounds)?)
                .result(val.correct)
                .output(&val.result)
                .correct(val.correct as f64 > 1024.0 * (2.0 / 3.0))
                .time_ms(dur.as_millis() as i32)
                .build();
            table.push(record);
        }

        let table = Output::builder().records(table).build();
        let res = serialize(self.args.ser, &table)?;

        println!("\nResult:");
        println!("{res}");

        println!("\nQuantum Volume: {}", len);
        Ok(())
    }

    async fn output_linear(
        &self,
        values: Vec<Value>,
        duration: Vec<Duration>,
        width: i32,
    ) -> Result<(), Error> {
        let mut table = vec![];

        for (i, (val, dur)) in values.into_iter().zip(duration).enumerate() {
            let record = Record::builder()
                .width(cast(i + 1).context(OutOfBounds)?)
                .depth(cast(width).context(OutOfBounds)?)
                .result(val.correct)
                .output(&val.result)
                .correct(val.correct as f64 > 1024.0 * (2.0 / 3.0))
                .time_ms(dur.as_millis() as i32)
                .build();
            table.push(record);
        }

        let table = Output::builder().records(table).build();
        let res = serialize(self.args.ser, &table)?;

        println!("\nResult:");
        println!("{res}");
        Ok(())
    }
}
