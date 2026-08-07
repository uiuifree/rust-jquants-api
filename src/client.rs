use crate::error::Error;
use crate::limiter::{Plan, RateLimiter};
use crate::models::{
    AmBar, ApiErrorResponse, ApiResponse, Breakdown, BulkGetResponse, BulkListItem, Calendar,
    CrossShareholdingsDoc, CursorPage, DailyBar, EarningsCalendar, EarningsDate, FinsDetails,
    FinsDividend, FinsSummary, FuturesBar, IndexDailyBar, InvestorType, LargeVolumeShareholdersDoc,
    MajorShareholdersDoc, MarginAlert, MarginInterest, MinuteBar, Options225Bar, OptionsBar,
    ShortRatio, ShortSaleReport, StockMaster, TdBulk, TdFiles, TdList, TopixDailyBar,
};
use crate::query::{
    BulkGetQuery, BulkListQuery, CalendarQuery, CodeDateQuery, EarningsDateQuery, EdinetQuery,
    FinsQuery, FuturesBarsQuery, InvestorTypesQuery, MasterQuery, OptionsBarsQuery, RangeQuery,
    ShortRatioQuery, ShortSaleReportQuery, TdListQuery,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;

/// J-Quants API v2 の本番ベース URL
pub const DEFAULT_BASE_URL: &str = "https://api.jquants.com/v2";

const HEADER_API_KEY: &str = "x-api-key";

/// J-Quants API v2 クライアント。
///
/// API キーは J-Quants のユーザーページ、または公式 CLI（`jquants login`）で取得する。
/// ページ分割（`pagination_key`）は各メソッドが内部で追従し、全件を結合して返す。
///
/// [`with_plan`](Self::with_plan) か [`with_rate_limit`](Self::with_rate_limit) を指定すると、
/// 契約プランの API コール制限を超えないようリクエスト間隔を自動調整する（既定では無効）。
/// クライアントを `clone` してもレート制限のカウンタは共有される
#[derive(Debug, Clone)]
pub struct JQuantsClient {
    http: reqwest::Client,
    api_key: String,
    base_url: String,
    limiter: Option<Arc<RateLimiter>>,
}

impl JQuantsClient {
    /// API キーを指定してクライアントを作る。
    /// キーは <https://jpx-jquants.com/dashboard/api-keys> で発行する
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            limiter: None,
        }
    }

    /// 環境変数 `JQUANTS_API_KEY` から API キーを読む
    pub fn from_env() -> Result<Self, Error> {
        let api_key = std::env::var("JQUANTS_API_KEY").map_err(|_| Error::MissingApiKey)?;
        Ok(Self::new(api_key))
    }

    /// 接続先を差し替える（テスト・デモ環境用）
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// タイムアウトやプロキシを設定済みの `reqwest::Client` に差し替える
    pub fn with_http_client(mut self, http: reqwest::Client) -> Self {
        self.http = http;
        self
    }

    /// 契約プランを指定して、そのプランの API コール制限（[`Plan::requests_per_minute`]）に
    /// 合わせたクライアント側レート制限を有効にする。
    ///
    /// ```no_run
    /// use jquants_api::{JQuantsClient, Plan};
    /// let client = JQuantsClient::new("key").with_plan(Plan::Light); // 60 件/分
    /// ```
    pub fn with_plan(self, plan: Plan) -> Self {
        self.with_rate_limit(plan.requests_per_minute())
    }

    /// 1 分あたりの最大リクエスト数を指定してレート制限を有効にする。
    /// `0` を渡すと無効になる（既定も無効）。
    ///
    /// 制限はページ分割の内部リクエストにも 1 件ずつ適用される。
    /// 上限に達している間は送信を待機するだけで、エラーにはならない
    pub fn with_rate_limit(mut self, requests_per_minute: u32) -> Self {
        self.limiter = if requests_per_minute == 0 {
            None
        } else {
            Some(Arc::new(RateLimiter::per_minute(requests_per_minute)))
        };
        self
    }

    // ── 内部共通処理 ─────────────────────────────────────────────

    async fn throttle(&self) {
        if let Some(limiter) = &self.limiter {
            limiter.acquire().await;
        }
    }

    async fn api_error(response: reqwest::Response, status: u16) -> Error {
        let message = match response.json::<ApiErrorResponse>().await {
            Ok(body) => body.message,
            Err(_) => format!("HTTP {status}"),
        };
        Error::Api { status, message }
    }

    fn decode<T: DeserializeOwned>(text: &str) -> Result<T, Error> {
        serde_json::from_str(text).map_err(|e| Error::Decode {
            source: e,
            body: text.chars().take(500).collect(),
        })
    }

    /// pagination_key を追従して全ページを取得する。
    /// ステータス 210 はデータ未提供（準備中など）を表すため、そこで打ち切って蓄積分を返す
    async fn get_paginated<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<CursorPage<T>, Error> {
        let url = format!("{}{}", self.base_url, path);
        let mut items: Vec<T> = Vec::new();
        let mut pagination_key: Option<String> = None;
        let mut cursor: Option<String> = None;

        loop {
            self.throttle().await;
            let mut request = self
                .http
                .get(&url)
                .header(HEADER_API_KEY, &self.api_key)
                .query(params);
            if let Some(ref pk) = pagination_key {
                request = request.query(&[("pagination_key", pk.as_str())]);
            }
            let response = request.send().await?;

            let status = response.status().as_u16();
            match status {
                200 => {
                    let text = response.text().await?;
                    let body: ApiResponse<T> = Self::decode(&text)?;
                    items.extend(body.data);
                    if body.cursor.is_some() {
                        cursor = body.cursor;
                    }
                    match body.pagination_key {
                        Some(key) => pagination_key = Some(key),
                        None => break,
                    }
                }
                210 => break,
                _ => return Err(Self::api_error(response, status).await),
            }
        }

        Ok(CursorPage { items, cursor })
    }

    async fn get_all<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<Vec<T>, Error> {
        Ok(self.get_paginated(path, params).await?.items)
    }

    async fn get_object<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T, Error> {
        let url = format!("{}{}", self.base_url, path);
        self.throttle().await;
        let response = self
            .http
            .get(&url)
            .header(HEADER_API_KEY, &self.api_key)
            .query(params)
            .send()
            .await?;

        let status = response.status().as_u16();
        if status == 200 {
            let text = response.text().await?;
            Self::decode(&text)
        } else {
            Err(Self::api_error(response, status).await)
        }
    }

    // ── 株式（equities） ─────────────────────────────────────────

    /// 上場銘柄マスタ
    pub async fn master(&self, query: &MasterQuery) -> Result<Vec<StockMaster>, Error> {
        self.get_all("/equities/master", &query.params()).await
    }

    /// 株価日足（調整前後・前後場込み）
    pub async fn daily_bars(&self, query: &CodeDateQuery) -> Result<Vec<DailyBar>, Error> {
        self.get_all("/equities/bars/daily", &query.params()).await
    }

    /// 前場終値時点の株価
    pub async fn am_bars(&self, code: Option<&str>) -> Result<Vec<AmBar>, Error> {
        let params: Vec<(&str, &str)> = code.map(|c| ("code", c)).into_iter().collect();
        self.get_all("/equities/bars/daily/am", &params).await
    }

    /// 株価分足
    pub async fn minute_bars(&self, query: &CodeDateQuery) -> Result<Vec<MinuteBar>, Error> {
        self.get_all("/equities/bars/minute", &query.params()).await
    }

    /// 決算発表予定
    pub async fn earnings_calendar(&self) -> Result<Vec<EarningsCalendar>, Error> {
        self.get_all("/equities/earnings-calendar", &[]).await
    }

    /// 投資部門別売買状況
    pub async fn investor_types(
        &self,
        query: &InvestorTypesQuery,
    ) -> Result<Vec<InvestorType>, Error> {
        self.get_all("/equities/investor-types", &query.params())
            .await
    }

    // ── 市場（markets） ──────────────────────────────────────────

    /// 売買内訳
    pub async fn breakdown(&self, query: &CodeDateQuery) -> Result<Vec<Breakdown>, Error> {
        self.get_all("/markets/breakdown", &query.params()).await
    }

    /// 営業日カレンダー
    pub async fn trading_calendar(&self, query: &CalendarQuery) -> Result<Vec<Calendar>, Error> {
        self.get_all("/markets/calendar", &query.params()).await
    }

    /// 日々公表信用取引残高
    pub async fn margin_alert(&self, query: &CodeDateQuery) -> Result<Vec<MarginAlert>, Error> {
        self.get_all("/markets/margin-alert", &query.params()).await
    }

    /// 信用取引週末残高
    pub async fn margin_interest(
        &self,
        query: &CodeDateQuery,
    ) -> Result<Vec<MarginInterest>, Error> {
        self.get_all("/markets/margin-interest", &query.params())
            .await
    }

    /// 業種別空売り比率
    pub async fn short_ratio(&self, query: &ShortRatioQuery) -> Result<Vec<ShortRatio>, Error> {
        self.get_all("/markets/short-ratio", &query.params()).await
    }

    /// 空売り残高報告
    pub async fn short_sale_report(
        &self,
        query: &ShortSaleReportQuery,
    ) -> Result<Vec<ShortSaleReport>, Error> {
        self.get_all("/markets/short-sale-report", &query.params())
            .await
    }

    // ── 指数（indices） ──────────────────────────────────────────

    /// 指数日足
    pub async fn index_daily_bars(
        &self,
        query: &CodeDateQuery,
    ) -> Result<Vec<IndexDailyBar>, Error> {
        self.get_all("/indices/bars/daily", &query.params()).await
    }

    /// TOPIX 日足
    pub async fn topix_daily_bars(&self, query: &RangeQuery) -> Result<Vec<TopixDailyBar>, Error> {
        self.get_all("/indices/bars/daily/topix", &query.params())
            .await
    }

    // ── デリバティブ（derivatives） ──────────────────────────────

    /// 先物四本値
    pub async fn futures_bars(&self, query: &FuturesBarsQuery) -> Result<Vec<FuturesBar>, Error> {
        self.get_all("/derivatives/bars/daily/futures", &query.params())
            .await
    }

    /// オプション四本値
    pub async fn options_bars(&self, query: &OptionsBarsQuery) -> Result<Vec<OptionsBar>, Error> {
        self.get_all("/derivatives/bars/daily/options", &query.params())
            .await
    }

    /// 日経225オプション四本値
    pub async fn options_225_bars(&self, date: Option<&str>) -> Result<Vec<Options225Bar>, Error> {
        let params: Vec<(&str, &str)> = date.map(|d| ("date", d)).into_iter().collect();
        self.get_all("/derivatives/bars/daily/options/225", &params)
            .await
    }

    // ── 財務（fins） ─────────────────────────────────────────────

    /// 財務諸表サマリー。`cursor` を次回の [`FinsQuery::cursor`] に渡すと差分取得できる
    pub async fn fins_summary(&self, query: &FinsQuery) -> Result<CursorPage<FinsSummary>, Error> {
        self.get_paginated("/fins/summary", &query.params()).await
    }

    /// 財務諸表詳細。`cursor` を次回の [`FinsQuery::cursor`] に渡すと差分取得できる
    pub async fn fins_details(&self, query: &FinsQuery) -> Result<CursorPage<FinsDetails>, Error> {
        self.get_paginated("/fins/details", &query.params()).await
    }

    /// 配当金情報
    pub async fn fins_dividend(&self, query: &CodeDateQuery) -> Result<Vec<FinsDividend>, Error> {
        self.get_all("/fins/dividend", &query.params()).await
    }

    /// 決算発表予定日。
    /// API の仕様上 `code` / `date` / `scheduled_date` のいずれか 1 つの指定が必須
    pub async fn fins_earnings_date(
        &self,
        query: &EarningsDateQuery,
    ) -> Result<Vec<EarningsDate>, Error> {
        self.get_all("/fins/earnings-date", &query.params()).await
    }

    // ── EDINET（Standard プラン以上） ───────────────────────────

    /// 大株主の状況（有価証券報告書等より）。Standard プラン以上
    pub async fn edinet_major_shareholders(
        &self,
        query: &EdinetQuery,
    ) -> Result<Vec<MajorShareholdersDoc>, Error> {
        self.get_all("/edinet/major-shareholders", &query.params())
            .await
    }

    /// 政策保有株式。Standard プラン以上
    pub async fn edinet_cross_shareholdings(
        &self,
        query: &EdinetQuery,
    ) -> Result<Vec<CrossShareholdingsDoc>, Error> {
        self.get_all("/edinet/cross-shareholdings", &query.params())
            .await
    }

    /// 大量保有報告書。Standard プラン以上
    pub async fn edinet_large_volume_shareholders(
        &self,
        query: &EdinetQuery,
    ) -> Result<Vec<LargeVolumeShareholdersDoc>, Error> {
        self.get_all("/edinet/large-volume-shareholders", &query.params())
            .await
    }

    // ── 適時開示（td） ───────────────────────────────────────────

    /// 適時開示一覧。`cursor` を次回の [`TdListQuery::cursor`] に渡すと差分取得できる
    pub async fn td_list(&self, query: &TdListQuery) -> Result<CursorPage<TdList>, Error> {
        self.get_paginated("/td/list", &query.params()).await
    }

    /// 適時開示の一括ダウンロード情報
    pub async fn td_bulk(&self) -> Result<TdBulk, Error> {
        self.get_object("/td/bulk", &[]).await
    }

    /// 適時開示ファイル（PDF / XBRL）の取得先 URL
    pub async fn td_files(&self, disc_no: &str, docs: Option<&str>) -> Result<TdFiles, Error> {
        let mut params = vec![("discNo", disc_no)];
        if let Some(d) = docs {
            params.push(("docs", d));
        }
        self.get_object("/td/files", &params).await
    }

    // ── 一括ダウンロード（bulk） ─────────────────────────────────

    /// 一括ダウンロード可能なファイル一覧
    pub async fn bulk_list(&self, query: &BulkListQuery) -> Result<Vec<BulkListItem>, Error> {
        let (endpoint, mut params) = query.params();
        if let Some(ref e) = endpoint {
            params.push(("endpoint", e.as_str()));
        }
        self.get_all("/bulk/list", &params).await
    }

    /// 一括ダウンロードファイルの署名付き URL を取得する
    pub async fn bulk_get(&self, query: &BulkGetQuery) -> Result<String, Error> {
        let (endpoint, mut params) = query.params();
        if let Some(ref e) = endpoint {
            params.push(("endpoint", e.as_str()));
        }
        let body: BulkGetResponse = self.get_object("/bulk/get", &params).await?;
        Ok(body.url)
    }
}
