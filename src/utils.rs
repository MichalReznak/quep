use fehler::throws;
use snafu::OptionExt;

use crate::error::Utf16;
use crate::Error;

#[throws]
pub fn dir(s: &str) -> String {
    dunce::canonicalize(s)?.to_str().context(Utf16)?.to_owned()
}
