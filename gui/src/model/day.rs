//! Gui 当天实体模型
use diary_core::base::date::Date;
use diary_core::model::{Day, Event};

pub const DEFAULT_MOOD: f64 = 0.0;

pub struct GuiDayState {
    pub(crate) date: Date,
    pub(crate) event: Event,
    pub(crate) weather: String,
    pub(crate) mood: f64,
}
impl From<Day> for GuiDayState {
    fn from(day: Day) -> Self {
        Self {
            date: day.date,
            event: day.event.clone(),
            weather: day.weather.unwrap_or_default().to_string(),
            mood: day.mood.unwrap_or(DEFAULT_MOOD),
        }
    }
}
impl From<&GuiDayState> for Day {
    fn from(gs: &GuiDayState) -> Self {
        let day = Day {
            date: gs.date,
            event: gs.event.clone(),
            weather: Some(gs.weather.clone()),
            mood: (gs.mood != DEFAULT_MOOD).then(|| gs.mood)
        };
        day
    }
}