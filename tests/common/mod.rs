use anyhow::{Context, Error};
use fehler::throws;

#[throws]
pub fn get_dir(s: &str) -> String {
    dunce::canonicalize(s)?.to_str().context("Cannot to string")?.to_owned()
}
