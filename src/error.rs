use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    None(Msg),
    Io(io::Error),
    FromUtf8(FromUtf8Error),
    SecretboxOpen(Msg),
    Database(diesel::result::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::None(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::FromUtf8(ref err) => err.fmt(f),
            Error::SecretboxOpen(ref err) => err.fmt(f),
            Error::Database(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::None(err) => Some(err),
            Error::Io(err) => Some(err),
            Error::FromUtf8(err) => Some(err),
            Error::SecretboxOpen(err) => Some(err),
            Error::Database(err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::FromUtf8(error)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Error {
        Error::Database(error)
    }
}

#[derive(Debug)]
pub struct Msg(String);

impl Msg {
    pub fn new(msg: &str) -> Msg {
        Msg(String::from(msg))
    }
}

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Msg {
    fn description(&self) -> &str {
        &self.0
    }
}