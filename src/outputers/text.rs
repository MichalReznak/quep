use async_trait::async_trait;
use cli_table::format::Justify;
use cli_table::{print_stdout, Cell, Style, Table};
use snafu::OptionExt;
use termcolor::Color;
use tokio::time::Duration;

use crate::args::CliArgsOutput;
use crate::error::OutOfBounds;
use crate::ext::outputer::Value;
use crate::ext::Outputer;
use crate::Error;

#[allow(dead_code)]
pub struct TextOutputer {
    args: CliArgsOutput,
}

impl TextOutputer {
    pub fn new(args: &CliArgsOutput) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl Outputer for TextOutputer {
    async fn output_table(
        &self,
        values: Vec<Vec<Value>>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<String, Error> {
        let mut table_dur = vec![];
        let mut table = vec![];

        let mut row = vec![];
        let mut row_dur = vec![];
        for i in 0..=values.get(0).context(OutOfBounds)?.len() {
            row.push(
                i.cell().justify(Justify::Center).bold(true).background_color(Some(Color::Cyan)),
            );
            row_dur.push(
                i.cell().justify(Justify::Center).bold(true).background_color(Some(Color::Cyan)),
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
                    .background_color(Some(Color::Cyan)),
            );
            row_dur.push(
                (i + 1)
                    .cell()
                    .justify(Justify::Center)
                    .bold(true)
                    .background_color(Some(Color::Cyan)),
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

                if let Some(durations) = &durations {
                    row_dur.push(
                        format!("{} ms", durations.get(i + j).context(OutOfBounds)?.as_millis())
                            .cell()
                            .justify(Justify::Right),
                    );
                }
                row.push(res);
            }

            table_dur.push(row_dur);
            table.push(row);
        }

        println!("\nResult:");
        print_stdout(table.table())?;

        if durations.is_some() {
            println!("\nRuntime:");
            print_stdout(table_dur.table())?;
        }

        println!("\nApplication Runtime: {} ms", runtime.as_millis());
        Ok("".to_string()) // TODO Cannot return as a string
    }

    async fn output_volume(
        &self,
        values: Vec<Value>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
    ) -> Result<String, Error> {
        let include_durs = matches!(durations, Some(_));
        let durations = durations.unwrap_or_else(|| {
            vec![Duration::from_millis(0)].into_iter().cycle().take(values.len()).collect()
        });

        let len = values.len();
        let mut table = vec![];
        for (i, (val, dur)) in values.into_iter().zip(durations).enumerate() {
            let mut row = vec![];
            let i = i + 1;

            let val = val.correct.cell().foreground_color(Some(if val.is_correct {
                Color::Green
            }
            else {
                Color::Red
            }));

            row.push(format!("{i} x {i}").cell().background_color(Some(Color::Cyan)));
            row.push(val);
            if include_durs {
                row.push(format!("{} ms", dur.as_millis()).cell());
            }
            table.push(row);
        }

        println!("\nResult:");
        print_stdout(table.table())?;

        println!("\nQuantum Volume: {}", 2i32.pow((len - 1) as u32));

        println!("\nApplication Runtime: {} ms", runtime.as_millis());
        Ok("".to_string()) // TODO Cannot return as a string
    }

    async fn output_linear(
        &self,
        values: Vec<Value>,
        durations: Option<Vec<Duration>>,
        width: i32,
        runtime: Duration,
    ) -> Result<String, Error> {
        let include_durs = matches!(durations, Some(_));
        // TODO this hack is not pretty
        let durations = durations.unwrap_or_else(|| {
            vec![Duration::from_millis(0)].into_iter().cycle().take(values.len()).collect()
        });

        let mut table = vec![];
        for (i, (val, dur)) in values.into_iter().zip(durations).enumerate() {
            let mut row = vec![];

            let val = val.correct.cell().foreground_color(Some(
                if (val.correct as f64) > 1024.0 * (2.0 / 3.0) {
                    Color::Green
                }
                else {
                    Color::Red
                },
            ));

            row.push(format!("{} x {width}", i + 1).cell().background_color(Some(Color::Cyan)));
            row.push(val);
            if include_durs {
                row.push(format!("{} ms", dur.as_millis()).cell());
            }
            table.push(row);
        }

        println!("\nResult:");
        print_stdout(table.table())?;

        println!("\nApplication Runtime: {} ms", runtime.as_millis());
        Ok("".to_string()) // TODO Cannot return as a string
    }
}
