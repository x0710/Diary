use std::str::FromStr;
use time::format_description::BorrowedFormatItem;
use time::{Duration, Month};
use crate::base::error::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Date {
    date: time::Date,
}
impl Date {
    pub fn date(&self) -> time::Date {
        self.date
    }
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, time::error::Error> {
        let d = time::Date::from_calendar_date(year, Month::try_from(month)?, day)?;
        Ok(Date { date: d })
    }
}
const DATE_FORMAT1: &[BorrowedFormatItem<'static>] = time::macros::format_description!("[year]-[month]-[day]");
const DATE_FORMAT2: &[BorrowedFormatItem<'static>] = time::macros::format_description!("[year][month][day]");
impl FromStr for Date {
    type Err = Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let source = source.trim();
        let today = time::OffsetDateTime::now_local()
            .unwrap_or(time::OffsetDateTime::now_utc()).date();

        let d = match source {
            "yesterday" | "y" => today.previous_day()
                .ok_or_else(|| Error::InvalidDate("It's too small".to_string())),
            "tomorrow" | "m" => today.next_day()
                .ok_or_else(|| Error::InvalidDate("It's too large".to_string())),
            "today" | "t" => Ok(today),
            _ => {
                if let Ok(dx) = source.parse() {
                    return Ok((today.saturating_add(Duration::days(dx))).into())
                }
                time::Date::parse(source, &DATE_FORMAT1)
                    .or_else(|_| time::Date::parse(source, &DATE_FORMAT2))
                    .map_err(|_| Error::InvalidDate(source.to_string()))
            }
        }?;
        Ok(d.into())
    }
}
impl From<time::Date> for Date {
    fn from(date: time::Date) -> Self {
        Self { date }
    }
}
impl Default for Date {
    fn default() -> Self {
        let cur = time::OffsetDateTime::now_local()
            .unwrap_or(time::OffsetDateTime::now_utc());
        Self {
            date: cur.date(),
        }
    }
}