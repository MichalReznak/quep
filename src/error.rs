use std::num::{ParseFloatError, ParseIntError};

use pyo3::PyErr;
use quick_xml::DeError;
use snafu::prelude::*;
use strum::ParseError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)))]
pub enum Error {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidIdError { id: u16 },

    #[snafu(display("{}", source), context(false))]
    ParseError { source: ParseError },

    #[snafu(display("{}", source), context(false))]
    IoError { source: std::io::Error },

    #[snafu(display("{}", source), context(false))]
    Pyo3Error { source: PyErr },

    #[snafu(display("{}", source), context(false))]
    RegexError { source: regex::Error },

    #[snafu(display("{}", source), context(false))]
    ParseFloatError { source: ParseFloatError },

    #[snafu(display("{}", source), context(false))]
    ParseIntError { source: ParseIntError },

    #[snafu(display("{}", source), context(false))]
    FmtError { source: std::fmt::Error },

    #[snafu(display("{}", source), context(false))]
    SerdeJson5Error { source: json5::Error },

    #[snafu(display("{}", source), context(false))]
    SerdeJsonError { source: serde_json::Error },

    #[snafu(display("{}", source), context(false))]
    SerdeXmlError { source: DeError },

    #[snafu(display("{}", source), context(false))]
    SerdeYamlError { source: serde_yaml::Error },

    #[snafu(display("{}", source), context(false))]
    SerdeTomlError { source: toml::ser::Error },

    #[snafu(display("{}", source), context(false))]
    SerdeRonError { source: ron::Error },

    #[snafu(display("Py downcast error has lifetime"))]
    PyDowncastError,

    #[snafu(display("Windows path is wrong"))]
    Utf16Error,

    #[snafu(display("Regex capture error"))]
    RegexCaptureError,

    #[snafu(display("Index out of bounds"))]
    OutOfBoundsError,

    #[snafu(display("Some random error"))]
    SomeError,
}
