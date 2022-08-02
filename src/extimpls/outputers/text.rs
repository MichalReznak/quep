use async_trait::async_trait;
use cli_table::format::Justify;
use cli_table::{print_stdout, Cell, Style, Table};
use fehler::throws;
use snafu::OptionExt;
use termcolor::Color;
use tokio::time::Duration;

use crate::args::CliArgsOutput;
use crate::error::OutOfBounds;
use crate::ext::outputer::OutValue;
use crate::ext::Outputer;
use crate::Error;

#[allow(dead_code)]
pub struct TextOutputer {
    args: CliArgsOutput,
}

impl TextOutputer {
    #[throws]
    pub fn from_args(args: &CliArgsOutput) -> Self {
        Self { args: args.clone() }
    }
}

#[async_trait]
impl Outputer for TextOutputer {
    async fn output_table(
        &self,
        values: Vec<Vec<OutValue>>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
        from: i32,
        from2: i32,
    ) -> Result<Option<String>, Error> {
        let mut table_dur = vec![];
        let mut table = vec![];

        let mut row =
            vec![0.cell().justify(Justify::Center).bold(true).background_color(Some(Color::Cyan))];
        let mut row_dur =
            vec![0.cell().justify(Justify::Center).bold(true).background_color(Some(Color::Cyan))];
        for i in 0..values.get(0).context(OutOfBounds)?.len() {
            let i = i + from2 as usize;
            row.push(
                i.cell().justify(Justify::Center).bold(true).background_color(Some(Color::Cyan)),
            );
            row_dur.push(
                i.cell().justify(Justify::Center).bold(true).background_color(Some(Color::Cyan)),
            );
        }
        table.push(row);
        table_dur.push(row_dur);

        let val_len = values.get(0).context(OutOfBounds)?.len();

        for (i, value) in values.iter().enumerate() {
            let i = i + from as usize;
            let mut row = vec![];
            let mut row_dur = vec![];
            row.push(
                i.cell().justify(Justify::Center).bold(true).background_color(Some(Color::Cyan)),
            );
            row_dur.push(
                i.cell().justify(Justify::Center).bold(true).background_color(Some(Color::Cyan)),
            );

            for (j, col) in value.iter().enumerate() {
                let j = j + from2 as usize;
                let res = if col.is_correct {
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
                    let dur_i = ((i - from as usize) * val_len) + (j - from2 as usize);
                    row_dur.push(
                        format!("{} ms", durations.get(dur_i).context(OutOfBounds)?.as_millis())
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
        Ok(None)
    }

    async fn output_volume(
        &self,
        values: Vec<OutValue>,
        durations: Option<Vec<Duration>>,
        runtime: Duration,
        from: i32,
    ) -> Result<Option<String>, Error> {
        let include_durs = matches!(durations, Some(_));
        let durations = durations.unwrap_or_else(|| {
            vec![Duration::from_millis(0)].into_iter().cycle().take(values.len()).collect()
        });

        let mut table = vec![];
        let mut last_correct = 0;

        for (i, (val, dur)) in values.into_iter().zip(durations).enumerate() {
            let i = i + from as usize;
            let mut row = vec![];

            let val_res = val.result.cell();
            let color = if val.is_correct {
                last_correct = i;
                Color::Green
            }
            else {
                Color::Red
            };

            let val = val.correct.cell().foreground_color(Some(color));

            row.push(format!("{i} x {i}").cell().background_color(Some(Color::Cyan)));
            row.push(val_res);
            row.push(val);
            if include_durs {
                row.push(format!("{} ms", dur.as_millis()).cell());
            }
            table.push(row);
        }

        println!("\nResult:");
        print_stdout(table.table())?;

        println!("\nQuantum Volume (log): {}", last_correct);

        println!("\nApplication Runtime: {} ms", runtime.as_millis());
        Ok(None)
    }

    async fn output_linear(
        &self,
        values: Vec<OutValue>,
        durations: Option<Vec<Duration>>,
        width: i32,
        runtime: Duration,
        from: i32,
    ) -> Result<Option<String>, Error> {
        let include_durs = matches!(durations, Some(_));
        let durations = durations.unwrap_or_else(|| {
            vec![Duration::from_millis(0)].into_iter().cycle().take(values.len()).collect()
        });

        let mut table = vec![];
        for (i, (val, dur)) in values.into_iter().zip(durations).enumerate() {
            let i = i + from as usize;
            let mut row = vec![];

            let val_res = val.result.cell();
            let val = val.correct.cell().foreground_color(Some(if val.is_correct {
                Color::Green
            }
            else {
                Color::Red
            }));

            row.push(format!("{i} x {width}").cell().background_color(Some(Color::Cyan)));
            row.push(val_res);
            row.push(val);
            if include_durs {
                row.push(format!("{} ms", dur.as_millis()).cell());
            }
            table.push(row);
        }

        println!("\nResult:");
        print_stdout(table.table())?;

        println!("\nApplication Runtime: {} ms", runtime.as_millis());
        Ok(None)
    }
}
