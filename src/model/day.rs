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
}
