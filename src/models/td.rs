use serde::{Deserialize, Serialize};

/// 適時開示一覧（`/td/list`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TdList {
    /// 開示番号（14桁）
    #[serde(rename = "DiscNo")]
    pub disc_no: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 会社名
    #[serde(rename = "Name")]
    pub name: String,
    /// 開示日 (YYYY-MM-DD)
    #[serde(rename = "DiscDate")]
    pub disc_date: String,
    /// 開示時刻 (HH:MM)
    #[serde(rename = "DiscTime")]
    pub disc_time: String,
    /// 開示タイトル
    #[serde(rename = "Title")]
    pub title: String,
    /// 取扱属性（null=通常/revision=修正/delete=削除）
    #[serde(rename = "DiscStatus")]
    pub disc_status: Option<String>,
    /// 開示履歴番号（1〜99）
    #[serde(rename = "RevNo")]
    pub rev_no: String,
    /// 公開項目コードリスト
    #[serde(rename = "DiscItems")]
    pub disc_items: Vec<String>,
    /// 書類タイプ（g=PDF/s=サマリー/x=XBRL）
    #[serde(rename = "Docs")]
    pub docs: Vec<String>,
}

/// 適時開示一括ダウンロード情報（`/td/bulk`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TdBulk {
    /// CSVファイルの最終更新日時（ISO 8601形式）
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    /// CSVファイルのダウンロードURL
    pub url: String,
}

/// 適時開示ファイルの取得先 URL
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TdFilesInner {
    /// 開示資料PDFのURL（提供がない場合は `None`）
    pub pdf: Option<String>,
    /// サマリーPDFのURL（提供がない場合は `None`）
    #[serde(rename = "summaryPdf")]
    pub summary_pdf: Option<String>,
    /// XBRLファイルのURL（提供がない場合は `None`）
    pub xbrl: Option<String>,
}

/// 適時開示ファイル（`/td/files`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TdFiles {
    /// 開示番号（14桁）
    #[serde(rename = "discNo")]
    pub disc_no: String,
    /// 各書類の取得先URL
    pub files: TdFilesInner,
}
