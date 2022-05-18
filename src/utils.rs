use fehler::throws;
use snafu::OptionExt;

use crate::error::Utf16;
use crate::Error;

#[throws]
pub fn dir(s: &str) -> String {
    dunce::canonicalize(s)?.to_str().context(Utf16)?.to_owned()
}

#[cfg(not(debug_assertions))]
pub fn debug() -> bool {
    false
}

#[cfg(debug_assertions)]
pub fn debug() -> bool {
    true
}
