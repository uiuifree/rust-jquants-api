use serde::{Deserialize, Serialize};

/// 一括ダウンロード可能なファイルの一覧項目（`/bulk/list`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BulkListItem {
    /// ファイルキー（bulk get で使用）
    #[serde(rename = "Key")]
    pub key: String,
    /// 最終更新日時
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    /// ファイルサイズ（バイト）
    #[serde(rename = "Size")]
    pub size: f64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BulkGetResponse {
    pub url: String,
}
