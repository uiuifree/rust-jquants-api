//! EDINET 3 エンドポイントの疎通確認。指定日に何件返るかを表示する。
//!
//! ```sh
//! cargo run --example edinet_probe -- 2025-06-02
//! ```

use jquants_api::{EdinetQuery, Error, JQuantsClient};

async fn probe(client: &JQuantsClient, date: &str) -> Result<(), Box<dyn std::error::Error>> {
    let q = EdinetQuery {
        date: Some(date.to_string()),
        ..Default::default()
    };

    macro_rules! show {
        ($label:expr, $call:expr) => {
            match $call {
                Ok(v) => println!("  {:<16} {} 件", $label, v.len()),
                Err(Error::Api { status, message }) => {
                    println!(
                        "  {:<16} API {status}: {}",
                        $label,
                        &message[..message.len().min(70)]
                    )
                }
                Err(e) => println!("  {:<16} エラー: {e}", $label),
            }
        };
    }

    println!("[{date}]");
    show!("大株主", client.edinet_major_shareholders(&q).await);
    std::thread::sleep(std::time::Duration::from_millis(1500));
    show!("政策保有", client.edinet_cross_shareholdings(&q).await);
    std::thread::sleep(std::time::Duration::from_millis(1500));
    show!(
        "大量保有",
        client.edinet_large_volume_shareholders(&q).await
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = JQuantsClient::from_env()?;

    let dates: Vec<String> = std::env::args().skip(1).collect();
    let dates = if dates.is_empty() {
        vec!["2025-06-02".to_string(), "2024-06-03".to_string()]
    } else {
        dates
    };

    for d in dates {
        probe(&client, &d).await?;
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
    Ok(())
}
