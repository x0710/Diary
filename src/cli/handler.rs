use std::error::Error;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::ExitStatus;
use std::str::FromStr;
use clap::Parser;
use rustyline::error::ReadlineError;
use tempfile::NamedTempFile;
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
    pub fn new(conn: DatabaseManager) -> Result<CliHandler, Box<dyn Error>> {
        let args = Args::parse();
        let mode = if args.interactive { Interactive } else {
            if args.time.is_none() {
                // Args::command().error(
                //     ErrorKind::MissingSubcommand,
                //     "Time is required for once mode"
                // ).exit()
                Interactive
            }else {Once}

        };
        Ok(Self {
            conn,
            args,
            mode,
        })
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
            let d = Day::new(
                date,
                event,
                None,
                None,
            );
            self.conn.add_day(&d).expect("Could not add day");
        }else {
            // check the day
            let res = self.conn.read_day(&date).expect("This day has not yet been recorded.");
            println!("{}", res);
        }
    }
    fn interactive(&self) {
        use rustyline::DefaultEditor;
        let mut rl = DefaultEditor::new().unwrap();
        loop {
            match rl.readline(">: ") {
                Ok(line) => {
                    match self.deal_command(line.as_str()) {
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
                        }
                        Err(CliErr::Exit) | Ok(_) => (),
                    }
                },
                Err(ReadlineError::Eof) => {
                    println!("Have a nice day!");
                    break;
                },
                Err(ReadlineError::Interrupted) => {
                    println!("Have a nice day!");
                    break;
                },
                Err(err) => panic!("{}", err),
            }
        }
    }
    fn deal_command(&self, command: &str) -> Result<(), CliErr> {
        let (ops, arg) = command.split_once(' ').unwrap_or((command, ""));
        let ops = ops.parse::<SubCommand>()?;
        let mut tmpfile = NamedTempFile::new()?;

        let date = parse_to_date(&arg);
        match ops {
            SubCommand::Add => {
                if let Some(date) = date {
                    if let Some(day) = self.conn.read_day(&date) {
                        tmpfile.write_all(day.event().instruct.as_bytes())?;
                        tmpfile.flush()?;
                        tmpfile.seek(SeekFrom::Start(0))?;
                    }
                    edit_file(tmpfile.path())?;
                        // .expect("Please Config your Default EDITOR!");
                    tmpfile.flush()?;
                    tmpfile.seek(SeekFrom::Start(0))?;
                    let mut data = String::new();
                    tmpfile.read_to_string(&mut data)?;
                    let day = Day::new(
                        date,
                        Event::new(&data),
                        None,
                        None,
                    );
                    self.conn.add_day(&day)?;
                }else {
                    println!("Please input correct time");
                }
            },
            SubCommand::Remove => {
                if let Some(date) = date {
                    self.conn.remove_day(&date)?;
                }else {
                    println!("Please input correct date");
                }
            },
            SubCommand::Check => {
                if let Some(date) = date {
                    if let Some(date) = self.conn.read_day(&date) {
                        tmpfile.write_all(date.event().instruct.as_bytes())?;
                        tmpfile.flush()?;
                        tmpfile.seek(SeekFrom::Start(0))?;
                        cat_file(tmpfile.path())?;
                    }else {
                        println!("There was no such day.");
                    }
                }else {
                    println!("Please input correct date");
                }
            },
            SubCommand::ListAll => {
                unimplemented!()
            },
            SubCommand::Quit => {
                return Err(CliErr::Exit)
            }
            SubCommand::Help => {
                println!(r#"Available commands: ad, rm, chk, ls, h"#);
            },
            _ => {
                unreachable!()
            }
        }

        Ok(())
    }
}
/*
impl FromStr for Date {
    type Err = time::error::Parse;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let f1 = time::macros::format_description!("[year]-[month]-[day]");
        let f2 = time::macros::format_description!("[year][month][day]");

        Date::parse(source, &f1)
            .or_else(|_| time::Date::parse(source, &f2))
    }
}
 */
pub fn parse_to_date(source: &str) -> Option<time::Date> {
    let f1 = time::macros::format_description!("[year]-[month]-[day]");
    let f2 = time::macros::format_description!("[year][month][day]");

    time::Date::parse(source, &f1).or_else(|_| time::Date::parse(source, &f2)).ok()
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

enum CliErr {
    Exit,
    Io(std::io::Error),
    Db(rusqlite::Error),
    InvalidDate(String),
    UnknownCommand(String),
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
            "rm" | "remove" => Ok(SubCommand::Remove),
            "chk" | "check" => Ok(SubCommand::Check),
            "ls" | "list" => Ok(SubCommand::ListAll),
            "h" | "help" => Ok(SubCommand::Help),
            "quit" | "exit" | "q" => Ok(SubCommand::Quit),
            _ => Err(CliErr::UnknownCommand(s.to_string())),
        }
    }
}