use diary_core::base::env::open_with_default_database;
use crate::terminal::CliSession;

mod terminal;
mod error;
mod args;
mod command;
mod executor;

fn main() {
    let db = open_with_default_database()
        .expect("Could not open database");

    let cli = CliSession::new(db);
    cli.run()
}