//! 指定銘柄の直近の適時開示一覧と、最新開示の書類 URL（PDF / XBRL）を表示する。
//!
//! ```sh
//! export JQUANTS_API_KEY=your_api_key
//! cargo run --example disclosures -- 215A0
//! ```

use jquants_api::{Error, JQuantsClient, MasterQuery, TdListQuery};

/// 429（レート制限）のとき 30 秒待って再試行する
macro_rules! retry {
    ($call:expr) => {{
        let mut result = $call.await;
        for _ in 0..5 {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "215A0".to_string());

    dotenvy::dotenv().ok();
    let client = JQuantsClient::from_env()?;

    // 銘柄名を確認
    let master = retry!(client.master(&MasterQuery {
        code: Some(code.clone()),
        ..Default::default()
    }))?;
    match master.first() {
        Some(m) => println!(
            "銘柄: {} {} ({} / {})",
            m.code, m.co_name, m.market_code_name, m.sector33_code_name
        ),
        None => println!("銘柄マスタに {code} が見つからない（コード確認を推奨）"),
    }

    // 適時開示の一覧
    let page = retry!(client.td_list(&TdListQuery {
        code: Some(code.clone()),
        ..Default::default()
    }))?;
    println!("適時開示: {} 件", page.items.len());
    for d in page.items.iter().rev().take(10) {
        println!(
            "  {} {} [{}] {}",
            d.disc_date, d.disc_time, d.disc_no, d.title
        );
    }

    // 最新開示の書類 URL
    if let Some(latest) = page.items.last() {
        let got = retry!(client.td_files(&latest.disc_no, None))?;
        println!("最新開示の書類 ({}):", latest.title);
        match &got.files {
            // 書類が1本も無い開示（招集通知など）では files が null で返る
            None => println!("  （この開示に書類は付いていない）"),
            Some(files) => {
                if let Some(pdf) = &files.pdf {
                    println!("  PDF:  {pdf}");
                }
                if let Some(s) = &files.summary_pdf {
                    println!("  要約: {s}");
                }
                if let Some(x) = &files.xbrl {
                    println!("  XBRL: {x}");
                }
            }
        }
    }
    Ok(())
}
