use diary_core::base::env::open_with_default_database;
use crate::terminal::CliSession;

mod terminal;
mod executor;
mod error;
mod command;
mod args;

fn main() {
    let db = open_with_default_database()
        .expect("Could not open database");

    let mut cli = CliSession::new(db);
    cli.run();

}
