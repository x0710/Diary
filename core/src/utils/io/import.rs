use std::fs::File;
use std::ops::DerefMut;
use std::path::Path;
use sqlx::{Connection, Executor};
use crate::base::date::DATE_FORMAT1;
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
    pub async fn import_to_db(&mut self, data: Vec<Day>, mode: DuplicateStrategy) -> Result<(), Error> {
        match mode {
            DuplicateStrategy::Replace => {
                for r in &data { self.db_mgr.add_day(r).await?; }
            }
            DuplicateStrategy::Ignore => {
                for r in data {
                    let query = sqlx::query("INSERT OR IGNORE INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)");
                    query.bind(r.date.format(DATE_FORMAT1).unwrap())
                        .bind(r.event.instruct)
                        .bind(r.weather)
                        .bind(r.mood)
                        .execute(self.db_mgr.deref_mut()).await?;

                }

            }
            DuplicateStrategy::Fail => {
                let mut bg = self.db_mgr.begin().await?;
                for r in data {
                    let query = sqlx::query("INSERT INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)")
                        .bind(r.date.format(DATE_FORMAT1).unwrap())
                        .bind(r.event.instruct)
                        .bind(r.weather)
                        .bind(r.mood);
                    let _ = query.execute(&mut *bg).await?;
                }
                bg.commit().await?;
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