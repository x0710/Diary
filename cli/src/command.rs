use std::str::FromStr;
use diary_core::base::command::Command;
use diary_core::base::env::version;
use diary_core::base::executor::Executor;
use diary_core::base::error::Error;
use diary_core::model::Day;
use crate::error::CliError;

#[derive(Debug)]
pub enum CliCommand {
    Command(Command),
    Version,
    Help,
    Quit,
}

impl CliCommand {
    pub async fn exec(&self, exec: &mut Executor) -> Result<Vec<Day>, CliError> {
        match self {
            CliCommand::Command(comm) => exec.exec(comm).await
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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s.parse::<Command>();
        match res {
            Ok(cmd) => Ok(CliCommand::Command(cmd)),
            Err(Error::UnknownCommand(cmd)) => {
                match cmd.as_str() {
                    "help" | "h" => Ok(CliCommand::Help),
                    "quit" | "exit" | "q" => Ok(CliCommand::Quit),
                    "version" | "v" => Ok(CliCommand::Version),
                    _ => Err(CliError::UnknownCommand(s.to_string()))
                }
            },
            Err(e) => Err(e.into()),
        }
    }
}