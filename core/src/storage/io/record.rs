use serde::{Deserialize, Serialize};
use crate::base::date::DATE_FORMAT1;
use crate::base::error::Error;
use crate::model::day::Day;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub date: String,
    pub event: String,
    pub weather: Option<String>,
    pub mood: Option<String>,
}

impl TryFrom<Record> for Day {
    type Error = Error;
    fn try_from(record: Record) -> Result<Self, Self::Error> {
        let date = time::Date::parse(record.date.as_str(), DATE_FORMAT1)?;
        let event = record.event;
        Ok(Self {
            date: date.into(),
            event: event.into(),
            weather: record.weather,
            mood: record.mood,
        })
    }
}
impl From<Day> for Record {
    fn from(value: Day) -> Self {
        Self {
            date: value.date.format(DATE_FORMAT1).unwrap().to_string(),
            event: value.event.instruct,
            weather: value.weather,
            mood: value.mood,
        }
    }
}
