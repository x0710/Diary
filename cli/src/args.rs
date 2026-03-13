use clap::{Args, Parser, Subcommand};

const DEFAULT_EDITOR: &str = "vi";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Default)]
pub enum Commands {
    #[default]
    Interactive,
    Import(TargetFormat),
    Export(TargetFormat),
}

#[derive(Args, Debug)]
pub struct TargetFormat {
    pub path: String,
    #[command(flatten)]
    pub format: FormatArg,
}

#[derive(Clone, Args, Debug)]
// 注意：如果 group 设置为 required = true 且没有默认值，用户必须输入其中一个
pub struct FormatArg {
    #[arg(long)]
    json: bool,
    #[arg(long)]
    csv: bool,
}
#[cfg(target_os = "linux")]
pub fn editor() -> String {
    std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| {
            eprintln!("Could not find $VISUAL or $EDITOR, using {} editor", DEFAULT_EDITOR);
            String::from(DEFAULT_EDITOR)
        })
}
#[cfg(target_os = "windows")]
pub fn editor() -> String {
    std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR")).unwrap_or("notepad".to_string())
}

impl From<&FormatArg> for diary_core::utils::io::format::Format {
    fn from(value: &FormatArg) -> Self {
        if value.csv {Self::Csv} else {Self::Json}
    }
}
