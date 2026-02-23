//! 软件支持的命令
use std::str::FromStr;
use crate::base::date::Date;
use crate::base::error::Error;

/// 储存用户在做操作时的参数
#[derive(Debug, Clone)]
pub enum Command {
    Add(Date, Option<String>),
    Remove(Date),
    Check(Date),
    ListAll,
    // Quit,
    // Help,
}
/// 表示用户正在做的操作
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
impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut args = s.splitn(3, ' ');
        let sub = args.next().unwrap_or_default().parse::<SubCommand>()?;
        let date = args.next().unwrap_or_default();
        let ctx = args.next();

        match sub {
            SubCommand::Add => {
                let date = date.parse::<Date>()?;
                Ok(Command::Add(date, ctx.map(str::to_string)))
            }
            SubCommand::Remove => {
                let date = date.parse::<Date>()?;
                Ok(Command::Remove(date))
            }
            SubCommand::Check => {
                let date = date.parse::<Date>()?;
                Ok(Command::Check(date))
            }
            SubCommand::ListAll => Ok(Command::ListAll),
            // SubCommand::Help => Ok(Command::Help),
            // SubCommand::Quit => Ok(Command::Quit),
        }
    }
}
