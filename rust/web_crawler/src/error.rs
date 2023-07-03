use std::convert::From;
use std::string::FromUtf8Error;
use thiserror::Error;
use tokio::sync::AcquireError;
use url::ParseError;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Reqwest: {0}")]
    Reqwest(String),
    #[error("Utf8Error: {0}")]
    Utf8Error(String),
    #[error("AcquireError: {0}")]
    AcquireError(String),
    #[error("ParseError: {0}")]
    ParseError(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

impl std::convert::From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::Utf8Error(err.to_string())
    }
}

impl From<AcquireError> for Error {
    fn from(error: AcquireError) -> Self {
        Error::AcquireError(error.to_string())
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::ParseError(error.to_string())
    }
}
