use std::fs::File;
use std::path::Path;
use rusqlite::params;
use crate::base::error::Error;
use crate::model::Day;
use crate::storage::DatabaseManager;
use crate::utils::io::format::Format;
use crate::utils::io::model::Record;

pub struct Importer<'a> {
    db_mgr: &'a mut DatabaseManager,
}
impl<'a> Importer<'a> {
    pub fn new(db_mgr: &'a mut DatabaseManager) -> Self {
        Importer { db_mgr, }
    }
    pub fn read_from_file<P: AsRef<Path>>(path: P, format: Format) -> Result<(Vec<Day>, Vec<String>), Error> {
        let mut days = Vec::new();
        let mut errors = Vec::new();
        match format {
            Format::Json => {
                let res = serde_json::from_reader::<_, Vec<Record>>(File::open(&path)?);
                if let Ok(r) = res {
                    for i in r {
                        days.push(i.try_into()?);
                    }
                }else {
                    errors.push(res.unwrap_err().to_string());
                }

            }
            Format::Csv => {
                let mut csv_reader = csv::Reader::from_path(path)?;
                for r in csv_reader.deserialize::<Record>() {
                    if let Ok(record) = r {
                        days.push(record.try_into()?);
                    }else {
                        errors.push(r.unwrap_err().to_string());
                    }
                }

            }
        }
        Ok((days, errors))
    }
    pub fn import_to_db(&mut self, data: Vec<Day>, mode: DuplicateStrategy) -> Result<(), Error> {
        match mode {
            DuplicateStrategy::Replace => {
                for r in &data { self.db_mgr.add_day(r)?; }
            }
            DuplicateStrategy::Ignore => {
                let mut stmt = self.db_mgr.connection().prepare("INSERT OR IGNORE INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)")?;
                for r in data {
                    stmt.execute(params![r.date.to_string(), r.event.instruct, r.weather, r.mood])?;
                }

            }
            DuplicateStrategy::Fail => {
                let tx = self.db_mgr.transaction()?;
                let mut stmt = tx.prepare("INSERT INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)")?;
                for r in data {
                    let ret = stmt.execute(params![r.date.to_string(), r.event.instruct, r.weather, r.mood]);
                    if let Err(e) = ret {
                        // Drop change
                        return Err(e.into());
                    }
                }
                drop(stmt);
                tx.commit()?;
            }
            DuplicateStrategy::Append => {todo!()}
        }

        Ok(())
    }
}
#[derive(Debug, Clone)]
pub enum DuplicateStrategy {
    Replace,
    Ignore,
    Fail,
    Append,
}