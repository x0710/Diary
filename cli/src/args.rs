const DEFAULT_EDITOR: &str = "vi";
#[derive(clap::Parser, Debug, Clone)]
pub struct Args {
    /// Diary for special date
    #[clap(short, long)]
    pub time: Option<String>,

    /// What happened on that day
    #[clap(short, long)]
    pub event: Option<String>,

    /// Delete the day had logged
    #[clap(short, long)]
    pub remove: bool,

    /// Delete All the Diaries
    #[clap(long)]
    pub clear: bool,

    /// Interactive Mode
    #[clap(short, long)]
    pub interactive: bool,
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
