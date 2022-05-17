use async_trait::async_trait;
use derive_more::Constructor;
use fehler::throws;
use num_traits::cast;
use regex::Regex;
use serde::{Deserialize, Serialize};
use snafu::OptionExt;
use tokio::time::Duration;
use typed_builder::TypedBuilder;
use unwrap_infallible::UnwrapInfallible;

use crate::args::types::OutputSerType;
use crate::error::{OutOfBounds, RegexCapture};
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
    time_ms: u128, // TODO issue for toml (not implemented)
    result: i32,
    correct: bool,
}

#[derive(Constructor)]
pub struct SerialOutputer {
    out: OutputSerType,
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
        unimplemented!();

        // let mut table = Vec::new();
        // let re = Regex::new(r"(?P<out>\d+): (?P<val>\d+)")?;
        //
        // for (i, value) in values.iter().enumerate() {
        //     for (j, col) in value.iter().enumerate() {
        //         let c = re.captures(&col).context(RegexCapture)?;
        //         let val = c["val"].parse::<f64>()?;
        //         let out = c["out"].parse::<String>().unwrap_infallible();
        //
        //         let record = Record::builder()
        //             .width(cast(i + 1).context(OutOfBounds)?)
        //             .depth(cast(j + 1).context(OutOfBounds)?)
        //             .output(&out)
        //             .result(cast(val).context(OutOfBounds)?)
        //             .correct(val > 1024.0 * (2.0 / 3.0))
        //             .time_ms(duration.get(j).context(OutOfBounds)?.as_millis())
        //             .build();
        //
        //         table.push(record);
        //     }
        // }
        //
        // let table = Output::builder().records(table).build();
        // let res = serialize(self.out, &table)?;
        //
        // println!("\nResult:");
        // println!("{res}");
        // Ok(())
    }

    async fn output_volume(
        &self,
        values: Vec<String>,
        duration: Vec<Duration>,
    ) -> Result<(), Error> {
        unimplemented!();
        // let len = values.len();
        // let mut table = vec![];
        // let re = Regex::new(r"(?P<out>\d+): (?P<val>\d+)")?;
        //
        // for (i, (val, dur)) in values.into_iter().zip(duration).enumerate() {
        //     let c = re.captures(&val).context(RegexCapture)?;
        //     let val = c["val"].parse::<f64>()?;
        //     let out = c["out"].parse::<String>().unwrap_infallible();
        //
        //     let record = Record::builder()
        //         .width(cast(i + 1).context(OutOfBounds)?)
        //         .depth(cast(i + 1).context(OutOfBounds)?)
        //         .result(cast(val).context(OutOfBounds)?)
        //         .output(&out)
        //         .correct(val > 1024.0 * (2.0 / 3.0))
        //         .time_ms(dur.as_millis())
        //         .build();
        //     table.push(record);
        // }
        //
        // let table = Output::builder().records(table).build();
        // let res = serialize(self.out, &table)?;
        //
        // println!("\nResult:");
        // println!("{res}");
        //
        // println!("\nQuantum Volume: {}", len);
        // Ok(())
    }

    async fn output_linear(
        &self,
        values: Vec<String>,
        duration: Vec<Duration>,
        width: i32,
    ) -> Result<(), Error> {
        unimplemented!();

        // let mut table = vec![];
        // let re = Regex::new(r"(?P<out>\d+): (?P<val>\d+)")?;
        //
        // for (i, (val, dur)) in values.into_iter().zip(duration).enumerate() {
        //     let c = re.captures(&val).context(RegexCapture)?;
        //     let val = c["val"].parse::<f64>()?;
        //     let out = c["out"].parse::<String>().unwrap_infallible();
        //
        //     let record = Record::builder()
        //         .width(cast(i + 1).context(OutOfBounds)?)
        //         .depth(cast(width).context(OutOfBounds)?)
        //         .result(cast(val).context(OutOfBounds)?)
        //         .output(&out)
        //         .correct(val > 1024.0 * (2.0 / 3.0))
        //         .time_ms(dur.as_millis())
        //         .build();
        //     table.push(record);
        // }
        //
        // let table = Output::builder().records(table).build();
        // let res = serialize(self.out, &table)?;
        //
        // println!("\nResult:");
        // println!("{res}");
        // Ok(())
    }
}
