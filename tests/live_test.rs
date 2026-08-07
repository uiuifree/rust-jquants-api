//! 実 API に対する疎通テスト。
//! 環境変数 `JQUANTS_API_KEY` が設定されているときだけ実行し、未設定ならスキップする。
//!
//! ```sh
//! export JQUANTS_API_KEY=your_api_key
//! cargo test --test live_test
//! ```

use jquants_api::{CodeDateQuery, JQuantsClient, MasterQuery};

fn live_client() -> Option<JQuantsClient> {
    // .env があれば読み込む（環境変数が既にあればそちらが優先される）
    dotenvy::dotenv().ok();
    match JQuantsClient::from_env() {
        Ok(client) => Some(client),
        Err(_) => {
            eprintln!("skip: JQUANTS_API_KEY が未設定");
            None
        }
    }
}

#[tokio::test]
async fn live_master_returns_rows() {
    let Some(client) = live_client() else { return };

    let rows = client
        .master(&MasterQuery {
            code: Some("86970".into()), // 日本取引所グループ
            ..Default::default()
        })
        .await
        .unwrap();

    assert!(!rows.is_empty());
    assert!(rows.iter().all(|r| r.code.starts_with("8697")));
}

#[tokio::test]
async fn live_daily_bars_returns_rows() {
    let Some(client) = live_client() else { return };

    // Free プランは 12 週遅延のため、確実に提供済みの過去期間を指定する
    let bars = client
        .daily_bars(&CodeDateQuery {
            code: Some("86970".into()),
            from: Some("2026-01-05".into()),
            to: Some("2026-01-30".into()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert!(!bars.is_empty());
    assert!(bars.iter().all(|b| !b.date.is_empty()));
}
