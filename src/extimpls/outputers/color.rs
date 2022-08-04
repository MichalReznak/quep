use async_trait::async_trait;
use cli_table::format::Justify;
use cli_table::{print_stdout, Cell, CellStruct, Style, Table, TableStruct};
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
pub struct ColorOutputer {
    args: CliArgsOutput,
}

impl ColorOutputer {
    #[throws]
    pub fn from_args(args: &CliArgsOutput) -> Self {
        Self { args: args.clone() }
    }
}

fn to_color_cell(val: i32) -> CellStruct {
    let a = (1024 - (1024 * 2 / 3)) / 4;
    let b = 1024 - a;
    let c = b - a;
    let d = c - a;
    let e = d - a;

    let res = if val >= b {
        Color::Rgb(105, 179, 76)
    }
    else if val >= c {
        Color::Rgb(172, 179, 52)
    }
    else if val >= d {
        Color::Rgb(255, 142, 21)
    }
    else if val >= e {
        Color::Rgb(255, 78, 17)
    }
    else {
        Color::Rgb(255, 13, 13)
    };

    "   ".cell().background_color(Some(res))
}

fn color_table() -> TableStruct {
    let a = (1024 - (1024 * 2 / 3)) / 4;
    let b = 1024 - a;
    let c = b - a;
    let d = c - a;
    let e = d - a;

    vec![
        vec![
            "   ".cell().background_color(Some(Color::Rgb(105, 179, 76))),
            format!(">= {b}").cell(),
        ],
        vec![
            "   ".cell().background_color(Some(Color::Rgb(172, 179, 52))),
            format!(">= {c}").cell(),
        ],
        vec![
            "   ".cell().background_color(Some(Color::Rgb(255, 142, 21))),
            format!(">= {d}").cell(),
        ],
        vec![
            "   ".cell().background_color(Some(Color::Rgb(255, 78, 17))),
            format!(">= {e}").cell(),
        ],
        vec![
            "   ".cell().background_color(Some(Color::Rgb(255, 13, 13))),
            format!("<  {e}").cell(),
        ], // TODO not 'e'
    ]
    .table()
    .title(vec!["Color".cell().bold(true), "Range".cell().bold(true)])
    .bold(true)
}

#[async_trait]
impl Outputer for ColorOutputer {
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

        let mut dur_i = 0;
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

            for col in value {
                if let Some(durations) = &durations {
                    row_dur.push(
                        format!("{} ms", durations.get(dur_i).context(OutOfBounds)?.as_millis())
                            .cell()
                            .justify(Justify::Right),
                    );
                    dur_i += 1;
                }
                row.push(to_color_cell(col.correct));
            }

            table_dur.push(row_dur);
            table.push(row);
        }

        println!("\nAgenda:"); // TODO rename?
        print_stdout(color_table())?;

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

            if val.is_correct {
                last_correct = i;
            }

            row.push(format!("{i} x {i}").cell().background_color(Some(Color::Cyan)));
            row.push(val.result.cell());
            row.push(to_color_cell(val.correct));
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

            row.push(format!("{i} x {width}").cell().background_color(Some(Color::Cyan)));
            row.push(val.result.cell());
            row.push(to_color_cell(val.correct));
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
