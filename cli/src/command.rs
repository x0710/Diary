use std::str::FromStr;
use diary_core::base::command::Command;
use diary_core::base::executor::Executor;
use diary_core::model::day::Day;
use diary_core::base::error::Error;
use crate::error::CliError;

#[derive(Debug)]
pub enum CliCommand {
    Command(Command),
    Help,
    Quit,
}

impl CliCommand {
    pub fn exec(&self, exec: &Executor) -> Result<Vec<Day>, CliError> {
        match self {
            CliCommand::Command(comm) => exec.exec(comm)
                .map_err(|e| e.into()),
            CliCommand::Help => {
                self.handle_help();
                Ok(Vec::new())
            }
            CliCommand::Quit => Err(CliError::Quit),
        }
    }
    fn handle_version(&self) {
        println!("version: {}", env!("CARGO_PKG_VERSION"));
    }
    fn handle_help(&self) {
        self.handle_version();
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
                    _ => Err(CliError::UnknownCommand(s.to_string()))
                }
            },
            Err(e) => Err(e.into()),
        }
    }
}