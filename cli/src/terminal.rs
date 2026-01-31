use std::ffi::OsStr;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::ExitStatus;
use clap::Parser;
use rustyline::{Config, DefaultEditor};
use rustyline::error::ReadlineError;
use diary_core::base::executor::Executor;
use diary_core::storage::db_mgr::DatabaseManager;
use diary_core::storage::io::export::Exporter;
use diary_core::storage::io::import::DuplicateStrategy::Replace;
use diary_core::storage::io::import::Importer;
use diary_core::storage::io::mode::Format::JSON;
use crate::args;
use crate::args::{Args, Commands};
use crate::error::CliError;
use crate::executor::CliExecutor;

pub struct CliSession {
    pub args: Args,
    pub(crate) executor: CliExecutor,
}
impl CliSession {
    pub fn new(conn: rusqlite::Connection, ) -> Self {
        let args = Args::parse();
        let exec = DatabaseManager::try_from(conn)
            .expect("Error when open database");
        let exec = Executor::from(exec);
        let exec = CliExecutor::from(exec);
        Self {
            args,
            executor: exec,
        }
    }
    pub fn run(&mut self) {
        if let Some(c) = self.args.command.clone() {
            self.once(c);
        }else {
            self.interactive();
        }
        
    }
    fn once(&mut self, command: Commands) {
        match command {
            Commands::Interactive => self.interactive(),
            Commands::Import {path} => {
                let mut imp = Importer::new(self.executor.exec.conn_mut());
                let data = Importer::read_from_file(path, JSON)
                    .expect("Error when read file");
                if !data.1.is_empty() {
                    for i in data.1 {
                        eprintln!("Import Fail at {}", i);
                    }
                }
                imp.import_to_db(data.0, Replace)
                    .expect("Error when import to database");
            }
            Commands::Export {path} => {
                let mut exp = Exporter::new(self.executor.exec.conn_mut(),
                                        path,
                                        JSON);
                exp.all_export()
                    .expect("Error when export all data");

            }
        }
    }
    fn interactive(&self) {
        // History Enable
        let s = Config::builder().auto_add_history(true).build();
        let mut rl = DefaultEditor::with_config(s).unwrap();
        loop {
            match rl.readline(">: ") {
                Ok(line) => {
                    match self.executor.exec_command(&line) {
                        Ok(_) => (),
                        Err(CliError::Quit) => break,
                        Err(CliError::InvalidArgs(s)) => println!("Invalid args: {}", s),
                        Err(CliError::Io(s)) => println!("IO error: {}", s),
                        Err(CliError::UnknownCommand(s)) => println!("Unknown command: {}", s),

                        _ => {println!("?I don't know what happened.")}
                    }
                },
                Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
                    println!("Have a nice day!");
                    break;
                },
                Err(err) => panic!("{}", err),
            }
        }
    }
}
pub fn edit_with_editor(s: &str, date: impl AsRef<OsStr>) -> Result<String, CliError> {
    let mut editor = tempfile::Builder::default()
        .suffix(date.as_ref())
        .prefix("Luck-for-you:>")
        .tempfile()?;
    editor.write_all(s.as_bytes())?;
    editor.flush()?;
    editor.seek(SeekFrom::Start(0))?;

    edit_file(editor.path())?;


    let mut res = String::new();
    editor.read_to_string(&mut res)?;
    Ok(res)
}
pub fn edit_file(file: impl AsRef<Path>) -> std::io::Result<ExitStatus> {
    std::process::Command::new(args::editor())
        .arg(file.as_ref())
        .status()
}
pub fn cat_file(file: impl AsRef<Path>) -> std::io::Result<ExitStatus> {
    std::process::Command::new("cat")
        .arg(file.as_ref())
        .status()
}
