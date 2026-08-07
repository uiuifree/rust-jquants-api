/// jquants-api の全 API が返すエラー型。
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// 通信レベルの失敗（接続・タイムアウト等）
    #[error("http error: {0}")]
    Http(
        #[doc = "元となった reqwest のエラー"]
        #[from]
        reqwest::Error,
    ),

    /// J-Quants API がエラー応答を返した（認証エラー・パラメータ不正・レート制限等）
    #[error("api error (status {status}): {message}")]
    Api {
        /// HTTP ステータスコード（401=認証エラー、429=レート制限 など）
        status: u16,
        /// API が返したエラーメッセージ
        message: String,
    },

    /// 応答 JSON を期待した型にデコードできなかった
    #[error("decode error: {source} (body: {body})")]
    Decode {
        /// デコード失敗の原因
        source: serde_json::Error,
        /// 応答本文の先頭 500 文字（API 仕様変更の調査用）
        body: String,
    },

    /// `from_env` で環境変数 `JQUANTS_API_KEY` が見つからなかった
    #[error("JQUANTS_API_KEY environment variable is not set")]
    MissingApiKey,
}
