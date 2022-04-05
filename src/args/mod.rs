use clap::Parser;
use lazy_static::lazy_static;

pub mod types;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(long, env = "QUEP_PROVIDER")]
    pub provider: String,

    #[clap(long, env = "QUEP_OUTPUT")]
    pub output: String,

    #[clap(long, env = "QUEP_CIRCUIT")]
    pub circuit: String,

    #[clap(long, env = "QUEP_SIZE")]
    pub size: i32,
}

lazy_static! {
    pub static ref ARGS: CliArgs = {
        dotenv::dotenv().ok();
        CliArgs::parse()
    };
}
