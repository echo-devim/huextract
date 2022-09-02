//! Module defining the Error type used in the program
//!
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error::from(msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self {
            msg: format!("IO error (kind: {}): {error}", error.kind()),
        }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self {
            msg: format!("conversion error: {e}"),
        }
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Self { msg: s.to_owned() }
    }
}
