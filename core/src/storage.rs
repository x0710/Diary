use std::ops::{Deref, DerefMut};
use sqlx::{Connection, Executor, Row, SqliteConnection};
use sqlx::sqlite::SqliteRow;
use crate::model::Day;
use crate::base::date::Date;
use crate::base::date::DATE_FORMAT1;
use crate::base::error::Error;

pub struct DatabaseManager {
    pub(crate) conn: SqliteConnection,
}
impl DatabaseManager {
    // pub fn transaction(&mut self) ->  Result<Transaction<'_>> {
    //     self.conn.transaction()
    // }
    pub fn from_path(path: &std::path::Path) -> Result<Self, Error> {
        let init_query = sqlx::query(r"
        CREATE TABLE IF NOT EXISTS day (
            date TEXT NOT NULL PRIMARY KEY,
            event TEXT,
            weather TEXT,
            mood TEXT)
        ");
        let conn = async_std::task::block_on(async {
            let mut conn = SqliteConnection::connect(path.to_str().unwrap()).await?;
            conn.execute(init_query).await?;
            Ok::<SqliteConnection, Error>(conn)
        })?;
        Ok(Self {
            conn
        })
    }
    pub async fn remove_day(&mut self, date: Date) -> Result<u64, Error> {
        let query = sqlx::query("DELETE FROM day WHERE date = ?")
            .bind(date.format(DATE_FORMAT1).unwrap());
        Ok(self.conn.execute(query).await?.rows_affected())
    }
    pub async fn read_all(&mut self) -> Result<Vec<Day>, Error> {
        let query = sqlx::query("SELECT date, event, weather, mood FROM day ORDER BY date ASC");
        Ok(self.conn.fetch_all(query).await?.iter()
            .map(Into::into)
            .collect())
    }
    pub async fn read_from_to(&mut self, from: Date, to: Date) -> Result<Vec<Day>, Error> {
        let query = sqlx::query("SELECT date,event,weather,mood FROM day WHERE date BETWEEN ?1 AND ?2")
            .bind(from.format(DATE_FORMAT1).unwrap())
            .bind(to.format(DATE_FORMAT1).unwrap());
        Ok(self.conn.fetch_all(query).await?.iter()
            .map(Into::into)
            .collect())
    }
    pub async fn read_day(&mut self, date: Date) -> Result<Option<Day>, Error> {
        let query = sqlx::query("SELECT date,event,weather,mood FROM day WHERE date = ?")
            .bind(date.format(DATE_FORMAT1).unwrap());
        Ok(self.conn.fetch_optional(query).await?.map(|t| (&t).into()))
    }
    pub async fn add_day(&mut self, day: &Day) -> Result<u64, Error> {
        let query = sqlx::query("INSERT OR REPLACE INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)")
            .bind(day.date.format(DATE_FORMAT1).unwrap())
            .bind(&day.event.instruct)
            .bind(day.weather.as_deref())
            .bind(day.mood);
        Ok(self.conn.execute(query).await?.rows_affected())
    }
}
impl DerefMut for DatabaseManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}
impl Deref for DatabaseManager {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}
impl From<&SqliteRow> for Day {
    fn from(row: &SqliteRow) -> Self {
        // raw_datum
        let date_raw: String = row.get("date");
        let date = time::Date::parse(&date_raw, DATE_FORMAT1).unwrap();
        let event_str: String = row.get("event");
        let weather = row.get("weather");
        let mood = row.get("mood");
        // Obj
        Day {
            date: date.into(),
            event: event_str.into(),
            weather,
            mood,
        }
    }
}
