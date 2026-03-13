//! 可供操作命令
use crate::base::date::Date;

/// 储存用户在做操作时的参数
#[derive(Debug, Clone)]
pub enum Command {
    Add(Date, Option<String>),
    Remove(Date),
    Check(Date),
    ListAll,
    // Quit,
    // Help,
}
