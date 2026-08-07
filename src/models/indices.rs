use super::flex::FlexString;
use serde::{Deserialize, Serialize};

/// TOPIX 日足（`/indices/bars/daily/topix`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TopixDailyBar {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 始値
    #[serde(rename = "O")]
    pub open: FlexString,
    /// 高値
    #[serde(rename = "H")]
    pub high: FlexString,
    /// 安値
    #[serde(rename = "L")]
    pub low: FlexString,
    /// 終値
    #[serde(rename = "C")]
    pub close: FlexString,
}

/// 指数日足（`/indices/bars/daily`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IndexDailyBar {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 指数コード（例: 0000=TOPIX, 0028=東証マザーズ指数）
    #[serde(rename = "Code")]
    pub code: String,
    /// 始値
    #[serde(rename = "O")]
    pub open: FlexString,
    /// 高値
    #[serde(rename = "H")]
    pub high: FlexString,
    /// 安値
    #[serde(rename = "L")]
    pub low: FlexString,
    /// 終値
    #[serde(rename = "C")]
    pub close: FlexString,
}
