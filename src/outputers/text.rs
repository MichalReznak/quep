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
        let mut table_dur = vec![];
        let mut table = vec![];

        let re = Regex::new(r"(\d+): (?P<val>\d+)")?;

        let mut row = vec![];
        let mut row_dur = vec![];
        for i in 0..=values.get(0).context(OutOfBounds)?.len() {
            row.push(
                i.cell()
                    .justify(Justify::Center)
                    .bold(true)
                    .background_color(Some(Color::Magenta)),
            );
            row_dur.push(
                i.cell()
                    .justify(Justify::Center)
                    .bold(true)
                    .background_color(Some(Color::Magenta)),
            );
        }
        table.push(row);
        table_dur.push(row_dur);

        for (i, value) in values.iter().enumerate() {
            let mut row = vec![];
            let mut row_dur = vec![];
            row.push(
                (i + 1)
                    .cell()
                    .justify(Justify::Center)
                    .bold(true)
                    .background_color(Some(Color::Magenta)),
            );
            row_dur.push(
                (i + 1)
                    .cell()
                    .justify(Justify::Center)
                    .bold(true)
                    .background_color(Some(Color::Magenta)),
            );

            for (j, col) in value.iter().enumerate() {
                let c = re.captures(&col).context(RegexCapture)?;

                let res = if c["val"].parse::<f64>()? > 1024.0 * (2.0 / 3.0) {
                    col.cell().foreground_color(Some(Color::Green))
                }
                else {
                    col.cell().foreground_color(Some(Color::Red))
                };

                row_dur.push(
                    format!("{} ms", duration.get(i + j).context(OutOfBounds)?.as_millis())
                        .cell()
                        .justify(Justify::Right),
                );
                row.push(res);
            }

            table_dur.push(row_dur);
            table.push(row);
        }

        println!("\nResult:");
        print_stdout(table.table())?;

        println!("\nRuntime:");
        print_stdout(table_dur.table())?;
        Ok(())
    }
}
