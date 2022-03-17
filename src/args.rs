use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(long, env = "QUEP_FOO")]
    pub foo: String,
}

lazy_static! {
    pub static ref ARGS: CliArgs = {
        dotenv::dotenv().ok();
        CliArgs::parse()
    };
}
