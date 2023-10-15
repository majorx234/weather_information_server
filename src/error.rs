use reqwest;
use std::error;
use std::fmt;
use std::io;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    HttpError(reqwest::Error),
    UrlError(ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "{}", e),
            Error::HttpError(e) => write!(f, "{}", e),
            Error::UrlError(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IOError(e) => Some(e),
            Error::HttpError(e) => Some(e),
            Error::UrlError(e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HttpError(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::UrlError(e)
    }
}
