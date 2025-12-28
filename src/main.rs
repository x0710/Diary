use std::error::Error;
use rusqlite::Connection;

const DB_NAME: &str = "diary.db";

mod model;
mod storage;
mod interface;
use crate::interface::cli::terminal::CliSession;

fn main() -> Result<(), Box<dyn Error>> {
    let prjdir = directories::ProjectDirs::from("x0710", "x0710", "diary")
        .expect("Could not find a valid home directory");
    let data_dir = prjdir.data_dir();
    std::fs::create_dir_all(&data_dir).unwrap();
    let db_path = data_dir.join(DB_NAME);
    let db = Connection::open(db_path)?;

    let cli = CliSession::new(db);
    cli.run();

    Ok(())
}
