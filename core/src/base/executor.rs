use crate::base::command::Command;
use crate::base::date::Date;
use crate::model::day::Day;
use crate::model::event::Event;
use crate::storage::db_mgr::DatabaseManager;

pub struct Executor {
    conn: DatabaseManager,
}
impl Executor {
    pub fn exec(&self, command: &Command) -> Result<Vec<Day>, rusqlite::Error> {
        let mut res = vec![];
        match command {
            Command::Add(date, ctx) => _ = {
                let d = Day::default()
                    .with_date(*date)
                    .with_event(Event::new(ctx.as_deref().unwrap_or_default()));
                self.handle_add(&d)?;
            },
            Command::Remove(date) => _ = self.handle_del(*date),
            Command::Check(date) => {
                if let Some(v) = self.handle_check(*date)? {
                    res.push(v);
                }
            }
            Command::ListAll => res.extend(self.handle_list_all()?)
        }
        Ok(res)
    }
    pub fn handle_check(&self, date: Date) -> Result<Option<Day>, rusqlite::Error> {
        self.conn.read_day(date)
    }
    pub fn handle_list_all(&self) -> Result<Vec<Day>, rusqlite::Error> {
        self.conn.read_all()
    }
    pub fn handle_del(&self, date: Date) -> Result<usize, rusqlite::Error> {
        self.conn.remove_day(date)
    }
    pub fn handle_add(&self, day: &Day) -> Result<usize, rusqlite::Error> {
        self.conn.add_day(day)
    }
    pub fn conn(&self) -> &DatabaseManager {
        &self.conn
    }
    pub fn conn_mut(&mut self) -> &mut DatabaseManager {
        &mut self.conn
    }
}
impl From<DatabaseManager> for Executor {
    fn from(conn: DatabaseManager) -> Self {
        Executor { conn }
    }
}