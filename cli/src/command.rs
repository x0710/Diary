//! 可供操作命令

use std::str::FromStr;
use diary_core::base::date::Date;
use diary_core::base::env::version;
use diary_core::model::Day;
use crate::error::CliError;
use crate::executor::Executor;
use SubCommand::*;

/// 储存用户在做操作时的参数
#[derive(Debug, Clone)]
pub enum Command {
    Add(Day),
    Remove(Date),
    Check(Date),
    ListAll,
}
#[derive(Debug, Clone, Copy)]
pub enum SubCommand {
    Add,
    Remove,
    Check,
    ListAll,
}
#[derive(Debug)]
pub enum CliCommand {
    CoreCommand(Command),
    Version,
    Help,
    Quit,
}

impl CliCommand {
    pub async fn exec(&self, exec: &mut Executor) -> Result<Vec<Day>, CliError> {
        match self {
            CliCommand::CoreCommand(comm) => exec.exec(comm).await
                .map_err(Into::into),
            CliCommand::Help => {
                self.handle_help();
                Ok(Vec::new())
            },
            CliCommand::Version => {
                self.handle_version();
                Ok(Vec::new())
            },
            CliCommand::Quit => Err(CliError::Quit),
        }
    }
    fn handle_version(&self) {
        println!("cli-version: {}\ncore-version: {}",
                 env!("CARGO_PKG_VERSION"),
                 version());
    }
    fn handle_help(&self) {
        println!(r#"
Available commands:
  add <date> [context]  - Add or edit an entry with context appened at the last
        (e.g., add today, add 20251225, add ye1225, add m25)
  remove <date>         - Delete an entry
  check <date>          - View a specific entry
  list                  - List all entries
  quit/exit             - Close the application
"#);
    }
}
impl FromStr for CliCommand {
    type Err = CliError;
    fn from_str(s: &str) -> Result<Self, CliError> {
        match s.trim() {
            "help" | "h" => return Ok(Self::Help),
            "quit" | "exit" | "q" => return Ok(Self::Quit),
            "version" | "v" => return Ok(Self::Version),
            _ => (),
        }

        Ok(Self::CoreCommand(s.parse()?))
    }
}
impl FromStr for SubCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "ad" | "add" => Ok(Add),
            "rm" | "remove" | "delete" | "del" => Ok(Remove),
            "chk" | "check" | "read" | "show" => Ok(Check),
            "ls" | "list" => Ok(ListAll),
            _ => Err(s.to_string()),
        }
    }
}
impl FromStr for Command {
    type Err = CliError;
    fn from_str(s: &str) -> Result<Self, CliError> {
        let mut args = s.split_whitespace();
        let sub = args.next().unwrap_or_default()
            .parse::<SubCommand>()
            .map_err(|s| CliError::UnknownCommand(s))?;

        let date = args.next()
            .unwrap_or_default()
            .parse::<Date>();
        let _ctx = args.next();

        match sub {
            Add => Ok(Command::Add(Day {
                date: date?,
                ..Day::default()
            })),
            Remove => Ok(Command::Remove(date?)),
            Check => Ok(Command::Check(date?)),
            ListAll => Ok(Command::ListAll),
        }
    }
}

