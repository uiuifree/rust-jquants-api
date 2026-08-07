# jquants-api — J-Quants API v2 Rust クライアント

[![CI](https://github.com/uiuifree/rust-jquants-api/actions/workflows/ci.yml/badge.svg)](https://github.com/uiuifree/rust-jquants-api/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/jquants-api.svg)](https://crates.io/crates/jquants-api)
[![docs.rs](https://img.shields.io/docsrs/jquants-api)](https://docs.rs/jquants-api)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

日本株の株価・財務・市場データを取得できる [J-Quants API](https://jpx-jquants.com/)（JPX 総研提供）の**非公式** async Rust クライアントです。株価日足・分足、財務諸表、信用取引残高、空売り比率、先物・オプション、TOPIX・指数、適時開示まで、v2 の全データ系エンドポイントを型付きで扱えます。

Unofficial async Rust client for the **J-Quants API v2** — Japanese stock market data
(equities, financial statements, margin trading, short selling, derivatives, indices,
timely disclosure) provided by JPX Market Innovation & Research. Requires a J-Quants
subscription (free plan available). See [`llms.txt`](./llms.txt) for an AI-friendly
API reference.

## 特徴

- **J-Quants API v2 対応**（`https://api.jquants.com/v2`・`x-api-key` 認証）
- **全 29 データ系エンドポイントを型付きでカバー** — 株価四本値（日足・前場・分足）、上場銘柄マスタ、決算発表予定日、投資部門別売買状況、売買内訳、営業日カレンダー、信用取引残高、空売り比率・残高報告、指数・TOPIX、先物・オプション、財務諸表サマリー・詳細・配当、EDINET（大株主・政策保有株式・大量保有報告書）、適時開示、一括ダウンロード
- **ページ分割の自動追従** — `pagination_key` をライブラリが処理し、全件を結合して返す
- **差分取得（cursor）対応** — 財務諸表・適時開示は前回の続きから取得できる
- **ゆらぎのある応答を吸収** — string / number / null が混在する項目は `FlexString`（`as_f64()` 付き）で安全に受け取る
- **プラン別レート制限に対応** — `with_plan(Plan::Light)` で契約プランの API コール制限（Free 5 件/分〜Premium 500 件/分）に合わせて自動待機。任意の値も `with_rate_limit(件数/分)` で指定可能
- ランタイムは tokio、TLS は rustls（OpenSSL 不要）

## インストール

```toml
[dependencies]
jquants-api = "0.1"
tokio = { version = "1", features = ["full"] }
```

## クイックスタート

API キーを [J-Quants Dashboard](https://jpx-jquants.com/dashboard/api-keys) で発行し、環境変数に設定します（無料プランで発行できます）。

```sh
export JQUANTS_API_KEY=your_api_key
```

```rust
use jquants_api::{CodeDateQuery, JQuantsClient};

#[tokio::main]
async fn main() -> Result<(), jquants_api::Error> {
    let client = JQuantsClient::from_env()?;

    // トヨタ自動車（7203）の日足を期間指定で取得
    let bars = client
        .daily_bars(&CodeDateQuery {
            code: Some("7203".into()),
            from: Some("2026-01-01".into()),
            to: Some("2026-03-31".into()),
            ..Default::default()
        })
        .await?;

    for bar in &bars {
        println!("{} 終値={:?} 調整後終値={:?}", bar.date, bar.close, bar.adj_close);
    }
    Ok(())
}
```

### 使用例: 財務諸表を差分取得する

`/fins/summary` `/fins/details` `/td/list` は `cursor` を返します。保存しておき、次回の呼び出しに渡すと前回以降の差分だけ取得できます。

```rust
use jquants_api::{FinsQuery, JQuantsClient};

async fn sync(client: &JQuantsClient, saved_cursor: Option<String>) -> Result<Option<String>, jquants_api::Error> {
    let page = client
        .fins_summary(&FinsQuery { cursor: saved_cursor, ..Default::default() })
        .await?;

    for row in &page.items {
        println!("{} {} 売上={}", row.disc_date, row.code, row.sales);
    }
    Ok(page.cursor) // 保存して次回に渡す
}
```

### 使用例: 営業日カレンダー・TOPIX

```rust
use jquants_api::{CalendarQuery, JQuantsClient, RangeQuery};

async fn market_info(client: &JQuantsClient) -> Result<(), jquants_api::Error> {
    let days = client
        .trading_calendar(&CalendarQuery {
            from: Some("2026-01-01".into()),
            to: Some("2026-01-31".into()),
            ..Default::default()
        })
        .await?;
    println!("1月の営業日データ: {} 件", days.len());

    let topix = client
        .topix_daily_bars(&RangeQuery {
            from: Some("2026-01-01".into()),
            ..Default::default()
        })
        .await?;
    println!("TOPIX: {} 件", topix.len());
    Ok(())
}
```

## 対応エンドポイント

| メソッド | エンドポイント | 内容 |
|---|---|---|
| `master` | `/equities/master` | 上場銘柄マスタ |
| `daily_bars` | `/equities/bars/daily` | 株価日足（調整前後・前後場込み） |
| `am_bars` | `/equities/bars/daily/am` | 前場終値時点の株価 |
| `minute_bars` | `/equities/bars/minute` | 株価分足 |
| `earnings_calendar` | `/equities/earnings-calendar` | 決算発表予定 |
| `investor_types` | `/equities/investor-types` | 投資部門別売買状況 |
| `breakdown` | `/markets/breakdown` | 売買内訳 |
| `trading_calendar` | `/markets/calendar` | 営業日カレンダー |
| `margin_alert` | `/markets/margin-alert` | 日々公表信用取引残高 |
| `margin_interest` | `/markets/margin-interest` | 信用取引週末残高 |
| `short_ratio` | `/markets/short-ratio` | 業種別空売り比率 |
| `short_sale_report` | `/markets/short-sale-report` | 空売り残高報告 |
| `index_daily_bars` | `/indices/bars/daily` | 指数日足 |
| `topix_daily_bars` | `/indices/bars/daily/topix` | TOPIX 日足 |
| `futures_bars` | `/derivatives/bars/daily/futures` | 先物四本値 |
| `options_bars` | `/derivatives/bars/daily/options` | オプション四本値 |
| `options_225_bars` | `/derivatives/bars/daily/options/225` | 日経225オプション四本値 |
| `fins_summary` | `/fins/summary` | 財務諸表サマリー（cursor 対応） |
| `fins_details` | `/fins/details` | 財務諸表詳細（cursor 対応） |
| `fins_dividend` | `/fins/dividend` | 配当金情報 |
| `fins_earnings_date` | `/fins/earnings-date` | 決算発表予定日 |
| `edinet_major_shareholders` | `/edinet/major-shareholders` | 大株主の状況（Standard 以上） |
| `edinet_cross_shareholdings` | `/edinet/cross-shareholdings` | 政策保有株式（Standard 以上） |
| `edinet_large_volume_shareholders` | `/edinet/large-volume-shareholders` | 大量保有報告書（Standard 以上） |
| `td_list` | `/td/list` | 適時開示一覧（cursor 対応） |
| `td_bulk` | `/td/bulk` | 適時開示の一括ダウンロード情報 |
| `td_files` | `/td/files` | 適時開示ファイル（PDF / XBRL） |
| `bulk_list` | `/bulk/list` | 一括ダウンロード可能ファイル一覧 |
| `bulk_get` | `/bulk/get` | 一括ダウンロード URL 取得 |

株価ティック（`/equities/trades`）は API の仕様上 CSV ファイル提供のみのため、
専用メソッドはありません。`bulk_list` / `bulk_get` に `endpoint: "/equities/trades"` を
指定してダウンロード URL を取得してください。

## プランとデータ範囲

利用できるエンドポイントとデータ期間は J-Quants の契約プラン（Free / Light / Standard / Premium）で異なります。目安として Free は約 2 年分（12 週遅延）、上位プランほど過去データが長く・遅延なしになります。正確な範囲は [J-Quants API Reference](https://jpx-jquants.com/ja/spec/data-spec) を確認してください。

Free プランで取得できないエンドポイント・期間を指定した場合、このライブラリは `Error::Api { status, message }` を返します。

## エラーハンドリング

```rust
use jquants_api::{CodeDateQuery, Error, JQuantsClient};

async fn handle(client: &JQuantsClient) {
    match client.daily_bars(&CodeDateQuery::code("7203")).await {
        Ok(bars) => println!("{} 件", bars.len()),
        Err(Error::Api { status: 401, .. }) => eprintln!("API キーが無効"),
        Err(Error::Api { status: 429, .. }) => eprintln!("レート制限。間隔を空けて再試行"),
        Err(Error::Api { status, message }) => eprintln!("API エラー {status}: {message}"),
        Err(e) => eprintln!("通信・デコードエラー: {e}"),
    }
}
```

### レート制限

契約プランを指定すると、API コール制限を超えないようリクエスト間隔を自動調整します（超えそうな場合は送信を待機するだけで、エラーにはなりません）。ページ分割の内部リクエストにも 1 件ずつ適用されます。

```rust
use jquants_api::{JQuantsClient, Plan};

let client = JQuantsClient::from_env()?
    .with_plan(Plan::Free);        // Free プラン: 5 件/分
// .with_plan(Plan::Light)        // 60 件/分
// .with_plan(Plan::Standard)     // 120 件/分
// .with_plan(Plan::Premium)      // 500 件/分
// .with_rate_limit(30)           // 任意の値（件/分）も指定可能
```

既定ではレート制限は無効です。プリセット値は 2026-08 時点の公式料金ページに基づくため、変更されていないか [J-Quants の料金プラン](https://jpx-jquants.com/)で確認してください。大量取得はレート制限にかかわらず `bulk_list` / `bulk_get`（一括ダウンロード）の利用を検討してください。

## AI エージェント・LLM で使う

コーディングエージェント（Claude Code、Cursor、Copilot など）にこのクレートを使わせる場合:

- [`llms.txt`](./llms.txt) — 全メソッドのシグネチャ・型・注意点をまとめた機械可読リファレンス。エージェントのコンテキストに渡すとそのまま正しいコードを書けます
- [`.claude/skills/jquants-api-usage/SKILL.md`](./.claude/skills/jquants-api-usage/SKILL.md) — Claude Code 用のスキル定義

## 開発

```sh
cargo test          # オフラインで完結（モックサーバー使用）
```

`JQUANTS_API_KEY` を設定（環境変数または `.env`）すると `tests/live_test.rs` が実 API への疎通テストとして実行されます（未設定ならスキップ）。

```sh
cargo test --test live_test              # 実 API 疎通テスト
cargo run --example daily_bars -- 7203   # 動作確認
```

## 注意事項

- **非公式クレートです。** JPX・JPX 総研とは無関係のコミュニティ実装で、API 仕様・利用条件の正は J-Quants 公式ドキュメントです
- J-Quants API は個人の私的利用に限定されています。取得したデータの再配布・分析結果の継続的な第三者提供は利用規約で禁止されています
- 本クレートは投資助言を行うものではありません。投資判断は自己責任で行ってください

## 謝辞

エンドポイント・レスポンス型の定義は JPX Market Innovation & Research, Inc. の公式 CLI [J-Quants/jquants-cli](https://github.com/J-Quants/jquants-cli)（MIT License）を参考にしています。

## License

MIT または Apache-2.0 のデュアルライセンス。

Licensed under either of [MIT license](./LICENSE-MIT) or [Apache License 2.0](./LICENSE-APACHE) at your option.
