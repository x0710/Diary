use rusqlite::{params, Connection};
use rusqlite::fallible_iterator::FallibleIterator;
use crate::model::day::Day;
use crate::model::event::Event;

pub struct DatabaseManager {
    conn: rusqlite::Connection,
}
impl DatabaseManager {
    pub fn remove_day(&self, date: &time::Date) -> Result<usize, rusqlite::Error> {
        let res = self.conn.execute("DELETE FROM day WHERE date=?",
        params![date.to_string()])?;

        Ok(res)
    }
    pub fn read_all(&self) -> Option<Vec<Day>> {
        unimplemented!("read all days had been recorded");
    }
    pub fn read_day(&self, date: &time::Date) -> Option<Day> {
        let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
        let res = self.conn.query_row("SELECT date,event,weather,mood FROM day WHERE date=?",
        [date.to_string()], |row| {
            // Formatter
            // raw_datum
            let date_raw: String = row.get(0)?;
            let date = time::Date::parse(&date_raw, &format).unwrap();
            let event_str: String = row.get(1)?;
            let weather = row.get(2)?;
            let mood = row.get(3)?;
            // Obj
            Ok(Day::new(date, Event::new(&event_str), weather, mood))
        });
        res.ok()
    }
    pub fn add_day(&self, day: &Day) -> Result<usize, rusqlite::Error> {
        let res = self.conn.execute(
            "INSERT OR REPLACE INTO  day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)",
            (day.date().to_string(),
             day.event().instruct.to_string(),
             day.weather(),
             day.mood()),
        )?;
        Ok(res)
    }
    pub fn with_path(db_path: &str) -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open(db_path)?;

        conn.execute(r#"
        CREATE TABLE IF NOT EXISTS day (
            date TEXT NOT NULL PRIMARY KEY,
            event TEXT,
            weather TEXT,
            mood TEXT)
        "#, ())?;
        Ok(DatabaseManager {
            conn,
        })
    }
}