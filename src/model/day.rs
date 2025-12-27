use std::fmt::Display;
use crate::model::event::Event;

#[derive(Debug)]
pub struct Day {
    date: time::Date,
    event: Event,
    weather: Option<String>,
    mood: Option<String>,
}
impl Day {
    pub fn mood(&self) -> Option<&str> {
        self.mood.as_deref()
    }
    pub fn weather(&self) -> Option<&str> {
        self.weather.as_deref()
    }
    pub fn event(&self) -> &Event {
        &self.event
    }
    pub fn date(&self) -> &time::Date {
        &self.date
    }
    pub fn new(
        date: time::Date,
        event: Event,
        weather: Option<String>,
        mood: Option<String>,
    ) -> Day {
        Day {
            date,
            event,
            weather,
            mood,
        }
    }
    pub fn from_event(date: &time::Date, event: &Event) -> Self {
        Self::new(
            date.clone(),
            event.clone(),
            None,
            None,
        )
    }
}
impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"date: {}, weather: {}, mood: {}, Event: {}"#,
            self.date,
            self.weather.as_deref().map_or_else(|| "".to_string(), |w| format!(", weather: {}", w)),
            self.mood.as_deref().map_or_else(|| "".to_string(), |w| format!(", mood: {}", w)),
            self.event,
        )
    }
}
