use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::base::date::Date;
use crate::base::error::Error;
use crate::model::day::Day;
use crate::storage::db_mgr::DatabaseManager;
use crate::storage::io::mode::Format;

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

    pub fn export_from_to(&mut self, from: Date, to: Date) -> Result<(), Error> {
        unimplemented!("Import from {:?} to {:?}", from, to);
        // let res = self.db_mgr.read_from_to(from, to)?.into_iter().map(|x| x.into_record()).collect::<Vec<_>>();
        // self.export_to_file(res)?;
        Ok(())
    }
    pub fn all_export(&mut self) -> Result<(), Error> {
        let res = self.db_mgr.read_all()?;
        self.export(res)?;
        Ok(())
    }
    pub fn export(&mut self, days: Vec<Day>) -> Result<(), Error> {
        let days = days.into_iter().map(|x| x.into_record()).collect::<Vec<_>>();
        match self.mode {
            Format::CSV => {
                let mut csv_writer = csv::Writer::from_path(&self.path)?;
                for record in days { csv_writer.serialize(record)?; }
                csv_writer.flush()?;
            },
            Format::JSON => {
                let json = serde_json::to_string(&days).unwrap();
                File::create(&self.path)?.write_all(json.as_bytes())?;
            }
        }

        Ok(())
    }
}
#[test]
fn write_to_csv() {
    let db_mgr = DatabaseManager::from_path(Path::new("/home/x0710/.local/share/diary/diary.db")).unwrap();
    let out = PathBuf::from("/tmp/test.csv");
    let mut e = Exporter::new(&db_mgr, out.into_boxed_path(), Format::CSV);
    // e.from_to(Date::new(2020, 1, 1).unwrap(), Date::new(2025, 12, 12).unwrap()).unwrap();

    e.all_export().unwrap();
}
#[test]
fn write_to_json() {
    let db_mgr = DatabaseManager::from_path(Path::new("/home/x0710/.local/share/diary/diary.db")).unwrap();
    let out = PathBuf::from("/tmp/test.json");
    let mut e = Exporter::new(&db_mgr, out.into_boxed_path(), Format::JSON);
    // e.from_to(Date::new(2020, 1, 1).unwrap(), Date::new(2025, 12, 12).unwrap()).unwrap();

    e.all_export().unwrap();
}
