use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::ExitStatus;
use clap::Parser;
use rustyline::error::ReadlineError;
use crate::args;
use crate::args::Args;
use crate::executor::{CliErr, Executor};

pub struct CliSession {
    pub args: Args,
    executor: Executor,
}
impl CliSession {
    pub fn new(conn: rusqlite::Connection) -> Self {
        let args = Args::parse();
        let exec = Executor::new(conn.try_into()
            .expect("database connection should be successful"));
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
                    match self.executor.exec_command(line.as_str()) {
                        Err(CliErr::Db(err)) => {
                            eprintln!("Database error: {}", err);
                        },
                        Err(CliErr::Io(err)) => {
                            eprintln!("IO Error: {}", err);
                        },
                        Err(CliErr::InvalidDate(err)) => {
                            eprintln!("Invalid date: {}", err);
                        },
                        Err(CliErr::UnknownCommand(cmd)) => {
                            eprintln!("Unknown command: {}", cmd);
                        },
                        Err(CliErr::Exit) => {
                            println!("Have a nice day!");
                            break;
                        },
                        Ok(_) => (),
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
pub fn edit_with_editor(s: &str) -> Result<String, CliErr> {
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
