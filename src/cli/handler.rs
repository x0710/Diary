use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::ExitStatus;
use std::str::FromStr;
use clap::Parser;
use rustyline::error::ReadlineError;
use time::error::Parse;
use crate::cli::args;
use crate::cli::args::Args;
use crate::cli::handler::Mode::{Interactive, Once};
use crate::model::day::Day;
use crate::model::event::Event;
use crate::storage::db_mgr::DatabaseManager;

pub struct CliHandler {
    conn: DatabaseManager,
    pub args: Args,
    mode: Mode,
}
impl CliHandler {
    pub fn new(conn: DatabaseManager) -> CliHandler {
        let args = Args::parse();
        let mode = if args.interactive { Interactive } else {
            if args.time.is_none() {
                Interactive
            }else {unimplemented!();Once}
        };
        Self {
            conn,
            args,
            mode,
        }
    }
    pub fn run(&self) {
        match self.mode {
            Interactive => self.interactive(),
            Once => self.once(),
        }
    }
    fn once(&self) {
        let date = parse_to_date(self.args.time.as_deref().unwrap())
            .expect("Unable to parse date");
        if let Some(event) = self.args.event.as_deref() {
            // write diary
            let event = Event::new(event);
            let d = Day::from_date(date)
                .with_event(&event);
            self.conn.add_day(d).expect("Could not add day");
        }else {
            // check the day
            let res = self.conn.read_day(date).expect("This day has not yet been recorded.");
            println!("{}", res);
        }
    }
    fn interactive(&self) {
        use rustyline::DefaultEditor;
        let mut rl = DefaultEditor::new().unwrap();
        loop {
            match rl.readline(">: ") {
                Ok(line) => {
                    match self.exec_command(line.as_str()) {
                        Err(CliErr::Db(err)) => {
                            eprintln!("Database error: {}", err);
                        },
                        Err(CliErr::Io(err)) => {
                            eprintln!("IO Error: {}", err);
                        },
                        Err(CliErr::InvalidDate(err)) => {
                            eprintln!("Invalid date: {}", err);
                        },
                        Err(CliErr::UnknownCommand(cmd)) => {
                            eprintln!("Unknown command: {}", cmd);
                        },
                        Err(CliErr::Exit) => {
                            println!("Have a nice day!");
                            break;
                        },
                        Ok(_) => (),
                    }
                },
                Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
                    println!("Have a nice day!");
                    break;
                },
                Err(err) => panic!("{}", err),
            }
        }
    }
    pub fn exec_command(&self, command: &str) -> Result<(), CliErr> {
        let (ops, arg) = command.split_once(' ').unwrap_or((command, ""));
        let ops = ops.parse::<SubCommand>()?;

        let date = parse_to_date(&arg);
        match ops {
            SubCommand::Add => {
                let date = date?;
                let initial = self.conn.read_day(date)
                    .map_or(String::from(""), |day| day.event().instruct.clone());
                let initial = edit_with_editor(&initial)?;
                let day = Day::from_event(&initial.parse::<Event>().unwrap())
                    .with_date(date);
                self.conn.add_day(day)?;
            },
            SubCommand::Remove => {
                let date = date?;
                self.conn.remove_day(date)?;
            },
            SubCommand::Check => {
                let date = date?;
                let initial = self.conn.read_day(date)
                    .map_or(String::from(""), |day| day.event().instruct.clone());
                println!("{}", initial);
            },
            SubCommand::ListAll => {
                let days = self.conn.read_all()?;
                days.iter().for_each(|day| println!("{}", day));
            },
            SubCommand::Quit => {
                return Err(CliErr::Exit)
            }
            SubCommand::Help => {
                println!(r#"Available commands: add, delete, check, ls, help"#);
            },
            _ => {
                unreachable!()
            }
        }

        Ok(())
    }
}
pub fn parse_to_date(source: &str) -> Result<time::Date, CliErr> {
    let source = source.trim();
    let today = time::OffsetDateTime::now_utc().date();

    match source {
        "yesterday" | "y" => Ok(today.previous_day().unwrap()),
        "tomorrow" | "m" => Ok(today.next_day().unwrap()),
        "today" | "t" => Ok(today),
        _ => {
            let f1 = time::macros::format_description!("[year]-[month]-[day]");
            let f2 = time::macros::format_description!("[year][month][day]");
            time::Date::parse(source, f1).or_else(|_| {
                time::Date::parse(source, f2)
            }).map_err(|e| CliErr::InvalidDate(e.to_string()))
        }
    }

}
fn edit_with_editor(s: &str) -> Result<String, CliErr> {
    let mut editor = tempfile::NamedTempFile::new()?;
    editor.write_all(s.as_bytes())?;
    editor.flush()?;
    editor.seek(SeekFrom::Start(0))?;

    edit_file(editor.path())?;


    let mut res = String::new();
    editor.read_to_string(&mut res)?;
    Ok(res)
}
pub fn edit_file(file: &Path) -> std::io::Result<ExitStatus> {
    std::process::Command::new(args::editor())
        .arg(file)
        .status()
}
pub fn cat_file(file: &Path) -> std::io::Result<ExitStatus> {
    std::process::Command::new("cat")
        .arg(file)
        .status()
}
pub enum Mode {
    Interactive,
    Once,
}

#[derive(Debug)]
pub enum CliErr {
    Exit,
    Io(std::io::Error),
    Db(rusqlite::Error),
    InvalidDate(String),
    UnknownCommand(String),
}

impl From<Parse> for CliErr {
    fn from(value: Parse) -> Self {
        CliErr::InvalidDate(value.to_string())
    }
}

impl From<rusqlite::Error> for CliErr {
    fn from(err: rusqlite::Error) -> Self {
        CliErr::Db(err)
    }
}
impl From<std::io::Error> for CliErr {
    fn from(err: std::io::Error) -> Self {
        CliErr::Io(err)
    }
}
#[derive(Debug, Clone, Copy)]
enum SubCommand {
    Add,
    Remove,
    Check,
    ListAll,
    Quit,
    Help,
}
impl FromStr for SubCommand {
    type Err = CliErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ad" | "add" => Ok(SubCommand::Add),
            "rm" | "remove" | "delete" | "del" => Ok(SubCommand::Remove),
            "chk" | "check" | "read" | "show" => Ok(SubCommand::Check),
            "ls" | "list" => Ok(SubCommand::ListAll),
            "h" | "help" => Ok(SubCommand::Help),
            "quit" | "exit" | "q" => Ok(SubCommand::Quit),
            _ => Err(CliErr::UnknownCommand(s.to_string())),
        }
    }
}