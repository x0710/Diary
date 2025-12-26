use std::error::Error;
use time::{Date, Month};
use crate::model::day::Day;
use crate::model::event::Event;
use crate::storage::db_mgr::DatabaseManager;

const DB_NAME: &str = "diary.db";

mod model;
mod storage;
mod cli;

fn main() -> Result<(), Box<dyn Error>>{
    let prjdir = directories::ProjectDirs::from("x0710", "x0710", "diary")
        .expect("Could not find a valid home directory");
    let data_dir = prjdir.data_dir();
    std::fs::create_dir_all(&data_dir)?;
    let db_path = data_dir.join(DB_NAME);
    let db_mgr = DatabaseManager::with_path(db_path.to_str()
        .expect("Could not get database path"))?;

    let the_day = time::Date::from_calendar_date(2025, Month::February, 1).unwrap();
    let day = Day::new(
        the_day,
        Event::new("Doing homework".to_string()),
        None,
        None,
    );
    db_mgr.add_day(day)?;
    let trg = db_mgr.read_day(the_day);
    if let Some(t) = trg {
        println!("You find the day, {}", t.date());
    }else {
        println!("Could Not find the day");
    }
    Ok(())
}
