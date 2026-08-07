//! 各エンドポイントの検索条件。すべて `Default` を実装しているので
//! 必要な条件だけ指定して `..Default::default()` で埋められる。
//! 日付は API の仕様どおり `YYYY-MM-DD` または `YYYYMMDD` 形式の文字列で渡す。

type Params<'a> = Vec<(&'static str, &'a str)>;

fn push<'a>(params: &mut Params<'a>, key: &'static str, value: &'a Option<String>) {
    if let Some(v) = value {
        params.push((key, v.as_str()));
    }
}

/// 銘柄コード・日付・期間の共通条件
/// （日足・分足・売買内訳・信用残・配当・指数日足で使用）
#[derive(Debug, Clone, Default)]
pub struct CodeDateQuery {
    /// 銘柄コード（例: "7203"）
    pub code: Option<String>,
    /// 対象日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
    /// 期間の開始日 (YYYY-MM-DD または YYYYMMDD)
    pub from: Option<String>,
    /// 期間の終了日 (YYYY-MM-DD または YYYYMMDD)
    pub to: Option<String>,
}

impl CodeDateQuery {
    /// 銘柄コードのみ指定するショートカット
    pub fn code(code: impl Into<String>) -> Self {
        Self {
            code: Some(code.into()),
            ..Self::default()
        }
    }

    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "code", &self.code);
        push(&mut p, "date", &self.date);
        push(&mut p, "from", &self.from);
        push(&mut p, "to", &self.to);
        p
    }
}

/// 上場銘柄マスタ（`/equities/master`）の条件
#[derive(Debug, Clone, Default)]
pub struct MasterQuery {
    /// 銘柄コード（例: "7203"）
    pub code: Option<String>,
    /// 対象日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
}

impl MasterQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "code", &self.code);
        push(&mut p, "date", &self.date);
        p
    }
}

/// 投資部門別売買状況（`/equities/investor-types`）の条件
#[derive(Debug, Clone, Default)]
pub struct InvestorTypesQuery {
    /// 市場区分（例: TSEPrime）
    pub section: Option<String>,
    /// 期間の開始日 (YYYY-MM-DD または YYYYMMDD)
    pub from: Option<String>,
    /// 期間の終了日 (YYYY-MM-DD または YYYYMMDD)
    pub to: Option<String>,
}

impl InvestorTypesQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "section", &self.section);
        push(&mut p, "from", &self.from);
        push(&mut p, "to", &self.to);
        p
    }
}

/// 営業日カレンダー（`/markets/calendar`）の条件
#[derive(Debug, Clone, Default)]
pub struct CalendarQuery {
    /// 休日区分（API パラメータ `holidayDivision`）
    pub holiday_division: Option<String>,
    /// 期間の開始日 (YYYY-MM-DD または YYYYMMDD)
    pub from: Option<String>,
    /// 期間の終了日 (YYYY-MM-DD または YYYYMMDD)
    pub to: Option<String>,
}

impl CalendarQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "holidayDivision", &self.holiday_division);
        push(&mut p, "from", &self.from);
        push(&mut p, "to", &self.to);
        p
    }
}

/// 業種別空売り比率（`/markets/short-ratio`）の条件
#[derive(Debug, Clone, Default)]
pub struct ShortRatioQuery {
    /// 33 業種コード
    pub s33: Option<String>,
    /// 対象日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
    /// 期間の開始日 (YYYY-MM-DD または YYYYMMDD)
    pub from: Option<String>,
    /// 期間の終了日 (YYYY-MM-DD または YYYYMMDD)
    pub to: Option<String>,
}

impl ShortRatioQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "s33", &self.s33);
        push(&mut p, "date", &self.date);
        push(&mut p, "from", &self.from);
        push(&mut p, "to", &self.to);
        p
    }
}

/// 空売り残高報告（`/markets/short-sale-report`）の条件
#[derive(Debug, Clone, Default)]
pub struct ShortSaleReportQuery {
    /// 銘柄コード（例: "7203"）
    pub code: Option<String>,
    /// 公表日 (YYYY-MM-DD または YYYYMMDD)
    pub disc_date: Option<String>,
    /// 公表日の開始日
    pub disc_date_from: Option<String>,
    /// 公表日の終了日
    pub disc_date_to: Option<String>,
    /// 計算日 (YYYY-MM-DD または YYYYMMDD)
    pub calc_date: Option<String>,
}

impl ShortSaleReportQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "code", &self.code);
        push(&mut p, "disc_date", &self.disc_date);
        push(&mut p, "disc_date_from", &self.disc_date_from);
        push(&mut p, "disc_date_to", &self.disc_date_to);
        push(&mut p, "calc_date", &self.calc_date);
        p
    }
}

/// 期間のみの条件（`/indices/bars/daily/topix`）
#[derive(Debug, Clone, Default)]
pub struct RangeQuery {
    /// 期間の開始日 (YYYY-MM-DD または YYYYMMDD)
    pub from: Option<String>,
    /// 期間の終了日 (YYYY-MM-DD または YYYYMMDD)
    pub to: Option<String>,
}

impl RangeQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "from", &self.from);
        push(&mut p, "to", &self.to);
        p
    }
}

/// 先物四本値（`/derivatives/bars/daily/futures`）の条件
#[derive(Debug, Clone, Default)]
pub struct FuturesBarsQuery {
    /// 商品区分（例: TOPIXF/NK225F）
    pub category: Option<String>,
    /// 対象日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
    /// 中心限月フラグ（API パラメータ `contractFlag`）
    pub contract_flag: Option<String>,
}

impl FuturesBarsQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "category", &self.category);
        push(&mut p, "date", &self.date);
        push(&mut p, "contractFlag", &self.contract_flag);
        p
    }
}

/// オプション四本値（`/derivatives/bars/daily/options`）の条件
#[derive(Debug, Clone, Default)]
pub struct OptionsBarsQuery {
    /// 商品区分（例: TOPIXF/NK225F）
    pub category: Option<String>,
    /// 銘柄コード（例: "7203"）
    pub code: Option<String>,
    /// 対象日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
    /// 中心限月フラグ（API パラメータ `contractFlag`）
    pub contract_flag: Option<String>,
}

impl OptionsBarsQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "category", &self.category);
        push(&mut p, "code", &self.code);
        push(&mut p, "date", &self.date);
        push(&mut p, "contractFlag", &self.contract_flag);
        p
    }
}

/// 財務系 cursor エンドポイント（`/fins/summary` `/fins/details`）の条件
#[derive(Debug, Clone, Default)]
pub struct FinsQuery {
    /// 銘柄コード（例: "7203"）
    pub code: Option<String>,
    /// 対象日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
    /// 前回応答の `cursor` を渡すと差分取得になる
    pub cursor: Option<String>,
}

impl FinsQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "code", &self.code);
        push(&mut p, "date", &self.date);
        push(&mut p, "cursor", &self.cursor);
        p
    }
}

/// 決算発表予定日（`/fins/earnings-date`）の条件。
/// API の仕様上 `code` / `date` / `scheduled_date` のいずれか 1 つの指定が必須
#[derive(Debug, Clone, Default)]
pub struct EarningsDateQuery {
    /// 銘柄コード（5桁、例: "86970"）
    pub code: Option<String>,
    /// 公表日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
    /// 決算発表予定日 (YYYY-MM-DD または YYYYMMDD)
    pub scheduled_date: Option<String>,
}

impl EarningsDateQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "code", &self.code);
        push(&mut p, "date", &self.date);
        push(&mut p, "scheduled_date", &self.scheduled_date);
        p
    }
}

/// EDINET 系エンドポイント（`/edinet/*`）共通の条件。
/// `edinet_code` と `code` は同時指定できない（API 側でエラーになる）
#[derive(Debug, Clone, Default)]
pub struct EdinetQuery {
    /// 提出者・発行者の EDINET コード（例: "E03814"）
    pub edinet_code: Option<String>,
    /// 銘柄コード（例: "86970"）
    pub code: Option<String>,
    /// 提出日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
}

impl EdinetQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "edinet_code", &self.edinet_code);
        push(&mut p, "code", &self.code);
        push(&mut p, "date", &self.date);
        p
    }
}

/// 適時開示一覧（`/td/list`）の条件
#[derive(Debug, Clone, Default)]
pub struct TdListQuery {
    /// 対象日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
    /// 銘柄コード（例: "7203"）
    pub code: Option<String>,
    /// 期間の開始日 (YYYY-MM-DD または YYYYMMDD)
    pub from: Option<String>,
    /// 期間の終了日 (YYYY-MM-DD または YYYYMMDD)
    pub to: Option<String>,
    /// 開示項目コード（API パラメータ `discItems`）
    pub disc_items: Option<String>,
    /// 前回応答の `cursor` を渡すと差分取得になる
    pub cursor: Option<String>,
}

impl TdListQuery {
    pub(crate) fn params(&self) -> Params<'_> {
        let mut p = Vec::new();
        push(&mut p, "date", &self.date);
        push(&mut p, "code", &self.code);
        push(&mut p, "from", &self.from);
        push(&mut p, "to", &self.to);
        push(&mut p, "discItems", &self.disc_items);
        push(&mut p, "cursor", &self.cursor);
        p
    }
}

/// 一括ダウンロード一覧（`/bulk/list`）の条件
#[derive(Debug, Clone, Default)]
pub struct BulkListQuery {
    /// 対象エンドポイントのパス（例: `/equities/bars/daily`。先頭の `/` は省略可）
    pub endpoint: Option<String>,
    /// 対象日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
    /// 期間の開始日 (YYYY-MM-DD または YYYYMMDD)
    pub from: Option<String>,
    /// 期間の終了日 (YYYY-MM-DD または YYYYMMDD)
    pub to: Option<String>,
}

impl BulkListQuery {
    pub(crate) fn params(&self) -> (Option<String>, Params<'_>) {
        let endpoint = normalize_endpoint(&self.endpoint);
        let mut p = Vec::new();
        push(&mut p, "date", &self.date);
        push(&mut p, "from", &self.from);
        push(&mut p, "to", &self.to);
        (endpoint, p)
    }
}

/// 一括ダウンロード URL 取得（`/bulk/get`）の条件
#[derive(Debug, Clone, Default)]
pub struct BulkGetQuery {
    /// `/bulk/list` が返した `key`
    pub key: Option<String>,
    /// 対象エンドポイントのパス（先頭の `/` は省略可）
    pub endpoint: Option<String>,
    /// 対象日 (YYYY-MM-DD または YYYYMMDD)
    pub date: Option<String>,
}

impl BulkGetQuery {
    pub(crate) fn params(&self) -> (Option<String>, Params<'_>) {
        let endpoint = normalize_endpoint(&self.endpoint);
        let mut p = Vec::new();
        push(&mut p, "key", &self.key);
        push(&mut p, "date", &self.date);
        (endpoint, p)
    }
}

/// endpoint パラメータの先頭に `/` を補完する
fn normalize_endpoint(endpoint: &Option<String>) -> Option<String> {
    endpoint.as_ref().map(|e| {
        if e.starts_with('/') {
            e.clone()
        } else {
            format!("/{e}")
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_date_query_skips_none() {
        let q = CodeDateQuery {
            code: Some("86970".into()),
            from: Some("2026-01-01".into()),
            ..Default::default()
        };
        assert_eq!(q.params(), vec![("code", "86970"), ("from", "2026-01-01")]);
    }

    #[test]
    fn code_date_query_empty() {
        assert!(CodeDateQuery::default().params().is_empty());
    }

    #[test]
    fn code_shortcut() {
        let q = CodeDateQuery::code("7203");
        assert_eq!(q.params(), vec![("code", "7203")]);
    }

    #[test]
    fn calendar_query_uses_camel_case_param() {
        let q = CalendarQuery {
            holiday_division: Some("1".into()),
            ..Default::default()
        };
        assert_eq!(q.params(), vec![("holidayDivision", "1")]);
    }

    #[test]
    fn bulk_query_normalizes_endpoint() {
        let (endpoint, _) = BulkListQuery {
            endpoint: Some("equities/bars/daily".into()),
            ..Default::default()
        }
        .params();
        assert_eq!(endpoint.as_deref(), Some("/equities/bars/daily"));

        let (endpoint, _) = BulkGetQuery {
            endpoint: Some("/equities/bars/daily".into()),
            ..Default::default()
        }
        .params();
        assert_eq!(endpoint.as_deref(), Some("/equities/bars/daily"));
    }
}
