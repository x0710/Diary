use crate::base::command::Command;
use crate::model::day::Day;
use crate::model::event::Event;
use crate::storage::db_mgr::DatabaseManager;

pub struct Executor {
    conn: DatabaseManager,
}
impl Executor {
    pub fn exec(&self, command: &Command) -> Result<Vec<Day>, rusqlite::Error> {
        match command {
            Command::Add(date, ctx) => {
                let d = Day {
                    date: *date,
                    event: Event::new(ctx.as_deref().unwrap_or_default()),
                    ..Default::default()
                };
                self.conn.add_day(&d)?;
                Ok(vec![d])
            },
            Command::Remove(date) => {
                self.conn.remove_day(*date)?;
                Ok(vec![])
            },
            Command::Check(date) => {
                Ok(self.conn.read_day(*date)?.map(|t| vec![t]).unwrap_or_default())
            }
            Command::ListAll => Ok(self.conn().read_all()?)
        }
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