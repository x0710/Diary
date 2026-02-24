use diary_core::base::command::Command;
use diary_core::base::executor::Executor;
use crate::command::CliCommand;
use crate::error::CliError;
use crate::terminal::edit_with_editor;

pub struct CliExecutor {
    pub(crate) exec: Executor,
}
impl CliExecutor {
    pub fn exec_command(&self, comm: &str) -> Result<(), CliError> {
        let mut command = comm.parse::<CliCommand>()?;
        if let CliCommand::Command(Command::Add(date, ctx)) = &mut command {
            // 使用add命令时，查询当天已经写过的数据
            let the_day = self.exec.conn().read_day(*date)?;

            let mut day_ins = the_day.map(|t| t.event.instruct).unwrap_or_default();
            // 如果在命令行中写了其它内容，追加到之前日记的后面
            if let Some(ctx) = ctx.as_deref() && !ctx.is_empty() {
                day_ins.push('\n');
                day_ins.push_str(ctx);
            }
            let subfix = date.to_string();
            let s = edit_with_editor(&day_ins, subfix);
            *ctx = Some(s?);
        }
        let res = command.exec(&self.exec)?;
        // 
        match command {
            CliCommand::Command(Command::Check(_)) => res.iter().for_each(|v| println!("{}", v.event)),
            CliCommand::Command(Command::ListAll) => res.iter().for_each(|x| println!("{}", x)),
            _ => (),
        }
        Ok(())
    }

}

impl From<Executor> for CliExecutor {
    fn from(exec: Executor) -> Self {
        Self { exec }
    }
}