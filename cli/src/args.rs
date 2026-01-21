use clap::Subcommand;

const DEFAULT_EDITOR: &str = "vi";
#[derive(clap::Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Interactive,
    Import {
        path: String,
    },
    Export {
        path: String,
    },
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
