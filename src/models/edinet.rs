//! EDINET 系エンドポイントのモデル。**Standard プラン以上**（アドオン扱い）。
//! フィールド定義は公式リファレンス（2026-08 時点）に基づく。
//! 実応答での型ゆらぎに備え、数値系は [`FlexString`]、
//! 仕様が公開されていない深い入れ子は `serde_json::Value` で受ける。

use super::flex::FlexString;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 大株主の状況（`/edinet/major-shareholders`）の書類 1 件
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MajorShareholdersDoc {
    /// 書類管理番号
    #[serde(rename = "DocId")]
    #[serde(default)]
    pub doc_id: FlexString,
    /// 銘柄コード
    #[serde(rename = "Code")]
    #[serde(default)]
    pub code: FlexString,
    /// 提出者の EDINET コード
    #[serde(rename = "EdinetCode")]
    #[serde(default)]
    pub edinet_code: FlexString,
    /// 提出者名
    #[serde(rename = "FilerName")]
    #[serde(default)]
    pub filer_name: FlexString,
    /// 提出者名（英語）
    #[serde(rename = "FilerNameEn")]
    #[serde(default)]
    pub filer_name_en: FlexString,
    /// 書類種別コード
    #[serde(rename = "DocTypeCode")]
    #[serde(default)]
    pub doc_type_code: FlexString,
    /// 提出日 (YYYY-MM-DD)
    #[serde(rename = "SubDate")]
    #[serde(default)]
    pub sub_date: FlexString,
    /// 提出時刻
    #[serde(rename = "SubTime")]
    #[serde(default)]
    pub sub_time: FlexString,
    /// 対象期間開始日
    #[serde(rename = "PerSt")]
    #[serde(default)]
    pub per_st: FlexString,
    /// 対象期間終了日
    #[serde(rename = "PerEn")]
    #[serde(default)]
    pub per_en: FlexString,
    /// 大株主の一覧
    #[serde(rename = "Hldrs")]
    #[serde(default, deserialize_with = "crate::models::flex::null_as_default")]
    pub hldrs: Vec<MajorShareholder>,
}

/// 大株主 1 名分の情報
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MajorShareholder {
    /// 順位
    #[serde(rename = "Rank")]
    #[serde(default)]
    pub rank: FlexString,
    /// 大株主の氏名・名称
    #[serde(rename = "HldrName")]
    #[serde(default)]
    pub hldr_name: FlexString,
    /// 大株主の住所・所在地
    #[serde(rename = "HldrAddr")]
    #[serde(default)]
    pub hldr_addr: FlexString,
    /// 所有株式数
    #[serde(rename = "ShsHeld")]
    #[serde(default)]
    pub shs_held: FlexString,
    /// 発行済株式総数に対する所有株式数の割合
    #[serde(rename = "ShsRatio")]
    #[serde(default)]
    pub shs_ratio: FlexString,
}

/// 政策保有株式（`/edinet/cross-shareholdings`）の書類 1 件
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CrossShareholdingsDoc {
    /// 書類管理番号
    #[serde(rename = "DocId")]
    #[serde(default)]
    pub doc_id: FlexString,
    /// 銘柄コード
    #[serde(rename = "Code")]
    #[serde(default)]
    pub code: FlexString,
    /// 提出者の EDINET コード
    #[serde(rename = "EdinetCode")]
    #[serde(default)]
    pub edinet_code: FlexString,
    /// 提出者名
    #[serde(rename = "FilerName")]
    #[serde(default)]
    pub filer_name: FlexString,
    /// 提出者名（英語）
    #[serde(rename = "FilerNameEn")]
    #[serde(default)]
    pub filer_name_en: FlexString,
    /// 書類種別コード
    #[serde(rename = "DocTypeCode")]
    #[serde(default)]
    pub doc_type_code: FlexString,
    /// 提出日 (YYYY-MM-DD)
    #[serde(rename = "SubDate")]
    #[serde(default)]
    pub sub_date: FlexString,
    /// 提出時刻
    #[serde(rename = "SubTime")]
    #[serde(default)]
    pub sub_time: FlexString,
    /// 対象期間開始日
    #[serde(rename = "PerSt")]
    #[serde(default)]
    pub per_st: FlexString,
    /// 対象期間終了日
    #[serde(rename = "PerEn")]
    #[serde(default)]
    pub per_en: FlexString,
    /// 提出会社の保有状況
    #[serde(rename = "Report")]
    #[serde(default)]
    pub report: Option<CrossShareholdingHolder>,
    /// 最大保有会社の保有状況
    #[serde(rename = "Largest")]
    #[serde(default)]
    pub largest: Option<CrossShareholdingHolder>,
    /// 第 2 位保有会社の保有状況
    #[serde(rename = "SecondLargest")]
    #[serde(default)]
    pub second_largest: Option<CrossShareholdingHolder>,
}

/// 政策保有株式の保有主体（提出会社・最大保有会社・第 2 位保有会社共通）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CrossShareholdingHolder {
    /// 保有会社名
    #[serde(rename = "HldrName")]
    #[serde(default)]
    pub hldr_name: FlexString,
    /// 保有会社の銘柄コード
    #[serde(rename = "HldrCode")]
    #[serde(default)]
    pub hldr_code: FlexString,
    /// 保有会社の EDINET コード
    #[serde(rename = "HldrEdinetCode")]
    #[serde(default)]
    pub hldr_edinet_code: FlexString,
    /// 上場株式の銘柄数
    #[serde(rename = "ListedIss")]
    #[serde(default)]
    pub listed_iss: FlexString,
    /// 上場株式の貸借対照表計上額
    #[serde(rename = "ListedBookVal")]
    #[serde(default)]
    pub listed_book_val: FlexString,
    /// 上場株式の増加銘柄数
    #[serde(rename = "ListedIncIss")]
    #[serde(default)]
    pub listed_inc_iss: FlexString,
    /// 上場株式の増加に係る取得価額
    #[serde(rename = "ListedIncAcqCost")]
    #[serde(default)]
    pub listed_inc_acq_cost: FlexString,
    /// 上場株式の減少銘柄数
    #[serde(rename = "ListedDecIss")]
    #[serde(default)]
    pub listed_dec_iss: FlexString,
    /// 上場株式の減少に係る売却価額
    #[serde(rename = "ListedDecSaleAmt")]
    #[serde(default)]
    pub listed_dec_sale_amt: FlexString,
    /// 上場株式の増加理由
    #[serde(rename = "ListedIncRsn")]
    #[serde(default)]
    pub listed_inc_rsn: FlexString,
    /// 非上場株式の銘柄数
    #[serde(rename = "NonListedIss")]
    #[serde(default)]
    pub non_listed_iss: FlexString,
    /// 非上場株式の貸借対照表計上額
    #[serde(rename = "NonListedBookVal")]
    #[serde(default)]
    pub non_listed_book_val: FlexString,
    /// 非上場株式の増加銘柄数
    #[serde(rename = "NonListedIncIss")]
    #[serde(default)]
    pub non_listed_inc_iss: FlexString,
    /// 非上場株式の増加に係る取得価額
    #[serde(rename = "NonListedIncAcqCost")]
    #[serde(default)]
    pub non_listed_inc_acq_cost: FlexString,
    /// 非上場株式の減少銘柄数
    #[serde(rename = "NonListedDecIss")]
    #[serde(default)]
    pub non_listed_dec_iss: FlexString,
    /// 非上場株式の減少に係る売却価額
    #[serde(rename = "NonListedDecSaleAmt")]
    #[serde(default)]
    pub non_listed_dec_sale_amt: FlexString,
    /// 非上場株式の増加理由
    #[serde(rename = "NonListedIncRsn")]
    #[serde(default)]
    pub non_listed_inc_rsn: FlexString,
    /// 特定投資株式の明細
    #[serde(rename = "Spec")]
    #[serde(default, deserialize_with = "crate::models::flex::null_as_default")]
    pub spec: Vec<CrossShareholdingIssue>,
    /// みなし保有株式の明細
    #[serde(rename = "Deem")]
    #[serde(default, deserialize_with = "crate::models::flex::null_as_default")]
    pub deem: Vec<CrossShareholdingIssue>,
    /// 特定投資株式の注記
    #[serde(rename = "SpecFn")]
    #[serde(default)]
    pub spec_fn: FlexString,
    /// みなし保有株式の注記
    #[serde(rename = "DeemFn")]
    #[serde(default)]
    pub deem_fn: FlexString,
}

/// 政策保有株式の銘柄別明細（特定投資株式・みなし保有株式共通）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CrossShareholdingIssue {
    /// 発行者名
    #[serde(rename = "IsrName")]
    #[serde(default)]
    pub isr_name: FlexString,
    /// 発行者の銘柄コード
    #[serde(rename = "IsrCode")]
    #[serde(default)]
    pub isr_code: FlexString,
    /// 発行者の EDINET コード
    #[serde(rename = "IsrEdinetCode")]
    #[serde(default)]
    pub isr_edinet_code: FlexString,
    /// 当期の保有株式数
    #[serde(rename = "CurShs")]
    #[serde(default)]
    pub cur_shs: FlexString,
    /// 前期の保有株式数
    #[serde(rename = "PriShs")]
    #[serde(default)]
    pub pri_shs: FlexString,
    /// 当期の貸借対照表計上額
    #[serde(rename = "CurBookVal")]
    #[serde(default)]
    pub cur_book_val: FlexString,
    /// 前期の貸借対照表計上額
    #[serde(rename = "PriBookVal")]
    #[serde(default)]
    pub pri_book_val: FlexString,
    /// 当期の保有株式数（不開示分）
    #[serde(rename = "CurShsNotDisc")]
    #[serde(default)]
    pub cur_shs_not_disc: FlexString,
    /// 前期の保有株式数（不開示分）
    #[serde(rename = "PriShsNotDisc")]
    #[serde(default)]
    pub pri_shs_not_disc: FlexString,
    /// 当期の貸借対照表計上額（不開示分）
    #[serde(rename = "CurBookValNotDisc")]
    #[serde(default)]
    pub cur_book_val_not_disc: FlexString,
    /// 前期の貸借対照表計上額（不開示分）
    #[serde(rename = "PriBookValNotDisc")]
    #[serde(default)]
    pub pri_book_val_not_disc: FlexString,
    /// 保有割合
    #[serde(rename = "HoldRat")]
    #[serde(default)]
    pub hold_rat: FlexString,
    /// 発行者による相互保有の有無
    #[serde(rename = "IsrHolds")]
    #[serde(default)]
    pub isr_holds: FlexString,
    /// 発行者による相互保有の銘柄コード
    #[serde(rename = "IsrHoldsCode")]
    #[serde(default)]
    pub isr_holds_code: FlexString,
}

/// 大量保有報告書（`/edinet/large-volume-shareholders`）の書類 1 件
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LargeVolumeShareholdersDoc {
    /// 書類管理番号
    #[serde(rename = "DocId")]
    #[serde(default)]
    pub doc_id: FlexString,
    /// 発行者の銘柄コード
    #[serde(rename = "Code")]
    #[serde(default)]
    pub code: FlexString,
    /// 発行者の EDINET コード
    #[serde(rename = "EdinetCode")]
    #[serde(default)]
    pub edinet_code: FlexString,
    /// 発行者名
    #[serde(rename = "IsrName")]
    #[serde(default)]
    pub isr_name: FlexString,
    /// 書類種別コード
    #[serde(rename = "DocTypeCode")]
    #[serde(default)]
    pub doc_type_code: FlexString,
    /// 提出日 (YYYY-MM-DD)
    #[serde(rename = "SubDate")]
    #[serde(default)]
    pub sub_date: FlexString,
    /// 提出時刻
    #[serde(rename = "SubTime")]
    #[serde(default)]
    pub sub_time: FlexString,
    /// 報告義務発生事由コード
    #[serde(rename = "LargeHldgTypeCode")]
    #[serde(default)]
    pub large_hldg_type_code: FlexString,
    /// 書類タイトル
    #[serde(rename = "DocTitle")]
    #[serde(default)]
    pub doc_title: FlexString,
    /// 変更事由
    #[serde(rename = "ChgRsn")]
    #[serde(default)]
    pub chg_rsn: FlexString,
    /// 保有株式等の総数
    #[serde(rename = "TotalShsHeld")]
    #[serde(default)]
    pub total_shs_held: FlexString,
    /// 発行済株式等の総数
    #[serde(rename = "TotalOutStks")]
    #[serde(default)]
    pub total_out_stks: FlexString,
    /// 株式等保有割合
    #[serde(rename = "TotalShsRatio")]
    #[serde(default)]
    pub total_shs_ratio: FlexString,
    /// 直前の報告書の株式等保有割合
    #[serde(rename = "TotalShsRatioLast")]
    #[serde(default)]
    pub total_shs_ratio_last: FlexString,
    /// 提出者・共同保有者の一覧
    #[serde(rename = "Hldrs")]
    #[serde(default, deserialize_with = "crate::models::flex::null_as_default")]
    pub hldrs: Vec<LargeVolumeHolder>,
}

/// 大量保有報告書の提出者・共同保有者 1 名分の情報
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LargeVolumeHolder {
    /// 保有者名
    #[serde(rename = "HldrName")]
    #[serde(default)]
    pub hldr_name: FlexString,
    /// 保有者名（英語）
    #[serde(rename = "HldrNameEn")]
    #[serde(default)]
    pub hldr_name_en: FlexString,
    /// 保有者の EDINET コード
    #[serde(rename = "HldrEdinetCode")]
    #[serde(default)]
    pub hldr_edinet_code: FlexString,
    /// 保有者区分コード
    #[serde(rename = "HldrTypeCode", alias = "LargeHldrTypeCode")]
    #[serde(default)]
    pub hldr_type_code: FlexString,
    /// 保有株式等の数
    #[serde(rename = "ShsHeld")]
    #[serde(default)]
    pub shs_held: FlexString,
    /// 自己資金額
    #[serde(rename = "OwnFund")]
    #[serde(default)]
    pub own_fund: FlexString,
    /// 借入金額合計
    #[serde(rename = "TotalBrw")]
    #[serde(default)]
    pub total_brw: FlexString,
    /// 取得資金合計
    #[serde(rename = "TotalFund")]
    #[serde(default)]
    pub total_fund: FlexString,
    /// 最近 60 日間の取得・処分の状況（構造は公式リファレンス参照）
    #[serde(rename = "AcqDisp")]
    #[serde(default, deserialize_with = "crate::models::flex::null_as_default")]
    pub acq_disp: Vec<Value>,
    /// 借入先の一覧（構造は公式リファレンス参照）
    #[serde(rename = "BrwList")]
    #[serde(default, deserialize_with = "crate::models::flex::null_as_default")]
    pub brw_list: Vec<Value>,
    /// 信用取引等の一覧（構造は公式リファレンス参照）
    #[serde(rename = "CredList")]
    #[serde(default, deserialize_with = "crate::models::flex::null_as_default")]
    pub cred_list: Vec<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2021-07-02 の大量保有報告書で実際に落ちた応答。
    /// `ChgRsn` / `HldrCode` / `TotalShsRatioLast` が null で返る。
    /// `Code`・`HldrTypeCode` に続く3度目の同じ失敗だったため、
    /// 素の String を全廃し Vec も null を受けるようにした
    #[test]
    fn large_volume_accepts_nulls_seen_in_production() {
        let body = r#"{
          "DocId": "S100LIG7", "Code": "40690", "EdinetCode": "E36653",
          "IsrName": "株式会社ＢｌｕｅＭｅｍｅ", "DocTypeCode": "350",
          "SubDate": "2021-07-02", "SubTime": "13:47:00",
          "LargeHldgTypeCode": "1", "DocTitle": "大量保有報告書",
          "ChgRsn": null, "TotalShsHeld": 742500, "TotalShsRatio": 0.2267,
          "TotalShsRatioLast": null, "TotalOutStks": 3199946,
          "Hldrs": [{
            "HldrName": "松岡 真功", "HldrNameEn": "Masanori Matsuoka",
            "HldrEdinetCode": "E36650", "HldrCode": null,
            "LargeHldrTypeCode": "1", "LargeHldrTypeRaw": "個人",
            "AcqDisp": null, "BrwList": null, "CredList": null
          }]
        }"#;
        let d: LargeVolumeShareholdersDoc = serde_json::from_str(body).unwrap();
        assert_eq!(&*d.doc_id, "S100LIG7");
        assert!(d.chg_rsn.is_empty(), "null は空文字列になる");
        assert!(d.total_shs_ratio_last.is_empty());
        assert_eq!(d.hldrs.len(), 1);
        // LargeHldrTypeCode は別名として HldrTypeCode に入る
        assert_eq!(&*d.hldrs[0].hldr_type_code, "1");
        // null の配列は空として受ける
        assert!(d.hldrs[0].acq_disp.is_empty());
    }

    /// 識別子まで null で来ても落ちないこと。
    /// 非上場の提出者では Code が null になる事例が既にあった
    #[test]
    fn identifiers_may_be_null() {
        let body = r#"{"DocId": null, "Code": null, "EdinetCode": null,
                       "SubDate": null, "Hldrs": null}"#;
        let d: LargeVolumeShareholdersDoc = serde_json::from_str(body).unwrap();
        assert!(d.doc_id.is_empty() && d.edinet_code.is_empty() && d.sub_date.is_empty());
        assert!(d.hldrs.is_empty());
    }
}
