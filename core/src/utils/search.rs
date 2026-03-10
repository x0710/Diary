use regex::Regex;
use sqlx::{FromRow, QueryBuilder, Sqlite};
use sqlx::sqlite::SqliteRow;
use crate::base::date::Date;
use crate::base::error::Error;
use crate::model::Day;
use crate::storage::DatabaseManager;
pub struct SearchCondition {
    keyword: Option<String>,
    regex: Option<Regex>,

    date_from: Option<Date>,
    date_to: Option<Date>,
    weather_like: Option<String>,
    mood_from: Option<i32>,
    mood_to: Option<i32>,
}
impl SearchCondition {
    pub fn with_keyword<S: Into<String>>(mut self, keyword: S) -> Self {
        self.keyword = Some(keyword.into());
        self
    }
    pub fn with_regex(mut self, regex: Regex) -> Self {
        self.regex = Some(regex);
        self
    }
    pub fn with_date_from(mut self, date_from: Date) -> Self {
        self.date_from = Some(date_from);
        self
    }
    pub fn with_date_to(mut self, date_to: Date) -> Self {
        self.date_to = Some(date_to);
        self
    }
    pub fn with_weather_like<S: Into<String>>(mut self, weather_like: S) -> Self {
        self.weather_like = Some(weather_like.into());
        self
    }
    pub fn with_mood_from(mut self, mood_from: i32) -> Self {
        self.mood_from = Some(mood_from);
        self
    }
    pub fn with_mood_to(mut self, mood_to: i32) -> Self {
        self.mood_to = Some(mood_to);
        self
    }
}
impl Default for SearchCondition {
    fn default() -> Self {
        Self {
            keyword: None,
            regex: None,
            date_from: None,
            date_to: None,
            weather_like: None,
            mood_from: None,
            mood_to: None,
        }
    }
}
impl DatabaseManager {
    pub async fn search_in_condition(&mut self, condition: SearchCondition) -> Result<Vec<Day>, Error> {
        let mut query: QueryBuilder<'_, Sqlite> = QueryBuilder::new("SELECT date, event, weather, mood FROM day WHERE 1=1");
        if let Some(keyword) = condition.keyword {
            query.push(" AND event like ");
            query.push_bind(format!("%{}%", keyword).to_string());
        }
        if let Some(regex) = condition.regex {
            query.push(" AND event regex ");
            query.push_bind(regex.to_string());
        }
        if let Some(date_from) = condition.date_from {
            query.push(" AND date >= ");
            query.push_bind(date_from.to_string());
        }
        if let Some(date_to) = condition.date_to {
            query.push(" AND date <= ");
            query.push_bind(date_to.to_string());
        }
        if let Some(weather_like) = condition.weather_like {
            query.push(" AND weather like ");
            query.push_bind(format!("%{}%", weather_like).to_string());
        }
        if let Some(mood_from) = condition.mood_from {
            query.push(" AND mood >= ");
            query.push_bind(mood_from);
        }
        if let Some(mood_to) = condition.mood_to {
            query.push(" AND mood <= ");
            query.push_bind(mood_to);
        }
        query.push(" ORDER BY ");
        query.push_bind("date DESC ");
        let query = query.build_query_as();
        let res = query.fetch_all(&mut self.conn).await?;
        Ok(res)
    }
}
impl FromRow<'_, SqliteRow> for Day {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(row.into())
    }
}
impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::InvalidData(err.to_string())
    }
}
