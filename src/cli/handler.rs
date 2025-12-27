use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::ExitStatus;
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
                    if self.deal_command(line.as_str()).is_err() {
                        break;
                    }
                }
                Err(ReadlineError::Eof) => {
                    println!("Have a nice day!");
                    break;
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Have a nice day!");
                    break;
                }
                Err(err) => panic!("{}", err),
            }
        }
    }
    fn deal_command(&self, command: &str) -> Result<(), Box<dyn Error>> {
        let (ops, arg) = command.split_once(' ').unwrap_or((command, ""));
        let mut tmpfile = NamedTempFile::new()?;

        let date = parse_to_date(&arg);
        match ops {
            "ad" | "add" => {
                if let Some(date) = date {
                    self.conn.read_day(&date).map(|day| {
                        tmpfile.write_all(day.event().instruct.as_bytes()).unwrap();
                        tmpfile.flush().unwrap();
                        tmpfile.seek(SeekFrom::Start(0)).unwrap()
                    });
                    edit_file(tmpfile.path())
                        .expect("Please Config your Default EDITOR!");
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
                    self.conn.add_day(&day).unwrap();
                }else {
                    println!("Please input correct time");
                }
            },
            "rm" | "remove" => {
                if let Some(date) = date {
                    self.conn.remove_day(&date)?;
                }else {
                    println!("Please input correct date");
                }
            },
            "check" | "chk" => {
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
            "ls" | "list" => {
                unimplemented!()
            },
            "quit" | "exit" | "q" => {
                return Err(Box::new(ExitNormally {}));
            }
            "h" | "help" => {
                println!(r#"Available commands: ad, rm, chk, ls, h"#);
            },
            _ => {
                println!("Unknown command: `{}`, to get more helps, please input `help` or `h`.", ops);
            }
        }

        Ok(())
    }
}
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
struct ExitNormally;
impl Debug for ExitNormally { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { unreachable!() } }
impl Display for ExitNormally { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { unreachable!() } }
impl Error for ExitNormally {}