use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(false)))]
pub enum Error {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidIdError { id: u16 },
}
