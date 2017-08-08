use ::std::error;
use ::std::fmt::{self, Formatter};
use ::std::io;
use ::std::num;
use ::std::result;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::Parse(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        match self {
            &Error::IO(ref e) => e.fmt(f),
            &Error::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::IO(ref e) => e.description(),
            &Error::Parse(ref e) => e.description(),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;