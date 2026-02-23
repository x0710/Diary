use diary_core::base::error::Error;

pub enum CliError {
    CoreError(Error),
    UnknownCommand(String),
    InvalidArgs(String),
    Io(std::io::Error),
    Quit,
}
impl From<rusqlite::Error> for CliError {
    fn from(error: rusqlite::Error) -> Self {
        CliError::CoreError(Error::Db(error))
    }
}
impl From<Error> for CliError {
    fn from(err: Error) -> Self {
        CliError::InvalidArgs(err.to_string())
    }
}
impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err)
    }
}