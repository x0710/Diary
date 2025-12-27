use std::error::Error;
use clap::CommandFactory;
use clap::error::ErrorKind;
use clap::Parser;

use crate::cli::args::Args;
use crate::cli::run::Mode::{Interactive, Once};
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
        let formatter = time::macros::format_description!("[year]-[month]-[day]");
        let date = time::Date::parse(self.args.time.as_deref().unwrap(), &formatter).unwrap();
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
        unimplemented!()
    }
}
pub enum Mode {
    Interactive,
    Once,
}