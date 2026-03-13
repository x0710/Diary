use std::fmt::{Display, Formatter};
use diary_core::base::error::Error;

#[derive(Debug)]
pub enum CliError {
    UnknownCommand(String),
    InvalidArgs(String),
    /// 表示内部错误
    Core(Error),
    Quit,
}
impl From<Error> for CliError {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidData(id) => CliError::InvalidArgs(id),
            _ => CliError::Core(err),
        }
    }
}
impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::InvalidArgs(s) => write!(f, "Invalid args: {}", s),
            CliError::UnknownCommand(s) => write!(f, "Unknown command: {}", s),
            CliError::Core(e) => write!(f, "Error Happened: {}", e),
            _ => write!(f, "{:?}", self),
        }
    }
}