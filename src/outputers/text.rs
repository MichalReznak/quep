use async_trait::async_trait;
use cli_table::format::Justify;
use cli_table::{print_stdout, Cell, Style, Table};
use derive_more::Constructor;
use regex::Regex;
use snafu::OptionExt;
use termcolor::Color;
use tokio::time::Duration;

use crate::error::{OutOfBounds, RegexCapture};
use crate::traits::Outputer;

#[derive(Constructor)]
pub struct TextOutputer;

#[async_trait]
impl Outputer for TextOutputer {
    async fn output(
        &self,
        values: Vec<Vec<String>>,
        duration: Vec<Duration>,
    ) -> Result<(), crate::Error> {
        // let duration = duration.as_millis();
        println!("\nRuntime: {duration:#?} ns");

        let mut table = Vec::new();
        let re = Regex::new(r"(\d+): (?P<val>\d+)")?;

        let mut row = vec![];
        for i in 0..=values.get(0).context(OutOfBounds)?.len() {
            row.push(i.cell().justify(Justify::Center).bold(true));
        }
        table.push(row);

        for (i, value) in values.iter().enumerate() {
            let mut row = Vec::new();
            row.push((i + 1).cell().justify(Justify::Center).bold(true));

            for col in value {
                let c = re.captures(&col).context(RegexCapture)?;

                let res = if c["val"].parse::<f64>()? > 1024.0 * (2.0 / 3.0) {
                    col.cell().foreground_color(Some(Color::Green))
                }
                else {
                    col.cell().foreground_color(Some(Color::Red))
                };

                row.push(res);
            }

            table.push(row);
        }

        let table = table.table();
        print_stdout(table)?;

        Ok(())
    }
}
