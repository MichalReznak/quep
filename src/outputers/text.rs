use async_trait::async_trait;
use derive_more::Constructor;
use tokio::time::Duration;
use cli_table::{print_stdout, Cell, Table};

use crate::traits::Outputer;

#[derive(Constructor)]
pub struct TextOutputer;

#[async_trait]
impl Outputer for TextOutputer {
    async fn output(&self, values: Vec<Vec<String>>, duration: Vec<Duration>) -> Result<(), crate::Error> {
        let mut table = Vec::new();

        for value in values {
            let mut row = Vec::new();
            for col in value {
                row.push(col.cell());
            }

            table.push(row);
        }

        let table = table.table();
        print_stdout(table).unwrap();


        // let duration = duration.as_millis();
        println!("\nRuntime: {duration:#?} ns");
        Ok(())
    }
}
