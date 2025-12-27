use std::error::Error;
use time::{Date, Month};
use crate::cli::handler::CliHandler;
use crate::model::day::Day;
use crate::model::event::Event;
use crate::storage::db_mgr::DatabaseManager;

const DB_NAME: &str = "diary.db";

mod model;
mod storage;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let prjdir = directories::ProjectDirs::from("x0710", "x0710", "diary")
        .expect("Could not find a valid home directory");
    let data_dir = prjdir.data_dir();
    std::fs::create_dir_all(&data_dir).unwrap();
    let db_path = data_dir.join(DB_NAME);
    let db_mgr = DatabaseManager::with_path(db_path.to_str()
        .expect("Could not get database path")).unwrap();

    let cli = CliHandler::new(db_mgr)?;
    cli.run();

    Ok(())
}
