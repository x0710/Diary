use diary_core::base::error::Error;
use diary_core::model::Day;
use diary_core::model::Event;
use diary_core::db::DatabaseManager;
use crate::command::Command;

pub struct Executor {
    pub(crate) conn: DatabaseManager,
}
impl Executor {
    pub async fn exec(&mut self, command: &Command) -> Result<Vec<Day>, Error> {
        match command {
            Command::Add(date, ctx) => {
                let d = Day {
                    date: *date,
                    event: Event::new(ctx.as_deref().unwrap_or_default()),
                    ..Default::default()
                };
                self.conn.add_day(&d).await?;
                Ok(vec![d])
            },
            Command::Remove(date) => {
                self.conn.remove_day(*date).await?;
                Ok(vec![])
            },
            Command::Check(date) => {
                Ok(self.conn.read_day(*date).await?.map(|t| vec![t]).unwrap_or_default())
            }
            Command::ListAll => Ok(self.conn.read_all().await?)
        }
    }
}
impl From<DatabaseManager> for Executor {
    fn from(conn: DatabaseManager) -> Self {
        Executor { conn }
    }
}