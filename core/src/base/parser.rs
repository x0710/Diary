use crate::base::command::{Command, SubCommand};
use crate::base::date::Date;
use crate::base::error::Error;

pub struct Parser;

impl Parser {
    pub fn parse(&self, input: &str) -> Result<Command, Error> {
        let mut args = input.splitn(3, ' ');
        let sub = args.nth(0).unwrap_or("").parse::<SubCommand>()?;
        let date = args.nth(1).unwrap_or("");
        let ctx = args.nth(2);

        match sub {
            SubCommand::Add => {
                let date = date.parse::<Date>()?;
                Ok(Command::Add(date, ctx.map(str::to_string)))
            }
            SubCommand::Remove => {
                let date = date.parse::<Date>()?;
                Ok(Command::Remove(date))
            }
            SubCommand::Check => {
                let date = date.parse::<Date>()?;
                Ok(Command::Check(date))
            }
            SubCommand::ListAll => Ok(Command::ListAll),
            // SubCommand::Help => Ok(Command::Help),
            // SubCommand::Quit => Ok(Command::Quit),
        }
    }
}
