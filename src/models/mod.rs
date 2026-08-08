mod bulk;
mod derivatives;
mod edinet;
mod equities;
mod fins;
pub mod flex;
mod indices;
mod markets;
mod td;

pub use bulk::BulkListItem;
pub use derivatives::{FuturesBar, Options225Bar, OptionsBar};
pub use edinet::{
    CrossShareholdingHolder, CrossShareholdingIssue, CrossShareholdingsDoc, LargeVolumeHolder,
    LargeVolumeShareholdersDoc, MajorShareholder, MajorShareholdersDoc,
};
pub use equities::{AmBar, DailyBar, EarningsCalendar, InvestorType, MinuteBar, StockMaster};
pub use fins::{EarningsDate, FinsDetails, FinsDividend, FinsSummary};
pub use flex::FlexString;
pub use indices::{IndexDailyBar, TopixDailyBar};
pub use markets::{
    Breakdown, Calendar, MarginAlert, MarginInterest, PubReason, ShortRatio, ShortSaleReport,
};
pub use td::{TdBulk, TdFiles, TdFilesInner, TdList};

pub(crate) use bulk::BulkGetResponse;

use serde::Deserialize;

/// 全エンドポイント共通の応答エンベロープ（クレート内部用）
#[derive(Debug, Deserialize)]
pub(crate) struct ApiResponse<T> {
    pub data: Vec<T>,
    pub pagination_key: Option<String>,
    #[serde(default)]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorResponse {
    pub message: String,
}

/// cursor 型エンドポイント（`/fins/summary` `/fins/details` `/td/list`）の結果。
/// `cursor` を次回リクエストに渡すと前回以降の差分を取得できる
#[derive(Debug)]
pub struct CursorPage<T> {
    /// 取得したデータ（ページ分割は追従済み・全件結合）
    pub items: Vec<T>,
    /// 差分取得用トークン。保存して次回クエリの `cursor` に渡す
    pub cursor: Option<String>,
}
