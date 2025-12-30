use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::ExitStatus;
use clap::Parser;
use rustyline::error::ReadlineError;
use diary_core::base::executor::Executor;
use diary_core::storage::db_mgr::DatabaseManager;
use crate::args;
use crate::args::Args;
use crate::error::CliError;
use crate::executor::CliExecutor;

pub struct CliSession {
    pub args: Args,
    executor: CliExecutor,
}
impl CliSession {
    pub fn new(conn: rusqlite::Connection) -> Self {
        let args = Args::parse();
        let exec = DatabaseManager::try_from(conn)
            .unwrap();
        let exec = Executor::from(exec);
        let exec = CliExecutor::from(exec);
        Self {
            args,
            executor: exec,
        }
    }
    pub fn run(&self) {
        self.interactive()
    }
    fn once(&self) {
        unimplemented!()
    }
    fn interactive(&self) {
        use rustyline::DefaultEditor;
        let mut rl = DefaultEditor::new().unwrap();
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
pub fn edit_with_editor(s: &str) -> Result<String, CliError> {
    let mut editor = tempfile::NamedTempFile::new()?;
    editor.write_all(s.as_bytes())?;
    editor.flush()?;
    editor.seek(SeekFrom::Start(0))?;

    edit_file(editor.path())?;


    let mut res = String::new();
    editor.read_to_string(&mut res)?;
    Ok(res)
}
pub fn edit_file(file: &Path) -> std::io::Result<ExitStatus> {
    std::process::Command::new(args::editor())
        .arg(file)
        .status()
}
pub fn cat_file(file: &Path) -> std::io::Result<ExitStatus> {
    std::process::Command::new("cat")
        .arg(file)
        .status()
}
