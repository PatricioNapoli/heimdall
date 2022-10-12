use std::error::Error as StdError;
use std::io;
use derive_more::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "HTTP error")]
    Http(http::Error),

    #[display(fmt = "Hyper error")]
    Hyper(hyper::Error),

    #[display(fmt = "I/O error")]
    Io(io::Error),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;

        match self {
            Io(e) => Some(e),
            Http(e) => Some(e),
            Hyper(e) => Some(e)
        }
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Error {
        Error::Http(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Error {
        Error::Hyper(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
