use rusqlite::{params, Connection, Transaction};
use crate::model::day::Day;
use crate::model::event::Event;
use crate::base::date::Date;
use crate::base::date::DATE_FORMAT1;

pub struct DatabaseManager {
    conn: Connection,
}
impl DatabaseManager {
    pub fn transaction(&mut self) ->  rusqlite::Result<Transaction<'_>> {
        self.conn.transaction()
    }
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
    pub fn from_path(path: &std::path::Path) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;
        Self::try_from(conn)
    }
    pub fn remove_day(&self, date: Date) -> Result<usize, rusqlite::Error> {
        self.conn.execute("DELETE FROM day WHERE date=?",
        [date.format(DATE_FORMAT1).unwrap()])
    }
    pub fn read_all(&self) -> Result<Vec<Day>, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT date, event, weather, mood FROM day ORDER BY date ASC")?;
        let res = stmt
            .query_map((), |row| row_to_day(row))?;
        res.collect()

    }
    pub fn read_from_to(&self, from: Date, to: Date) -> Result<Vec<Day>, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT date,event,weather,mood FROM day WHERE date BETWEEN ?1 AND ?2")?;
        let res = stmt.query_map([from.format(DATE_FORMAT1).unwrap(), to.format(DATE_FORMAT1).unwrap()], |row| {
            row_to_day(row)
        })?;
        res.collect()
    }
    pub fn read_day(&self, date: Date) -> Result<Option<Day>, rusqlite::Error> {
        let r = self.conn.query_row("SELECT date,event,weather,mood FROM day WHERE date=?",
                                    params![date.format(DATE_FORMAT1).unwrap()], |row|
            row_to_day(row)
        );
        match r {
            Ok(day) => Ok(Some(day)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
    pub fn add_day(&self, day: &Day) -> Result<usize, rusqlite::Error> {
        let res = self.conn.execute(
            "INSERT OR REPLACE INTO day (date, event, weather, mood) VALUES (?1, ?2, ?3, ?4)",
            (day.date().format(DATE_FORMAT1).unwrap(),
             day.event().instruct.to_string(),
             day.weather(),
             day.mood()),
        )?;
        Ok(res)
    }
}
impl TryFrom<Connection> for DatabaseManager {
    type Error = rusqlite::Error;
    fn try_from(conn: Connection) -> Result<Self, Self::Error> {
        conn.execute(r#"
        CREATE TABLE IF NOT EXISTS day (
            date TEXT NOT NULL PRIMARY KEY,
            event TEXT,
            weather TEXT,
            mood TEXT)
        "#, ())?;
        Ok(Self {
            conn,
        })
    }
}
fn row_to_day(row: &rusqlite::Row) -> Result<Day, rusqlite::Error> {
    // raw_datum
    let date_raw: String = row.get(0)?;
    let date = time::Date::parse(&date_raw, DATE_FORMAT1).unwrap();
    let event_str: String = row.get(1)?;
    let weather = row.get(2)?;
    let mood = row.get(3)?;
    // Obj
    Ok(Day::new(date.into(), Event::new(&event_str), weather, mood))
}
