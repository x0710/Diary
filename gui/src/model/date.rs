use chrono::{Datelike, NaiveDate};

pub struct Date {
    date: diary_core::base::date::Date,
}
impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        let d = diary_core::base::date::Date::new(date.year(), date.month() as u8, date.day() as u8).unwrap();
        Date { date: d }
    }
}
impl From<Date> for NaiveDate {
    fn from(date: Date) -> Self {
        NaiveDate::from_ymd(
            date.date.date().year(),
            date.date.date().month() as u32,
            date.date.date().day() as u32,
        )
    }
}
impl From<Date> for diary_core::base::date::Date {
    fn from(date: Date) -> Self {
        date.date
    }
}
impl Default for Date {
    fn default() -> Self {
        diary_core::base::date::Date::default().into()
    }
}
impl From<diary_core::base::date::Date> for Date {
    fn from(date: diary_core::base::date::Date) -> Self {
        Date { date }
    }
}