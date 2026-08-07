//! 疎通確認用: 指定銘柄の直近の日足を取得して表示する。
//!
//! ```sh
//! export JQUANTS_API_KEY=your_api_key
//! cargo run --example daily_bars -- 7203
//! ```

use jquants_api::{CodeDateQuery, JQuantsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "7203".to_string());

    dotenvy::dotenv().ok();
    let client = JQuantsClient::from_env()?;
    let bars = client.daily_bars(&CodeDateQuery::code(&code)).await?;

    println!("{} 件取得", bars.len());
    for bar in bars.iter().rev().take(5) {
        println!(
            "{} O={:?} H={:?} L={:?} C={:?} (調整後C={:?})",
            bar.date, bar.open, bar.high, bar.low, bar.close, bar.adj_close
        );
    }
    Ok(())
}
