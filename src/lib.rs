//! J-Quants API v2 の非公式 Rust クライアント。
//!
//! Unofficial async Rust client for the [J-Quants API](https://jpx-jquants.com/) v2 —
//! Japanese stock market data (equities, financials, margin trading, short selling,
//! derivatives, indices, timely disclosure) by JPX Market Innovation & Research.
//!
//! J-Quants API は JPX 総研が提供する個人投資家向けの日本株データ API。
//! 本クレートの利用には J-Quants の契約（API キー、無料プランあり）が必要で、
//! 取得したデータの扱いは J-Quants の利用規約に従うこと。
//!
//! # クイックスタート
//!
//! API キーを <https://jpx-jquants.com/dashboard/api-keys> で発行し、
//! 環境変数 `JQUANTS_API_KEY` に設定する。
//!
//! ```no_run
//! use jquants_api::{CodeDateQuery, JQuantsClient};
//!
//! # async fn run() -> Result<(), jquants_api::Error> {
//! let client = JQuantsClient::from_env()?; // JQUANTS_API_KEY を読む
//!
//! let query = CodeDateQuery {
//!     code: Some("7203".into()),
//!     from: Some("2026-01-01".into()),
//!     ..Default::default()
//! };
//! let bars = client.daily_bars(&query).await?;
//! for bar in &bars {
//!     println!("{} close={:?} (adjusted: {:?})", bar.date, bar.close, bar.adj_close);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # 設計
//!
//! - ページ分割（`pagination_key`）は各メソッドが自動追従し、全件を結合して返す
//! - [`fins_summary`](JQuantsClient::fins_summary) / [`fins_details`](JQuantsClient::fins_details) /
//!   [`td_list`](JQuantsClient::td_list) は [`CursorPage`] を返す。`cursor` を保存して
//!   次回のクエリに渡すと、前回以降の差分だけ取得できる
//! - string / number / null が混在する項目は [`FlexString`] で受け取る（[`FlexString::as_f64`] あり）
//! - [`JQuantsClient::with_plan`] / [`JQuantsClient::with_rate_limit`] で、契約プランの
//!   API コール制限（Free 5 件/分〜Premium 500 件/分）に合わせた自動待機を有効にできる
//!   （既定は無効）。それでも `Error::Api { status: 429, .. }` が返ったら間隔を空けること
//!
//! ```no_run
//! use jquants_api::{FinsQuery, JQuantsClient};
//!
//! # async fn run(client: &JQuantsClient, saved: Option<String>) -> Result<(), jquants_api::Error> {
//! // 差分取得: 前回保存した cursor を渡す
//! let page = client
//!     .fins_summary(&FinsQuery { cursor: saved, ..Default::default() })
//!     .await?;
//! for row in &page.items {
//!     println!("{} {} sales={}", row.disc_date, row.code, row.sales);
//! }
//! let next_cursor = page.cursor; // 保存して次回に渡す
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]

mod client;
mod error;
mod limiter;
mod models;
mod query;

pub use client::{JQuantsClient, DEFAULT_BASE_URL};
pub use error::Error;
pub use limiter::Plan;
pub use models::{
    AmBar, Breakdown, BulkListItem, Calendar, CrossShareholdingHolder, CrossShareholdingIssue,
    CrossShareholdingsDoc, CursorPage, DailyBar, EarningsCalendar, EarningsDate, FinsDetails,
    FinsDividend, FinsSummary, FlexString, FuturesBar, IndexDailyBar, InvestorType,
    LargeVolumeHolder, LargeVolumeShareholdersDoc, MajorShareholder, MajorShareholdersDoc,
    MarginAlert, MarginInterest, MinuteBar, Options225Bar, OptionsBar, PubReason, ShortRatio,
    ShortSaleReport, StockMaster, TdBulk, TdFiles, TdFilesInner, TdList, TopixDailyBar,
};
pub use query::{
    BulkGetQuery, BulkListQuery, CalendarQuery, CodeDateQuery, EarningsDateQuery, EdinetQuery,
    FinsQuery, FuturesBarsQuery, InvestorTypesQuery, MasterQuery, OptionsBarsQuery, RangeQuery,
    ShortRatioQuery, ShortSaleReportQuery, TdListQuery,
};
