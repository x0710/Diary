use std::str::FromStr;
use diary_core::base::date::Date;
use diary_core::db::command::Command;
use diary_core::base::env::version;
use diary_core::db::executor::Executor;
use diary_core::base::error::Error;
use diary_core::model::Day;
use crate::error::CliError;

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
                .map_err(|e| e.into()),
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
impl ParseStr for CliCommand {
    fn convert(s: &str) -> Result<Self, Error> {
        let res = Command::convert(s);
        match res {
            Ok(cmd) => Ok(CliCommand::CoreCommand(cmd)),
            Err(Error::UnknownCommand(cmd)) => {
                match cmd.as_str() {
                    "help" | "h" => Ok(CliCommand::Help),
                    "quit" | "exit" | "q" => Ok(CliCommand::Quit),
                    "version" | "v" => Ok(CliCommand::Version),
                    _ => Err(Error::UnknownCommand(s.to_string()))
                }
            },
            Err(e) => Err(e.into()),
        }
    }
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
    fn from_str(s: &str) -> Result<Self, Error> {
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
impl ParseStr for Command {
    fn convert(s: &str) -> Result<Self, Error> {
        // 以命令`>: ad 0 ctx`为例，
        let mut args = s.split_whitespace();
        // `SubCommand::Add`
        let sub = args.next().unwrap_or_default().parse()?;
        // today
        let date = args.next().unwrap_or_default();
        // Some("ctx")
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

pub trait ParseStr: Sized {
    fn convert(s: &str) -> Result<Self, Error>;
}