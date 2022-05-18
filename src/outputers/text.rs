use async_trait::async_trait;
use cli_table::format::Justify;
use cli_table::{print_stdout, Cell, Style, Table};
use derive_more::Constructor;
use snafu::OptionExt;
use termcolor::Color;
use tokio::time::Duration;

use crate::error::OutOfBounds;
use crate::traits::outputer::Value;
use crate::traits::Outputer;
use crate::Error;

#[derive(Constructor)]
pub struct TextOutputer;

#[async_trait]
impl Outputer for TextOutputer {
    async fn output_table(
        &self,
        values: Vec<Vec<Value>>,
        duration: Vec<Duration>,
    ) -> Result<(), Error> {
        // TODO fix crash when Volume is used
        let values = dbg!(values);
        let duration = dbg!(duration);

        let mut table_dur = vec![];
        let mut table = vec![];

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
                let res = if (col.correct as f64) > 1024.0 * (2.0 / 3.0) {
                    format!("{}: {}", col.result, col.correct)
                        .cell()
                        .foreground_color(Some(Color::Green))
                }
                else {
                    format!("{}: {}", col.result, col.correct)
                        .cell()
                        .foreground_color(Some(Color::Red))
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

    async fn output_volume(
        &self,
        values: Vec<Value>,
        duration: Vec<Duration>,
    ) -> Result<(), Error> {
        let len = values.len();
        let mut table = vec![];
        for (i, (val, dur)) in values.into_iter().zip(duration).enumerate() {
            let mut row = vec![];
            let i = i + 1;
            row.push(format!("{i} x {i}").cell().background_color(Some(Color::Magenta)));
            row.push(val.correct.cell());
            row.push(format!("{} ms", dur.as_millis()).cell());
            table.push(row);
        }

        println!("\nResult:");
        print_stdout(table.table())?;

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
            let mut row = vec![];
            row.push(format!("{} x {width}", i + 1).cell().background_color(Some(Color::Magenta)));
            row.push(val.correct.cell());
            row.push(format!("{} ms", dur.as_millis()).cell());
            table.push(row);
        }

        println!("\nResult:");
        print_stdout(table.table())?;
        Ok(())
    }
}
