use std::path::PathBuf;
use rusqlite::Connection;
use crate::base::error::Error;
use crate::storage::db_mgr::DatabaseManager;

pub const DEFAULT_DB_NAME: &str = "diary.db";
pub fn default_project_path() -> PathBuf {
    let prjdir = directories::ProjectDirs::from("x0710", "x0710", "diary")
        .expect("Could not find a valid home directory");
    let data_dir = prjdir.data_dir();
    std::fs::create_dir_all(&data_dir)
        .expect("Could not create data directory");
    data_dir.to_path_buf()
}
pub fn open_with_default_database() -> Result<DatabaseManager, Error> {
    let base_dir = default_project_path();
    let db_path = base_dir.join(DEFAULT_DB_NAME);

    let conn = Connection::open(db_path).map_err(|e| Error::from(e))?;
    Ok(conn.try_into()?)
}
pub fn open_with_db_file(
    db_path: PathBuf,
) -> Result<DatabaseManager, Error> {
    let conn = Connection::open(db_path).map_err(|e| Error::from(e))?;
    Ok(conn.try_into()?)
}