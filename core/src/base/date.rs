//! 项目封装的日期类

use std::ops::Deref;
use std::str::FromStr;
use time::format_description::BorrowedFormatItem;
use time::{Duration, Month};
use crate::base::error::Error;

/// 首选日期数据交换格式
pub const DATE_FORMAT1: &[BorrowedFormatItem<'static>] = time::macros::format_description!("[year][month][day]");
/// 备选日期数据交换格式
pub const DATE_FORMAT2: &[BorrowedFormatItem<'static>] = time::macros::format_description!("[year]-[month]-[day]");

/// 项目中与时间相关的操作均用此结构体表示
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Date {
    date: time::Date,
}
impl Date {
    /// 通过年月日手动创建`Date`
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, time::error::Error> {
        let d = time::Date::from_calendar_date(year, Month::try_from(month)?, day)?;
        Ok(Date { date: d })
    }
}
impl FromStr for Date {
    type Err = Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let source = source.trim();
        let today = time::OffsetDateTime::now_local()
            .unwrap_or(time::OffsetDateTime::now_utc()).date();
        if source.is_empty() || !source.is_ascii() {
            return Ok(today.into());
        }

        let d = match source {
            "yesterday" | "y" | "yes" => today.previous_day()
                .ok_or_else(|| Error::InvalidDate("It's too small".to_string())),
            "tomorrow" | "tom" => today.next_day()
                .ok_or_else(|| Error::InvalidDate("It's too large".to_string())),
            "today" | "t" => Ok(today),
            _ => {
                if let Ok(dx) = source.parse() {
                    return Ok(today.saturating_add(Duration::days(dx)).into())
                }
                // Regard value input as day when prefix is "m"
                if let Some(dxs) = source.strip_prefix("m") {
                    if let Ok(dx) = dxs.parse() {
                        if let Ok(d) = today.replace_day(dx) {
                            return Ok(Self { date: d });
                        }
                    }
                }
                // Regard value input as day when prefix is "ye"
                if let Some(dxs) = source.strip_prefix("ye") {
                    if dxs.len() == 4 {
                        let month = Month::try_from((dxs.as_bytes()[0]-b'0')*10+dxs.as_bytes()[1]-b'0');
                        let day = (dxs.as_bytes()[2]-b'0')*10+dxs.as_bytes()[3]-b'0';

                        if let Ok(m) = month && let Ok(d) = time::Date::from_calendar_date(today.year(), m, day) {
                            return Ok(Self { date: d });
                        }
                    }
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
    /// 通过系统日期实例化日期
    fn default() -> Self {
        let cur = time::OffsetDateTime::now_local()
            .unwrap_or(time::OffsetDateTime::now_utc());
        Self {
            date: cur.date(),
        }
    }
}
impl Deref for Date {
    type Target = time::Date;

    fn deref(&self) -> &Self::Target {
        &self.date
    }
}