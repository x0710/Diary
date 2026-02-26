//! 提供数据导入与导出相关功能。
//!
//! 本模块负责：
//! - 将外部数据格式（JSON / CSV 等）解析为内部领域模型
//! - 将内部领域模型转换为可导出的结构
//! - 统一管理导入导出所使用的数据结构与格式枚举
//!
//! 设计原则：
//! - 与核心领域模型（如 `Day`）解耦
//! - 导入导出使用独立的数据结构（Record）
//! - 通过 `From` / `TryFrom` 实现与领域模型之间的转换

/// 导入相关实现。
///
/// 负责：
/// - 从文件或字符串读取数据
/// - 根据指定格式解析数据
/// - 转换为内部领域对象
pub mod import;

/// 导出相关实现。
///
/// 负责：
/// - 将领域对象转换为导出结构
/// - 序列化为指定格式
/// - 写入文件或输出流
pub mod export;

/// 导入导出所使用的数据模型。
///
/// 该模块定义专用于数据交换的结构体，
/// 不直接等同于核心领域模型。
///
/// 通过 `From` / `TryFrom` 实现与领域模型的双向转换。
pub mod model {
    //! 导入导出数据实体结构体。
    //!
    //! `Record` 表示对外交换的数据格式，
    //! 字段以可序列化形式存在（如日期为字符串）。
    //!
    //! 与内部 `Day` 结构体之间通过：
    //! - `TryFrom<Record>`
    /// - `From<Day>`
    ///
    /// 实现安全转换。

    use serde::{Deserialize, Serialize};
    use crate::base::date::DATE_FORMAT1;
    use crate::base::error::Error;
    use crate::model::Day;

    /// 导入导出使用的数据记录结构。
    ///
    /// 说明：
    /// - `date` 采用字符串形式存储，便于序列化
    /// - `weather` 与 `mood` 为可选字段
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Record {
        pub date: String,
        pub event: String,
        pub weather: Option<String>,
        pub mood: Option<String>,
    }

    impl TryFrom<Record> for Day {
        type Error = Error;

        /// 将导入结构转换为领域模型。
        ///
        /// 可能失败的情况：
        /// - 日期格式解析失败
        fn try_from(record: Record) -> Result<Self, Self::Error> {
            let date = time::Date::parse(record.date.as_str(), DATE_FORMAT1)?;
            let event = record.event;

            Ok(Self {
                date: date.into(),
                event: event.into(),
                weather: record.weather,
                mood: record.mood,
            })
        }
    }

    impl From<Day> for Record {
        /// 将领域模型转换为可导出结构。
        fn from(value: Day) -> Self {
            Self {
                date: value.date.format(DATE_FORMAT1).unwrap().to_string(),
                event: value.event.instruct,
                weather: value.weather,
                mood: value.mood,
            }
        }
    }
}

/// 支持的导入导出格式。
///
/// 用于统一管理可解析或可导出的数据类型。
pub mod format {
    //! 导入导出格式枚举及解析实现。

    use std::str::FromStr;
    use crate::base::error::Error;

    /// 支持的数据格式。
    pub enum Format {
        Json,
        Csv,
    }

    impl FromStr for Format {
        type Err = Error;

        /// 从字符串解析格式类型（大小写不敏感）。
        ///
        /// 支持：
        /// - "json"
        /// - "csv"
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_ascii_lowercase().as_str() {
                "json" => Ok(Format::Json),
                "csv" => Ok(Format::Csv),
                _ => Err(Error::InvalidData(format!("Unsupported format: '{}'! ", s))),
            }
        }
    }
}