use diary_core::base::error::Error;
use diary_core::model::Day;
use diary_core::model::Event;
use diary_core::db::DatabaseManager;
use crate::command::CliCommand::CoreCommand;
use crate::command::Command;
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
            Add(date, ctx) => {
                let d = Day {
                    date: *date,
                    event: Event::new(ctx.as_deref().unwrap_or_default()),
                    ..Default::default()
                };
                self.conn.add_day(&d).await?;
                Ok(vec![d])
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
        let mut command = comm.parse::<_>()?;

        if let CoreCommand(Add(date, ctx)) = &mut command {
            // 使用add命令时，查询当天已经写过的数据
            let mut day = self.conn.read_day(*date).await?
                .unwrap_or_default();

            // 如果在命令行中写了其它内容，追加到之前日记的后面
            if let Some(append_str) = ctx.as_deref() && !append_str.is_empty() {
                day.event.instruct.push('\n');
                day.event.instruct.push_str(append_str);
            }
            let s = edit_with_editor(&mut day)?;
            *ctx = Some(s);
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
impl From<DatabaseManager> for Executor {
    fn from(conn: DatabaseManager) -> Self {
        Executor { conn }
    }
}