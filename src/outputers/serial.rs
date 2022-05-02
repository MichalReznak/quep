use async_trait::async_trait;
use derive_more::Constructor;
use num_traits::cast;
use regex::Regex;
use serde::{Deserialize, Serialize};
use snafu::OptionExt;
use tokio::time::Duration;
use typed_builder::TypedBuilder;
use unwrap_infallible::UnwrapInfallible;

use crate::args::types::OutputSerType;
use crate::error::{OutOfBounds, RegexCapture};
use crate::traits::Outputer;

#[derive(Serialize, Deserialize, TypedBuilder, Debug)]
struct Output {
    records: Vec<Record>, // TODO linearize or not?
}

#[derive(Serialize, Deserialize, TypedBuilder, Debug)]
struct Record {
    width: i32,
    depth: i32,
    #[builder(setter(into))]
    output: String,
    #[builder(setter(into))]
    time_ms: u128,
    result: i32,
    correct: bool,
}

#[derive(Constructor)]
pub struct SerialOutputer {
    out: OutputSerType,
}

// TODO can be any serialized format

#[async_trait]
impl Outputer for SerialOutputer {
    async fn output(
        &self,
        values: Vec<Vec<String>>,
        duration: Vec<Duration>,
    ) -> Result<(), crate::Error> {
        let mut table = Vec::new();
        let re = Regex::new(r"(?P<out>\d+): (?P<val>\d+)")?;

        for (i, value) in values.iter().enumerate() {
            for (j, col) in value.iter().enumerate() {
                let c = re.captures(&col).context(RegexCapture)?;
                let val = c["val"].parse::<f64>()?;

                let record = Record::builder()
                    .width(cast(i + 1).context(OutOfBounds)?)
                    .depth(cast(j + 1).context(OutOfBounds)?)
                    .output(&c["out"].parse::<String>().unwrap_infallible())
                    .result(cast(val).context(OutOfBounds)?)
                    .correct(val > 1024.0 * (2.0 / 3.0))
                    .time_ms(duration.get(j).context(OutOfBounds)?.as_millis())
                    .build();

                table.push(record);
            }
        }

        let table = Output::builder().records(table).build();

        use OutputSerType::*;
        let res = match self.out {
            Json => serde_json::to_string(&table)?,
            Xml => quick_xml::se::to_string(&table)?,
            Yaml => serde_yaml::to_string(&table)?,
            Toml => toml::to_string(&table)?,
            Ron => ron::to_string(&table)?,
        };

        println!("{res}");
        Ok(())
    }
}
