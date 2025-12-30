use time::error::Parse;

#[derive(Debug)]
pub enum Error {
    Exit,
    Io(std::io::Error),
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
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
