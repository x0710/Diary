use std::str::FromStr;
use time::error::Parse;
use time::format_description::BorrowedFormatItem;
use crate::terminal;
use diary_core::model::day::Day;
use diary_core::model::event::Event;
use diary_core::storage::db_mgr::DatabaseManager;

pub struct Executor {
    conn: DatabaseManager,
}
impl Executor {
    pub fn new(conn: DatabaseManager) -> Executor {
        Self { conn }
    }
    pub fn exec_command(&self, command: &str) -> Result<(), CliErr> {
        let (ops, arg) = command.split_once(' ').unwrap_or((command, ""));
        let ops = ops.parse::<SubCommand>()?;

        let date = parse_to_date(&arg);
        match ops {
            SubCommand::Add => self.handle_add(date?),
            SubCommand::Remove => self.handle_del(date?),
            SubCommand::Check => self.handle_check(date?),
            SubCommand::ListAll => self.handle_list_all(),
            SubCommand::Quit => return Err(CliErr::Exit),
            SubCommand::Help => self.handle_help(),
        }?;

        Ok(())
    }
    fn handle_list_all(&self) -> Result<usize, CliErr> {
        let days = self.conn.read_all()?;
        days.iter().for_each(|day| println!("{}", day));
        Ok(0)
    }
    fn handle_check(&self, date: time::Date) -> Result<usize, CliErr> {
        let initial = self.conn.read_day(date)
            .map_or(String::from(""), |day| day.event().instruct.clone());
        println!("{}", initial);
        Ok(0)
    }
    fn handle_del(&self, date: time::Date) -> Result<usize, CliErr> {
        Ok(self.conn.remove_day(date)?)
    }
    fn handle_add(&self, date: time::Date) -> Result<usize, CliErr> {
        let initial = self.conn.read_day(date)
            .map_or(String::from(""), |day| day.event().instruct.clone());
        let initial = terminal::edit_with_editor(&initial)?;
        let day = Day::from_event(&initial.parse::<Event>().unwrap())
            .with_date(date);
        Ok(self.conn.add_day(day)?)
    }
    fn handle_help(&self) -> Result<usize, CliErr> {
        println!(r#"
Available commands:
  add <date>    - Add or edit an entry (e.g., add today, add 20251225, add 2025-12-25)
  remove <date> - Delete an entry
  check <date>  - View a specific entry
  list          - List all entries
  quit/exit     - Close the application
"#);
        Ok(0)
    }
}
const DATE_FORMAT1: &[BorrowedFormatItem<'static>] = time::macros::format_description!("[year]-[month]-[day]");
const DATE_FORMAT2: &[BorrowedFormatItem<'static>] = time::macros::format_description!("[year][month][day]");
pub fn parse_to_date(source: &str) -> Result<time::Date, CliErr> {
    let source = source.trim();
    let today = time::OffsetDateTime::now_utc().date();

    match source {
        "yesterday" | "y" => Ok(today.previous_day().unwrap()),
        "tomorrow" | "m" => Ok(today.next_day().unwrap()),
        "today" | "t" => Ok(today),
        _ => {
            time::Date::parse(source, DATE_FORMAT1).or_else(|_| {
                time::Date::parse(source, DATE_FORMAT2)
            }).map_err(|e| CliErr::InvalidDate(e.to_string()))
        }
    }

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