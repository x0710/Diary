use std::fmt::Display;
use time::error::Parse;

#[derive(Debug)]
pub enum Error {
    Db(rusqlite::Error),
    InvalidDate(String),
    UnknownCommand(String),
}

impl From<Parse> for Error {
    fn from(value: Parse) -> Self {
        Error::InvalidDate(value.to_string())
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::Db(err)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
