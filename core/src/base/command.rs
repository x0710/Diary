use std::str::FromStr;
use crate::base::date::Date;
use crate::base::error::Error;

#[derive(Debug, Clone)]
pub enum Command {
    Add(Date, Option<String>),
    Remove(Date),
    Check(Date),
    ListAll,
    // Quit,
    // Help,
}
#[derive(Debug, Clone, Copy)]
pub enum SubCommand {
    Add,
    Remove,
    Check,
    ListAll,
    // Quit,
    // Help,
}
impl FromStr for SubCommand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ad" | "add" => Ok(SubCommand::Add),
            "rm" | "remove" | "delete" | "del" => Ok(SubCommand::Remove),
            "chk" | "check" | "read" | "show" => Ok(SubCommand::Check),
            "ls" | "list" => Ok(SubCommand::ListAll),
            // "h" | "help" => Ok(SubCommand::Help),
            // "quit" | "exit" | "q" => Ok(SubCommand::Quit),
            _ => Err(Error::UnknownCommand(s.to_string())),
        }
    }
}
