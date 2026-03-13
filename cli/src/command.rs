//! 可供操作命令

use std::str::FromStr;
use diary_core::base::date::Date;
use diary_core::base::env::version;
use diary_core::model::Day;
use crate::error::CliError;
use crate::executor::Executor;

/// 储存用户在做操作时的参数
#[derive(Debug, Clone)]
pub enum Command {
    Add(Date, Option<String>),
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
impl FromStr for CliCommand {
    type Err = CliError;
    fn from_str(s: &str) -> Result<Self, CliError> {
        let res = s.parse();
        match res {
            Ok(cmd) => Ok(CliCommand::CoreCommand(cmd)),
            Err(CliError::UnknownCommand(cmd)) => {
                match cmd.as_str() {
                    "help" | "h" => Ok(CliCommand::Help),
                    "quit" | "exit" | "q" => Ok(CliCommand::Quit),
                    "version" | "v" => Ok(CliCommand::Version),
                    _ => Err(CliError::UnknownCommand(cmd)),
                }
            },
            Err(e) => Err(e.into()),
        }
    }
}
impl FromStr for SubCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "ad" | "add" => Ok(SubCommand::Add),
            "rm" | "remove" | "delete" | "del" => Ok(SubCommand::Remove),
            "chk" | "check" | "read" | "show" => Ok(SubCommand::Check),
            "ls" | "list" => Ok(SubCommand::ListAll),
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
        let ctx = args.next();

        match sub {
            SubCommand::Add => {
                Ok(Command::Add(date?, ctx.map(str::to_string)))
            }
            SubCommand::Remove => {
                Ok(Command::Remove(date?))
            }
            SubCommand::Check => {
                Ok(Command::Check(date?))
            }
            SubCommand::ListAll => Ok(Command::ListAll),
        }
    }
}

