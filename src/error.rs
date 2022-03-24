#[cfg(feature = "qiskit")]
use pyo3::PyErr;
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

    #[snafu(display("Some random error"))]
    SomeError,

    #[cfg(feature = "qiskit")]
    #[snafu(display("{}", source), context(false))]
    Pyo3Err { source: PyErr },
}
