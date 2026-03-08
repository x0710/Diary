use regex::Regex;
use crate::base::date::Date;
use crate::base::error::Error;
use crate::storage::DatabaseManager;

/*
pub struct Searcher {
    pub(crate) db_mgr: DatabaseManager,
}
impl Searcher {
    pub fn new(db_mgr: DatabaseManager) -> Self {
        Searcher { db_mgr }
    }
    pub fn regex(&self, regex_str: &str) -> Result<Vec<Date>, Error> {
        let regex = RegexBuilder::new(regex_str).build()?;
        let res = self.db_mgr.conn.query_row_and_then(
            "SELECT date",
            params![regex_str],
            |row| {
                row.get(0)
            }
        )

        todo!()
    }
}
 */
impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::InvalidData(err.to_string())
    }
}
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
    fn search(&self, query: &SearchCondition) -> Result<Vec<Date>, Error> {
        todo!()
    }
}