use std::path::PathBuf;
use crate::base::error::Error;
use crate::db::DatabaseManager;

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

    open_with_db_file(db_path)
}
pub fn open_with_db_file(
    db_path: PathBuf,
) -> Result<DatabaseManager, Error> {
    if !db_path.exists() {
        std::fs::File::create(&db_path)?;
    }
    Ok(DatabaseManager::from_path(&db_path)?)
}
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}