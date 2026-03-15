use diary_core::base::error::Error;
use diary_core::db::DatabaseManager;
use diary_core::model::Day;
use crate::command::CliCommand::CoreCommand;
use crate::command::{CliCommand, Command};
use crate::command::Command::*;
use crate::error::CliError;
use crate::terminal::edit_with_editor;

pub struct Executor {
    pub(crate) conn: DatabaseManager,
}
impl Executor {
    pub fn new(conn: DatabaseManager) -> Self {
        Self { conn }
    }
    pub async fn exec(&mut self, command: &Command) -> Result<Vec<Day>, Error> {
        match command {
            Add(d) => {
                self.conn.add_day(&d).await?;
                Ok(vec![d.to_owned()])
            },
            Remove(date) => {
                self.conn.remove_day(*date).await?;
                Ok(vec![])
            },
            Check(date) => {
                Ok(self.conn.read_day(*date).await?.map(|t| vec![t]).unwrap_or_default())
            }
            ListAll => Ok(self.conn.read_all().await?)
        }
    }
}

impl Executor {
    pub(crate) async fn exec_command(&mut self, comm: &str) -> Result<(), CliError> {
        let mut command = comm.parse::<CliCommand>()?;

        if let CoreCommand(Add(day_cond)) = &mut command {
            // 使用add命令时，查询当天已经写过的数据
            let mut day = self.conn.read_day(day_cond.date).await?
                .unwrap_or_default();

            edit_with_editor(&mut day)?;
            *day_cond = day;
        }
        let res = command.exec(self).await?;
        //
        match command {
            CoreCommand(Check(_)) => res.iter().for_each(|v| println!("{}", v.event)),
            CoreCommand(ListAll) => res.iter().for_each(|x| println!("{}", x)),
            _ => (),
        }
        Ok(())
    }

}