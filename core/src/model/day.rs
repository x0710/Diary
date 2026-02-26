use std::fmt::Display;
use std::ops::Deref;
use crate::model::Event;
use crate::base::date::Date;

/// 某一天的完整记录(DTO)
#[derive(Debug, Clone)]
pub struct Day {
    pub date: Date,
    pub event: Event,
    pub weather: Option<String>,
    pub mood: Option<String>,
}
impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Date: {}, {}]", self.date.deref(), self.date.weekday())?;
        if let Some(w) = &self.weather { write!(f, " [Weather: {}]", w)?; }
        if let Some(m) = &self.mood { write!(f, " [Mood: {}]", m)?; }
        write!(f, "\nEvent: {}", self.event)
    }
}
impl Default for Day {
    fn default() -> Self {
        let now = crate::base::date::Date::native_time();
        Self {
            date: now,
            event: Event::default(),
            weather: None,
            mood: None,
        }
    }
}