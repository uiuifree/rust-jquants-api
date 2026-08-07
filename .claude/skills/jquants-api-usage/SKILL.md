---
name: jquants-api-usage
description: >
  Rust クレート jquants-api で J-Quants API v2 から日本株データを取得するコードを書く。
  株価日足・分足・四本値、上場銘柄マスタ、財務諸表・配当、信用取引残高、空売り比率、
  先物・オプション、TOPIX・指数、適時開示、一括ダウンロードに対応。
  トリガー: jquants, J-Quants, jquants-api, 日本株, 株価, 株価データ, 日足, 四本値, OHLCV,
  JPX, 東証, TOPIX, 財務諸表, 決算, 配当, 信用取引, 空売り, 先物, オプション,
  Japanese stock, stock price, market data, Rust, バックテスト, backtesting
---

# jquants-api 使用スキル

## 前提

- `Cargo.toml` に `jquants-api` と `tokio` を追加する
- API キーは環境変数 `JQUANTS_API_KEY`（または `.env` + dotenvy）。
  取得先: https://jpx-jquants.com/dashboard/api-keys
- 全メソッド async。エラー型は `jquants_api::Error`

## コードを書くときの判断順序

1. **エンドポイントを選ぶ** — 完全な一覧とシグネチャは同リポジトリの `llms.txt` を読む（推測しない）
2. **クエリ構造体を組む** — すべて `Default` 実装。必要なフィールドだけ設定して
   `..Default::default()` で埋める。日付は `"YYYY-MM-DD"` 文字列
3. **戻り値の型を確認する** — 通常は `Vec<T>`（ページ分割は自動追従済み・全件結合）。
   `fins_summary` / `fins_details` / `td_list` は `CursorPage<T>`（`items` + `cursor`）。
   cursor は保存して次回の query に渡すと差分取得になる
4. **数値フィールドの型に注意** — `FlexString` の項目は `.as_f64()` で数値化する
   （API が string/number/null を混在させるため）。`DailyBar` の価格は `Option<f64>`
5. **レート制限を設定する** — ユーザーの契約プランが分かるなら
   `client.with_plan(Plan::Free)`（Free 5/Light 60/Standard 120/Premium 500 件/分）を付ける。
   既定は無制限なので、連続呼び出しするコードでは必ず付ける
6. **大量取得はループしない** — 全銘柄 × 長期間は `bulk_list` / `bulk_get` を使う

## 最小コード

```rust
use jquants_api::{CodeDateQuery, JQuantsClient};

#[tokio::main]
async fn main() -> Result<(), jquants_api::Error> {
    let client = JQuantsClient::from_env()?;
    let bars = client.daily_bars(&CodeDateQuery::code("7203")).await?;
    println!("{} 件", bars.len());
    Ok(())
}
```

## 注意

- Free プランは約 2 年分・12 週遅延。直近データを指定すると空か API エラーになる
  （バグではない）。プラン別範囲: https://jpx-jquants.com/ja/spec/data-spec
- EDINET 系（`edinet_*`）は Standard プラン以上。`fins_earnings_date` は
  code / date / scheduled_date のいずれか 1 つの指定が必須
- 株価ティック（/equities/trades）は CSV 提供のみで専用メソッドは無い。
  `bulk_list` / `bulk_get` に endpoint "/equities/trades" を指定して取得する
- HTTP 210 は「データ未提供」。エラーにならず空の結果が返る仕様
- 取得データの再配布は J-Quants 規約で禁止。テストに実データを埋め込まない
