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
            let s = match ctx.as_deref() {
                Some(ctx) => ctx.to_string(),
                None => self.exec.conn().read_day(*date)?
                    .map(|t| t.event().instruct.clone())
                    .unwrap_or_default()
            };
            let subfix = format!("{}-{}-{} {}", date.date().year(), date.date().month(), date.date().day(), date.date().weekday());
            let s = edit_with_editor(&s, subfix);
            *ctx = Some(s?);
        }
        let res = command.exec(&self.exec)?;
        // 
        match command {
            CliCommand::Command(Command::Check(_)) => res.iter().for_each(|v| println!("{}", v.event())),
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