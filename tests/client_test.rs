//! モックサーバーに対する結合テスト。
//! フィクスチャはすべて架空の値（実際の API 応答データは含めない）。

use jquants_api::{
    BulkGetQuery, CalendarQuery, CodeDateQuery, EarningsDateQuery, EdinetQuery, Error, FinsQuery,
    JQuantsClient, MasterQuery, Plan,
};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client_for(server: &MockServer) -> JQuantsClient {
    JQuantsClient::new("test-key").with_base_url(server.uri())
}

fn daily_bar(date: &str, close: f64) -> serde_json::Value {
    json!({
        "Date": date,
        "Code": "00010",
        "O": 100.0,
        "H": 110.0,
        "L": 90.0,
        "C": close,
        "Vo": 1000.0,
        "Va": 100000.0,
        "AdjFactor": 1.0,
        "AdjC": close
    })
}

#[tokio::test]
async fn daily_bars_follows_pagination_and_sends_api_key() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/equities/bars/daily"))
        .and(header("x-api-key", "test-key"))
        .and(query_param("code", "00010"))
        .and(query_param_is_missing("pagination_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [daily_bar("2026-01-05", 105.0)],
            "pagination_key": "page2"
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/equities/bars/daily"))
        .and(header("x-api-key", "test-key"))
        .and(query_param("code", "00010"))
        .and(query_param("pagination_key", "page2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [daily_bar("2026-01-06", 106.0)],
            "pagination_key": null
        })))
        .expect(1)
        .mount(&server)
        .await;

    let bars = client_for(&server)
        .daily_bars(&CodeDateQuery::code("00010"))
        .await
        .unwrap();

    assert_eq!(bars.len(), 2);
    assert_eq!(bars[0].date, "2026-01-05");
    assert_eq!(bars[0].close, Some(105.0));
    assert_eq!(bars[1].date, "2026-01-06");
    // 省略されたフィールドは None になる
    assert!(bars[0].morning_open.is_none());
}

#[tokio::test]
async fn master_decodes_all_fields() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/equities/master"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{
                "Date": "2026-01-05",
                "Code": "00010",
                "CoName": "テスト株式会社",
                "CoNameEn": "Test Co., Ltd.",
                "S17": "99",
                "S17Nm": "テスト業種17",
                "S33": "9999",
                "S33Nm": "テスト業種33",
                "ScaleCat": "-",
                "Mkt": "0000",
                "MktNm": "テスト市場",
                "Mrgn": "1",
                "MrgnNm": "貸借"
            }],
            "pagination_key": null
        })))
        .mount(&server)
        .await;

    let rows = client_for(&server)
        .master(&MasterQuery::default())
        .await
        .unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].code, "00010");
    assert_eq!(rows[0].co_name, "テスト株式会社");
    assert_eq!(rows[0].market_code_name, "テスト市場");
}

#[tokio::test]
async fn api_error_is_mapped_with_status_and_message() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/equities/bars/daily"))
        .respond_with(
            ResponseTemplate::new(401).set_body_json(json!({"message": "invalid api key"})),
        )
        .mount(&server)
        .await;

    let err = client_for(&server)
        .daily_bars(&CodeDateQuery::default())
        .await
        .unwrap_err();

    match err {
        Error::Api { status, message } => {
            assert_eq!(status, 401);
            assert_eq!(message, "invalid api key");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn status_210_returns_accumulated_items() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/markets/calendar"))
        .respond_with(ResponseTemplate::new(210))
        .mount(&server)
        .await;

    let rows = client_for(&server)
        .trading_calendar(&CalendarQuery::default())
        .await
        .unwrap();

    assert!(rows.is_empty());
}

#[tokio::test]
async fn decode_error_includes_body_snippet() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/equities/bars/daily"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;

    let err = client_for(&server)
        .daily_bars(&CodeDateQuery::default())
        .await
        .unwrap_err();

    match err {
        Error::Decode { body, .. } => assert_eq!(body, "not-json"),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn fins_summary_returns_cursor() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fins/summary"))
        .and(query_param("code", "00010"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [],
            "pagination_key": null,
            "cursor": "cursor-token-1"
        })))
        .mount(&server)
        .await;

    let page = client_for(&server)
        .fins_summary(&FinsQuery {
            code: Some("00010".into()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert!(page.items.is_empty());
    assert_eq!(page.cursor.as_deref(), Some("cursor-token-1"));
}

#[tokio::test]
async fn bulk_get_normalizes_endpoint_and_returns_url() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/bulk/get"))
        .and(query_param("endpoint", "/equities/bars/daily"))
        .and(query_param("key", "dir/file.csv.gz"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"url": "https://example.invalid/file.csv.gz?sig=x"})),
        )
        .mount(&server)
        .await;

    let url = client_for(&server)
        .bulk_get(&BulkGetQuery {
            key: Some("dir/file.csv.gz".into()),
            endpoint: Some("equities/bars/daily".into()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(url, "https://example.invalid/file.csv.gz?sig=x");
}

#[tokio::test]
async fn rate_limited_client_completes_paginated_requests() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/equities/bars/daily"))
        .and(query_param_is_missing("pagination_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [daily_bar("2026-01-05", 105.0)],
            "pagination_key": "page2"
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/equities/bars/daily"))
        .and(query_param("pagination_key", "page2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [daily_bar("2026-01-06", 106.0)],
            "pagination_key": null
        })))
        .expect(1)
        .mount(&server)
        .await;

    // Premium 相当（500件/分）なら 2 ページ取得しても待機は発生しない
    let bars = JQuantsClient::new("test-key")
        .with_base_url(server.uri())
        .with_plan(Plan::Premium)
        .daily_bars(&CodeDateQuery::default())
        .await
        .unwrap();

    assert_eq!(bars.len(), 2);
}

#[tokio::test]
async fn fins_earnings_date_decodes_rows() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fins/earnings-date"))
        .and(query_param("code", "00010"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{
                "PubDate": "2026-01-10",
                "SchDate": "2026-02-10",
                "FQName": "3Q",
                "FYE": "0331",
                "Code": "00010",
                "CoName": "テスト株式会社",
                "CoNameEn": "Test Co., Ltd."
            }],
            "pagination_key": null
        })))
        .mount(&server)
        .await;

    let rows = client_for(&server)
        .fins_earnings_date(&EarningsDateQuery {
            code: Some("00010".into()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].sch_date, "2026-02-10");
    assert_eq!(rows[0].fq_name, "3Q");
}

#[tokio::test]
async fn edinet_large_volume_decodes_nested_holders() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/edinet/large-volume-shareholders"))
        .and(query_param("edinet_code", "E00001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{
                "DocId": "S000TEST",
                "Code": "00010",
                "EdinetCode": "E00001",
                "IsrName": "テスト株式会社",
                "DocTypeCode": "350",
                "SubDate": "2026-01-10",
                "SubTime": "09:00",
                "LargeHldgTypeCode": "01",
                "DocTitle": "大量保有報告書",
                "ChgRsn": null,
                "TotalShsHeld": 1000000,
                "TotalOutStks": "10000000",
                "TotalShsRatio": 10.0,
                "TotalShsRatioLast": null,
                "Hldrs": [{
                    "HldrName": "テスト保有者",
                    "HldrNameEn": "Test Holder",
                    "HldrEdinetCode": "E00002",
                    "HldrTypeCode": "1",
                    "ShsHeld": 1000000,
                    "OwnFund": 500,
                    "TotalBrw": null,
                    "TotalFund": 500,
                    "AcqDisp": [{"AnyKey": "AnyValue"}],
                    "BrwList": [],
                    "CredList": []
                }]
            }],
            "pagination_key": null
        })))
        .mount(&server)
        .await;

    let docs = client_for(&server)
        .edinet_large_volume_shareholders(&EdinetQuery {
            edinet_code: Some("E00001".into()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(docs.len(), 1);
    // string/number/null が混在しても FlexString で受かる
    assert_eq!(docs[0].total_shs_held.as_f64(), Some(1_000_000.0));
    assert_eq!(docs[0].total_out_stks.as_f64(), Some(10_000_000.0));
    assert!(docs[0].total_shs_ratio_last.is_empty());
    assert_eq!(docs[0].hldrs.len(), 1);
    assert_eq!(docs[0].hldrs[0].shs_held.as_f64(), Some(1_000_000.0));
    assert_eq!(docs[0].hldrs[0].acq_disp.len(), 1);
}

#[tokio::test]
async fn td_files_decodes_object_response() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/td/files"))
        .and(query_param("discNo", "20260105000001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "discNo": "20260105000001",
            "files": {
                "pdf": "https://example.invalid/a.pdf",
                "summaryPdf": null,
                "xbrl": null
            }
        })))
        .mount(&server)
        .await;

    let files = client_for(&server)
        .td_files("20260105000001", None)
        .await
        .unwrap();

    assert_eq!(files.disc_no, "20260105000001");
    let inner = files.files.expect("files が返っている");
    assert_eq!(inner.pdf.as_deref(), Some("https://example.invalid/a.pdf"));
    assert!(inner.xbrl.is_none());
}

/// 書類が1本も無い開示では `files` が null で返る。
/// 2025-11-05 の招集通知など、実際にデコードで落ちた（過去5年で700件ほどある）
#[tokio::test]
async fn td_files_accepts_null_files() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/td/files"))
        .and(query_param("discNo", "20251104586107"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "discNo": "20251104586107",
            "files": null
        })))
        .mount(&server)
        .await;

    let files = client_for(&server)
        .td_files("20251104586107", None)
        .await
        .unwrap();

    assert_eq!(files.disc_no, "20251104586107");
    assert!(files.files.is_none());
}

#[tokio::test]
async fn edinet_accepts_null_string_fields() {
    // 非上場の提出者は Code が null になる（実 API で発生した応答パターン）
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/edinet/major-shareholders"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{
                "DocId": "S000TEST",
                "Code": null,
                "EdinetCode": "E00001",
                "FilerName": "テスト株式会社",
                "FilerNameEn": null,
                "DocTypeCode": "120",
                "SubDate": "2026-01-05",
                "SubTime": "15:19:00",
                "PerSt": "2025-01-01",
                "PerEn": "2025-12-31",
                "Hldrs": [{
                    "Rank": 1,
                    "HldrName": "テスト保有者",
                    "HldrAddr": null,
                    "ShsHeld": 207070600,
                    "ShsRatio": 0.3835
                }]
            }],
            "pagination_key": null
        })))
        .mount(&server)
        .await;

    let docs = client_for(&server)
        .edinet_major_shareholders(&EdinetQuery::default())
        .await
        .unwrap();

    assert_eq!(docs.len(), 1);
    assert!(docs[0].code.is_empty()); // null は空文字列になる
    assert_eq!(&*docs[0].filer_name, "テスト株式会社");
    assert_eq!(docs[0].hldrs[0].shs_held.as_f64(), Some(207_070_600.0));
    assert_eq!(docs[0].hldrs[0].shs_ratio.as_f64(), Some(0.3835));
}

#[tokio::test]
async fn edinet_tolerates_missing_and_renamed_fields() {
    // 実 API では書類種別により項目自体が欠け、保有者区分は
    // LargeHldrTypeCode という名前で返ることがある
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/edinet/large-volume-shareholders"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{
                "DocId": "S000TEST",
                "Code": "00010",
                "EdinetCode": "E00001",
                "IsrName": "テスト株式会社",
                "DocTypeCode": "350",
                "SubDate": "2026-01-05",
                "SubTime": "15:54:00",
                "LargeHldgTypeCode": "1",
                "DocTitle": "大量保有報告書",
                "ChgRsn": null,
                "TotalShsHeld": null,
                "TotalOutStks": 40050000,
                "Hldrs": [{
                    "HldrName": "テスト投資会社",
                    "HldrEdinetCode": "E00002",
                    "HldrCode": null,
                    "LargeHldrTypeCode": "2"
                }]
            }],
            "pagination_key": null
        })))
        .mount(&server)
        .await;

    let docs = client_for(&server)
        .edinet_large_volume_shareholders(&EdinetQuery::default())
        .await
        .unwrap();

    assert_eq!(docs.len(), 1);
    // 欠落した項目は空、null も空になる
    assert!(docs[0].total_shs_ratio.is_empty());
    assert!(docs[0].total_shs_held.is_empty());
    assert_eq!(docs[0].total_out_stks.as_f64(), Some(40_050_000.0));
    // 別名でも保有者区分が読める
    assert_eq!(&*docs[0].hldrs[0].hldr_type_code, "2");
    assert!(docs[0].hldrs[0].shs_held.is_empty());
}
