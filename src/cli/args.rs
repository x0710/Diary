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
pub fn editor() -> String {
    std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR")).unwrap_or("vi".to_string())
}