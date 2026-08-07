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
    pub doc_id: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 提出者の EDINET コード
    #[serde(rename = "EdinetCode")]
    pub edinet_code: String,
    /// 提出者名
    #[serde(rename = "FilerName")]
    pub filer_name: String,
    /// 提出者名（英語）
    #[serde(rename = "FilerNameEn")]
    pub filer_name_en: String,
    /// 書類種別コード
    #[serde(rename = "DocTypeCode")]
    pub doc_type_code: String,
    /// 提出日 (YYYY-MM-DD)
    #[serde(rename = "SubDate")]
    pub sub_date: String,
    /// 提出時刻
    #[serde(rename = "SubTime")]
    pub sub_time: String,
    /// 対象期間開始日
    #[serde(rename = "PerSt")]
    pub per_st: String,
    /// 対象期間終了日
    #[serde(rename = "PerEn")]
    pub per_en: String,
    /// 大株主の一覧
    #[serde(rename = "Hldrs")]
    pub hldrs: Vec<MajorShareholder>,
}

/// 大株主 1 名分の情報
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MajorShareholder {
    /// 順位
    #[serde(rename = "Rank")]
    pub rank: FlexString,
    /// 大株主の氏名・名称
    #[serde(rename = "HldrName")]
    pub hldr_name: String,
    /// 大株主の住所・所在地
    #[serde(rename = "HldrAddr")]
    pub hldr_addr: String,
    /// 所有株式数
    #[serde(rename = "ShsHeld")]
    pub shs_held: FlexString,
    /// 発行済株式総数に対する所有株式数の割合
    #[serde(rename = "ShsRatio")]
    pub shs_ratio: FlexString,
}

/// 政策保有株式（`/edinet/cross-shareholdings`）の書類 1 件
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CrossShareholdingsDoc {
    /// 書類管理番号
    #[serde(rename = "DocId")]
    pub doc_id: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 提出者の EDINET コード
    #[serde(rename = "EdinetCode")]
    pub edinet_code: String,
    /// 提出者名
    #[serde(rename = "FilerName")]
    pub filer_name: String,
    /// 提出者名（英語）
    #[serde(rename = "FilerNameEn")]
    pub filer_name_en: String,
    /// 書類種別コード
    #[serde(rename = "DocTypeCode")]
    pub doc_type_code: String,
    /// 提出日 (YYYY-MM-DD)
    #[serde(rename = "SubDate")]
    pub sub_date: String,
    /// 提出時刻
    #[serde(rename = "SubTime")]
    pub sub_time: String,
    /// 対象期間開始日
    #[serde(rename = "PerSt")]
    pub per_st: String,
    /// 対象期間終了日
    #[serde(rename = "PerEn")]
    pub per_en: String,
    /// 提出会社の保有状況
    #[serde(rename = "Report")]
    pub report: Option<CrossShareholdingHolder>,
    /// 最大保有会社の保有状況
    #[serde(rename = "Largest")]
    pub largest: Option<CrossShareholdingHolder>,
    /// 第 2 位保有会社の保有状況
    #[serde(rename = "SecondLargest")]
    pub second_largest: Option<CrossShareholdingHolder>,
}

/// 政策保有株式の保有主体（提出会社・最大保有会社・第 2 位保有会社共通）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CrossShareholdingHolder {
    /// 保有会社名
    #[serde(rename = "HldrName")]
    pub hldr_name: String,
    /// 保有会社の銘柄コード
    #[serde(rename = "HldrCode")]
    pub hldr_code: FlexString,
    /// 保有会社の EDINET コード
    #[serde(rename = "HldrEdinetCode")]
    pub hldr_edinet_code: FlexString,
    /// 上場株式の銘柄数
    #[serde(rename = "ListedIss")]
    pub listed_iss: FlexString,
    /// 上場株式の貸借対照表計上額
    #[serde(rename = "ListedBookVal")]
    pub listed_book_val: FlexString,
    /// 上場株式の増加銘柄数
    #[serde(rename = "ListedIncIss")]
    pub listed_inc_iss: FlexString,
    /// 上場株式の増加に係る取得価額
    #[serde(rename = "ListedIncAcqCost")]
    pub listed_inc_acq_cost: FlexString,
    /// 上場株式の減少銘柄数
    #[serde(rename = "ListedDecIss")]
    pub listed_dec_iss: FlexString,
    /// 上場株式の減少に係る売却価額
    #[serde(rename = "ListedDecSaleAmt")]
    pub listed_dec_sale_amt: FlexString,
    /// 上場株式の増加理由
    #[serde(rename = "ListedIncRsn")]
    pub listed_inc_rsn: FlexString,
    /// 非上場株式の銘柄数
    #[serde(rename = "NonListedIss")]
    pub non_listed_iss: FlexString,
    /// 非上場株式の貸借対照表計上額
    #[serde(rename = "NonListedBookVal")]
    pub non_listed_book_val: FlexString,
    /// 非上場株式の増加銘柄数
    #[serde(rename = "NonListedIncIss")]
    pub non_listed_inc_iss: FlexString,
    /// 非上場株式の増加に係る取得価額
    #[serde(rename = "NonListedIncAcqCost")]
    pub non_listed_inc_acq_cost: FlexString,
    /// 非上場株式の減少銘柄数
    #[serde(rename = "NonListedDecIss")]
    pub non_listed_dec_iss: FlexString,
    /// 非上場株式の減少に係る売却価額
    #[serde(rename = "NonListedDecSaleAmt")]
    pub non_listed_dec_sale_amt: FlexString,
    /// 非上場株式の増加理由
    #[serde(rename = "NonListedIncRsn")]
    pub non_listed_inc_rsn: FlexString,
    /// 特定投資株式の明細
    #[serde(rename = "Spec")]
    #[serde(default)]
    pub spec: Vec<CrossShareholdingIssue>,
    /// みなし保有株式の明細
    #[serde(rename = "Deem")]
    #[serde(default)]
    pub deem: Vec<CrossShareholdingIssue>,
    /// 特定投資株式の注記
    #[serde(rename = "SpecFn")]
    pub spec_fn: FlexString,
    /// みなし保有株式の注記
    #[serde(rename = "DeemFn")]
    pub deem_fn: FlexString,
}

/// 政策保有株式の銘柄別明細（特定投資株式・みなし保有株式共通）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CrossShareholdingIssue {
    /// 発行者名
    #[serde(rename = "IsrName")]
    pub isr_name: String,
    /// 発行者の銘柄コード
    #[serde(rename = "IsrCode")]
    pub isr_code: FlexString,
    /// 発行者の EDINET コード
    #[serde(rename = "IsrEdinetCode")]
    pub isr_edinet_code: FlexString,
    /// 当期の保有株式数
    #[serde(rename = "CurShs")]
    pub cur_shs: FlexString,
    /// 前期の保有株式数
    #[serde(rename = "PriShs")]
    pub pri_shs: FlexString,
    /// 当期の貸借対照表計上額
    #[serde(rename = "CurBookVal")]
    pub cur_book_val: FlexString,
    /// 前期の貸借対照表計上額
    #[serde(rename = "PriBookVal")]
    pub pri_book_val: FlexString,
    /// 当期の保有株式数（不開示分）
    #[serde(rename = "CurShsNotDisc")]
    pub cur_shs_not_disc: FlexString,
    /// 前期の保有株式数（不開示分）
    #[serde(rename = "PriShsNotDisc")]
    pub pri_shs_not_disc: FlexString,
    /// 当期の貸借対照表計上額（不開示分）
    #[serde(rename = "CurBookValNotDisc")]
    pub cur_book_val_not_disc: FlexString,
    /// 前期の貸借対照表計上額（不開示分）
    #[serde(rename = "PriBookValNotDisc")]
    pub pri_book_val_not_disc: FlexString,
    /// 保有割合
    #[serde(rename = "HoldRat")]
    pub hold_rat: FlexString,
    /// 発行者による相互保有の有無
    #[serde(rename = "IsrHolds")]
    pub isr_holds: FlexString,
    /// 発行者による相互保有の銘柄コード
    #[serde(rename = "IsrHoldsCode")]
    pub isr_holds_code: FlexString,
}

/// 大量保有報告書（`/edinet/large-volume-shareholders`）の書類 1 件
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LargeVolumeShareholdersDoc {
    /// 書類管理番号
    #[serde(rename = "DocId")]
    pub doc_id: String,
    /// 発行者の銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 発行者の EDINET コード
    #[serde(rename = "EdinetCode")]
    pub edinet_code: String,
    /// 発行者名
    #[serde(rename = "IsrName")]
    pub isr_name: String,
    /// 書類種別コード
    #[serde(rename = "DocTypeCode")]
    pub doc_type_code: String,
    /// 提出日 (YYYY-MM-DD)
    #[serde(rename = "SubDate")]
    pub sub_date: String,
    /// 提出時刻
    #[serde(rename = "SubTime")]
    pub sub_time: String,
    /// 報告義務発生事由コード
    #[serde(rename = "LargeHldgTypeCode")]
    pub large_hldg_type_code: String,
    /// 書類タイトル
    #[serde(rename = "DocTitle")]
    pub doc_title: String,
    /// 変更事由
    #[serde(rename = "ChgRsn")]
    pub chg_rsn: FlexString,
    /// 保有株式等の総数
    #[serde(rename = "TotalShsHeld")]
    pub total_shs_held: FlexString,
    /// 発行済株式等の総数
    #[serde(rename = "TotalOutStks")]
    pub total_out_stks: FlexString,
    /// 株式等保有割合
    #[serde(rename = "TotalShsRatio")]
    pub total_shs_ratio: FlexString,
    /// 直前の報告書の株式等保有割合
    #[serde(rename = "TotalShsRatioLast")]
    pub total_shs_ratio_last: FlexString,
    /// 提出者・共同保有者の一覧
    #[serde(rename = "Hldrs")]
    pub hldrs: Vec<LargeVolumeHolder>,
}

/// 大量保有報告書の提出者・共同保有者 1 名分の情報
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LargeVolumeHolder {
    /// 保有者名
    #[serde(rename = "HldrName")]
    pub hldr_name: String,
    /// 保有者名（英語）
    #[serde(rename = "HldrNameEn")]
    pub hldr_name_en: FlexString,
    /// 保有者の EDINET コード
    #[serde(rename = "HldrEdinetCode")]
    pub hldr_edinet_code: FlexString,
    /// 保有者区分コード
    #[serde(rename = "HldrTypeCode")]
    pub hldr_type_code: FlexString,
    /// 保有株式等の数
    #[serde(rename = "ShsHeld")]
    pub shs_held: FlexString,
    /// 自己資金額
    #[serde(rename = "OwnFund")]
    pub own_fund: FlexString,
    /// 借入金額合計
    #[serde(rename = "TotalBrw")]
    pub total_brw: FlexString,
    /// 取得資金合計
    #[serde(rename = "TotalFund")]
    pub total_fund: FlexString,
    /// 最近 60 日間の取得・処分の状況（構造は公式リファレンス参照）
    #[serde(rename = "AcqDisp")]
    #[serde(default)]
    pub acq_disp: Vec<Value>,
    /// 借入先の一覧（構造は公式リファレンス参照）
    #[serde(rename = "BrwList")]
    #[serde(default)]
    pub brw_list: Vec<Value>,
    /// 信用取引等の一覧（構造は公式リファレンス参照）
    #[serde(rename = "CredList")]
    #[serde(default)]
    pub cred_list: Vec<Value>,
}
