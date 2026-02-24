//! Gui 当天实体模型
use diary_core::model::day::Day;
use diary_core::model::event::Event;
use diary_core::base::date::Date;
pub const DEFAULT_MOOD: f32 = 0.0;

pub struct GuiDayState {
    pub(crate) date: Date,
    pub(crate) event: Event,
    pub(crate) weather: String,
    pub(crate) mood: f32,
}
impl From<Day> for GuiDayState {
    fn from(day: Day) -> Self {
        Self {
            date: day.date,
            event: day.event.clone(),
            weather: day.weather.unwrap_or_default().to_string(),
            mood: day.mood.map(|t| t.parse().unwrap_or_default()).unwrap_or_default(),
        }
    }
}
impl From<&GuiDayState> for Day {
    fn from(gs: &GuiDayState) -> Self {
        let day = Day {
            date: gs.date,
            event: gs.event.clone(),
            weather: Some(gs.weather.clone()),
            mood: (gs.mood != DEFAULT_MOOD).then( | | gs.mood.to_string())
        };
        day
    }
}