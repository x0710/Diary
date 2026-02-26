use std::ffi::OsStr;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::ExitStatus;
use clap::Parser;
use rustyline::{Config, DefaultEditor};
use rustyline::error::ReadlineError;
use diary_core::base::executor::Executor;
use diary_core::storage::DatabaseManager;
use diary_core::utils::io::export::Exporter;
use diary_core::utils::io::import::DuplicateStrategy::Replace;
use diary_core::utils::io::import::Importer;
use diary_core::utils::io::format::Format::Json;
use crate::args;
use crate::args::{Args, Commands};
use crate::error::CliError;
use crate::executor::CliExecutor;

/// Cli实体表示
pub struct CliSession {
    /// 用户启动程序时所采用的参数
    pub args: Args,
    pub(crate) executor: CliExecutor,
}
impl CliSession {
    pub fn new(db_mgr: DatabaseManager) -> Self {
        let args = Args::parse();
        let exec = Executor::from(db_mgr);
        let exec = CliExecutor::from(exec);
        Self {
            args,
            executor: exec,
        }
    }
    pub fn run(&mut self) {
        if self.args.command.is_some() {
            self.once();
        }else {
            self.interactive();
        }
    }
    /// 如果用户通过命令行解析
    fn once(&mut self) {
        match self.args.command.as_ref().unwrap() {
            Commands::Interactive => self.interactive(),
            Commands::Import {path} => {
                let mut imp = Importer::new(self.executor.exec.conn_mut());
                let data = Importer::read_from_file(path, Json)
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
                                            Json);
                exp.all_export()
                    .expect("Error when export all data");

            }
        }
    }
    /// 如果用户通过交互式运行程序
    fn interactive(&self) {
        // History Enable
        let s = Config::builder().auto_add_history(true).build();
        let mut rl = DefaultEditor::with_config(s).unwrap();
        loop {
            match rl.readline(">: ") {
                Ok(line) => {
                    if line.is_empty() { continue }
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
/// 通过调用外部编辑器编辑文本
/// *s* 预设文本
/// *date* 预设日期（将在临时文件名中出现）
pub fn edit_with_editor(s: &str, date: impl AsRef<OsStr>) -> Result<String, CliError> {
    let mut suffix = date.as_ref().to_os_string();
    // 设置临时文件为markdown格式
    suffix.push(".md");

    let mut editor = tempfile::Builder::default()
        .suffix(&suffix)
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
/// 通过默认编辑器打开文件
pub fn edit_file(file: impl AsRef<Path>) -> std::io::Result<ExitStatus> {
    std::process::Command::new(args::editor())
        .arg(file.as_ref())
        .status()
}
