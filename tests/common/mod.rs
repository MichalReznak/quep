use anyhow::{Context, Error};
use fehler::throws;

#[throws]
pub fn get_dir() -> String {
    dunce::canonicalize("./python")?
        .to_str()
        .context("Cannot to string")?
        .to_owned()
}
