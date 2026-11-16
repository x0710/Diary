use crate::base::date::DATE_FORMAT1;
use crate::base::error::Error;
use crate::model::Day;
use crate::storage::DatabaseManager;
use crate::utils::io::format::Format;
use crate::utils::io::model::Record;
use sqlx::Connection;
use std::fs::File;
use std::path::Path;

pub struct Importer<'a> {
    db_mgr: &'a mut DatabaseManager,
}
impl<'a> Importer<'a> {
    pub fn new(db_mgr: &'a mut DatabaseManager) -> Self {
        Importer { db_mgr }
    }
    pub fn read_from_file<P: AsRef<Path>>(
        path: P,
        format: Format,
    ) -> Result<(Vec<Day>, Vec<String>), Error> {
        let mut days = Vec::new();
        let mut errors = Vec::new();
        match format {
            Format::Json => {
                let res = serde_json::from_reader::<_, Vec<Record>>(File::open(&path)?);
                if let Ok(r) = res {
                    for i in r {
                        days.push(i.try_into()?);
                    }
                } else {
                    errors.push(res.unwrap_err().to_string());
                }
            }
            Format::Csv => {
                let mut csv_reader = csv::Reader::from_path(path)?;
                for r in csv_reader.deserialize::<Record>() {
                    if let Ok(record) = r {
                        days.push(record.try_into()?);
                    } else {
                        errors.push(r.unwrap_err().to_string());
                    }
                }
            }
        }
        Ok((days, errors))
    }
    pub async fn import_to_db(
        &mut self,
        data: Vec<Day>,
        mode: DuplicateStrategy,
    ) -> Result<(), Error> {
        let statement = match mode {
            DuplicateStrategy::Replace => {
                "INSERT INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(date) DO UPDATE SET event = excluded.event, weather = excluded.weather, mood = excluded.mood"
            }
            DuplicateStrategy::Ignore => {
                "INSERT INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(date) DO NOTHING"
            }
            DuplicateStrategy::Fail => {
                "INSERT INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)"
            }
            // Append nonempty text and retain existing metadata when the import omits it.
            DuplicateStrategy::Append => {
                "INSERT INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(date) DO UPDATE SET event = CASE WHEN COALESCE(day.event, '') = '' THEN excluded.event WHEN excluded.event = '' THEN day.event ELSE day.event || char(10) || excluded.event END, weather = COALESCE(excluded.weather, day.weather), mood = COALESCE(excluded.mood, day.mood)"
            }
        };
        let mut transaction = self.db_mgr.begin().await?;
        for day in data {
            sqlx::query(statement)
                .bind(day.date.format(DATE_FORMAT1).unwrap())
                .bind(day.event.instruct)
                .bind(day.weather)
                .bind(day.mood)
                .execute(&mut *transaction)
                .await?;
        }
        transaction.commit().await?;

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
