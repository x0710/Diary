use diary_core::db::command::Command::*;
use diary_core::db::executor::Executor;
use crate::command::{CliCommand, ParseStr};
use crate::command::CliCommand::*;
use crate::error::CliError;
use crate::terminal::edit_with_editor;

pub trait CliExecutor {
    async fn exec_command(&mut self, comm: &str) -> Result<(), CliError>;
}
impl CliExecutor for Executor {
    async fn exec_command(&mut self, comm: &str) -> Result<(), CliError> {
        let mut command = CliCommand::convert(comm)?;

        if let CoreCommand(Add(date, ctx)) = &mut command {
            // 使用add命令时，查询当天已经写过的数据
            let mut day = self.conn_mut().read_day(*date).await?
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
