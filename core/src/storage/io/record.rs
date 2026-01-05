use serde::{Deserialize, Serialize};
use crate::base::error::Error;
use crate::model::day::Day;
use crate::model::event::Event;

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
        let date = record.date.parse()?;
        let event = record.event;
        Ok(Day::new(date, Event::new(&event), record.weather, record.mood))
    }
}
