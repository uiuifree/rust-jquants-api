//! 指定銘柄の会社情報・直近の決算サマリー・決算発表予定日を表示する（全プランで利用可）。
//!
//! ```sh
//! export JQUANTS_API_KEY=your_api_key
//! cargo run --example company -- 215A0
//! ```

use jquants_api::{EarningsDateQuery, Error, FinsQuery, FlexString, JQuantsClient, MasterQuery};

/// 429（レート制限）のとき 30 秒待って再試行する
macro_rules! retry {
    ($call:expr) => {{
        let mut result = $call.await;
        for _ in 0..10 {
            match &result {
                Err(Error::Api { status: 429, .. }) => {
                    eprintln!("(429 rate limit: 30秒待機)");
                    std::thread::sleep(std::time::Duration::from_secs(30));
                    result = $call.await;
                }
                _ => break,
            }
        }
        result
    }};
}

fn oku(v: &FlexString) -> String {
    match v.as_f64() {
        Some(n) => format!("{:.1}億円", n / 1e8),
        None => "-".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "215A0".to_string());

    dotenvy::dotenv().ok();
    let client = JQuantsClient::from_env()?;

    let master = retry!(client.master(&MasterQuery {
        code: Some(code.clone()),
        ..Default::default()
    }))?;
    match master.first() {
        Some(m) => println!(
            "銘柄: {} {} ({} / {} / 規模: {})",
            m.code, m.co_name, m.market_code_name, m.sector33_code_name, m.scale_category
        ),
        None => println!("銘柄マスタに {code} が見つからない"),
    }

    let fins = retry!(client.fins_summary(&FinsQuery {
        code: Some(code.clone()),
        ..Default::default()
    }))?;
    println!("決算サマリー: {} 件", fins.items.len());
    for f in fins.items.iter().rev().take(6) {
        println!(
            "  {} [{} {}] 売上 {} / 営業利益 {} / 純利益 {} / EPS {}",
            f.disc_date,
            f.doc_type,
            f.cur_per_type,
            oku(&f.sales),
            oku(&f.op),
            oku(&f.np),
            f.eps
        );
    }

    let dates = retry!(client.fins_earnings_date(&EarningsDateQuery {
        code: Some(code.clone()),
        ..Default::default()
    }))?;
    if let Some(d) = dates.last() {
        let sch = if d.sch_date.is_empty() {
            "未定"
        } else {
            &d.sch_date
        };
        println!("次回決算発表予定: {} ({})", sch, d.fq_name);
    }
    Ok(())
}
