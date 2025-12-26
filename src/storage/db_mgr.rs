use rusqlite::Connection;
use rusqlite::fallible_iterator::FallibleIterator;
use crate::model::day::Day;
use crate::model::event::Event;

pub struct DatabaseManager {
    conn: rusqlite::Connection,
}
impl DatabaseManager {
    pub fn read_all(&self) -> Option<Vec<Day>> {
        unimplemented!()
    }
    pub fn read_day(&self, date: time::Date) -> Option<Day> {
        let res = self.conn.query_row("SELECT date,event,weather,mood FROM day WHERE date=?",
        [date.to_string()], |row| {
            // Formatter
            let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
            // raw_datum
            let date_raw: String = row.get(0)?;
            let date = time::Date::parse(&date_raw, &format).unwrap();
            let event = Event::new(row.get(1)?);
            let weather = row.get(2)?;
            let mood = row.get(3)?;
            // Obj
            Ok(Day::new(date, event, weather, mood))
        });
        res.ok()
    }
    pub fn add_day(&self, day: Day) -> Result<usize, rusqlite::Error> {
        if let Ok(res) = self.conn.execute(
            "INSERT INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)",
            (day.date().to_string(),
                 day.event().instruct.to_string(),
                 day.weather(),
                 day.mood())) {
            println!("New Day: {}", day.date());
            Ok(res)
        }else {
            if let Ok(res) = self.conn.execute(
                "UPDATE day SET event=?1, weather=?2, mood=?3 WHERE date=?4",
                (day.event().instruct.to_string(),
                day.weather(),
                day.mood(),
                day.date().to_string(),)
            ) {
                println!("Modify Day: {}", day.date());
                Ok(res)
            }else {
                eprintln!("Failed to operate in new Day");
                Err(rusqlite::Error::QueryReturnedNoRows)
            }
        }
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