use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::base::error::Error;
use crate::model::Day;
use crate::storage::DatabaseManager;
use crate::utils::io::format::Format;
use crate::utils::io::model::Record;

pub struct Exporter<'a> {
    db_mgr: &'a DatabaseManager,
    path: PathBuf,
    mode: Format,
}
impl<'a> Exporter<'a> {
    pub fn new(db_mgr: &'a DatabaseManager, export_path: impl AsRef<Path>, mode: Format) -> Self {
        let path = export_path.as_ref().to_path_buf();
        Self {
            db_mgr,
            path,
            mode,
        }
    }

    pub fn all_export(&mut self) -> Result<(), Error> {
        let res = self.db_mgr.read_all()?;
        self.export(res)?;
        Ok(())
    }
    pub fn export(&mut self, days: Vec<Day>) -> Result<(), Error> {
        let days: Vec<Record> = days.into_iter().map(|x| x.into()).collect::<Vec<_>>();
        match self.mode {
            Format::Csv => {
                let mut csv_writer = csv::Writer::from_path(&self.path)?;
                for record in days { csv_writer.serialize(record)?; }
                csv_writer.flush()?;
            }
            Format::Json => {
                let json = serde_json::to_string(&days).unwrap();
                File::create(&self.path)?.write_all(json.as_bytes())?;
            }
        }

        Ok(())
    }
}
