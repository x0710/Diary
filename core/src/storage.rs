use crate::base::date::DATE_FORMAT1;
use crate::base::date::Date;
use crate::base::error::Error;
use crate::model::Day;
use sqlx::sqlite::{SqliteConnectOptions, SqliteRow};
use sqlx::{Connection, Executor, FromRow, Row, SqliteConnection};
use std::ops::{Deref, DerefMut};

pub struct DatabaseManager {
    pub(crate) conn: SqliteConnection,
}
impl DatabaseManager {
    pub fn from_path(path: &std::path::Path) -> Result<Self, Error> {
        let init_query = sqlx::query(
            r"
        CREATE TABLE IF NOT EXISTS day (
            date TEXT NOT NULL PRIMARY KEY,
            event TEXT,
            weather TEXT,
            mood DOUBLE)
        ",
        );
        let rt = tokio::runtime::Builder::new_current_thread().build()?;
        let conn = rt.block_on(async {
            let mut conn = SqliteConnection::connect_with(
                &SqliteConnectOptions::new()
                    .filename(path)
                    .create_if_missing(true),
            )
            .await?;
            conn.execute(init_query).await?;
            Ok::<SqliteConnection, Error>(conn)
        })?;
        Ok(Self { conn })
    }
    pub async fn remove_day(&mut self, date: Date) -> Result<u64, Error> {
        let query =
            sqlx::query("DELETE FROM day WHERE date = ?").bind(date.format(DATE_FORMAT1).unwrap());
        Ok(self.conn.execute(query).await?.rows_affected())
    }
    pub async fn read_all(&mut self) -> Result<Vec<Day>, Error> {
        let query = sqlx::query("SELECT date, event, weather, mood FROM day ORDER BY date ASC");
        Ok(self
            .conn
            .fetch_all(query)
            .await?
            .iter()
            .map(Day::from_row)
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn read_from_to(&mut self, from: Date, to: Date) -> Result<Vec<Day>, Error> {
        let query = sqlx::query("SELECT date,event,weather,mood FROM day WHERE date BETWEEN ?1 AND ?2 ORDER BY date ASC")
            .bind(from.format(DATE_FORMAT1).unwrap())
            .bind(to.format(DATE_FORMAT1).unwrap());
        Ok(self
            .conn
            .fetch_all(query)
            .await?
            .iter()
            .map(Day::from_row)
            .collect::<Result<Vec<_>, _>>()?)
    }
    pub async fn read_day(&mut self, date: Date) -> Result<Option<Day>, Error> {
        let query = sqlx::query("SELECT date,event,weather,mood FROM day WHERE date = ?")
            .bind(date.format(DATE_FORMAT1).unwrap());
        Ok(self
            .conn
            .fetch_optional(query)
            .await?
            .as_ref()
            .map(Day::from_row)
            .transpose()?)
    }
    pub async fn add_day(&mut self, day: &Day) -> Result<u64, Error> {
        let query = sqlx::query(
            "INSERT OR REPLACE INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)",
        )
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
impl FromRow<'_, SqliteRow> for Day {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let date_raw: String = row.try_get("date")?;
        let date = time::Date::parse(&date_raw, DATE_FORMAT1).map_err(|source| {
            sqlx::Error::ColumnDecode {
                index: "date".into(),
                source: Box::new(source),
            }
        })?;
        let event: Option<String> = row.try_get("event")?;
        Ok(Day {
            date: date.into(),
            event: event.unwrap_or_default().into(),
            weather: row.try_get("weather")?,
            mood: row.try_get("mood")?,
        })
    }
}
