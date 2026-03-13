use std::fmt::Display;
use time::error::Parse;

#[derive(Debug)]
pub enum Error {
    Db(sqlx::Error),
    Io(std::io::Error),
    InvalidData(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<Parse> for Error {
    fn from(value: Parse) -> Self {
        Error::InvalidData(value.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Db(err)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Db(e) => write!(f, "Database error: {}", e),
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::InvalidData(id) => write!(f, "Invalid data: {}", id),
        }
    }
}
