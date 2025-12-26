#[derive(clap::Parser, Debug, Clone)]
pub struct Args {
    /// Diary for special date
    #[clap(short, long)]
    time: String,
    /// What happened on that day
    #[clap(short, long)]
    event: String,
    /// Delete the day had logged
    #[clap(short, long)]
    remove: Option<bool>,
    /// Delete All the Diaries
    #[clap(long)]
    clear: Option<bool>,
}