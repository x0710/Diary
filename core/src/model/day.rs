use std::fmt::Display;
use crate::model::event::Event;
use crate::base::date::Date;

#[derive(Debug)]
pub struct Day {
    date: Date,
    event: Event,
    weather: Option<Box<str>>,
    mood: Option<Box<str>>,
}
impl Day {
    pub fn new(
        date: Date,
        event: Event,
        weather: Option<Box<str>>,
        mood: Option<Box<str>>,
    ) -> Day {
        Day {
            date,
            event,
            weather,
            mood,
        }
    }
    pub fn from_event(event: &Event) -> Self {
        Self {
            event: event.clone(),
            ..Self::default()
        }
    }
    pub fn from_date(date: Date) -> Self {
        Self {
            date: date,
            ..Self::default()
        }
    }
    pub fn with_event(mut self, event: Event) -> Self {
        self.event = event.clone();
        self
    }
    pub fn with_date(mut self, date: Date) -> Self {
        self.date = date;
        self
    }
    pub fn with_weather(mut self, w: impl Into<Box<str>>) -> Self {
        self.weather = Some(w.into());
        self
    }
    pub fn with_mood(mut self, m: impl Into<Box<str>>) -> Self {
        self.mood = Some(m.into());
        self
    }
    pub fn mood(&self) -> Option<&str> { self.mood.as_deref() }
    pub fn weather(&self) -> Option<&str> { self.weather.as_deref() }
    pub fn event(&self) -> &Event { &self.event }
    pub fn date(&self) -> Date { self.date }
}
impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Date: {}, {}]", self.date.date(), self.date.date().weekday())?;
        if let Some(w) = &self.weather { write!(f, " [Weather: {}]", w)?; }
        if let Some(m) = &self.mood { write!(f, " [Mood: {}]", m)?; }
        write!(f, "\nEvent: {}", self.event)
    }
}
impl Default for Day {
    fn default() -> Self {
        let now = time::OffsetDateTime::now_utc();
        Self {
            date: now.date().into(),
            event: Event::default(),
            weather: None,
            mood: None,
        }
    }
}