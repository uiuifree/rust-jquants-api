use super::flex::FlexString;
use serde::{Deserialize, Serialize};

/// 売買内訳（`/markets/breakdown`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Breakdown {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// ロング売り売買代金
    #[serde(rename = "LongSellVa")]
    pub long_sell_va: f64,
    /// 空売り（信用外）売買代金
    #[serde(rename = "ShrtNoMrgnVa")]
    pub shrt_no_mrgn_va: f64,
    /// 信用新規売り売買代金
    #[serde(rename = "MrgnSellNewVa")]
    pub mrgn_sell_new_va: f64,
    /// 信用返済売り売買代金
    #[serde(rename = "MrgnSellCloseVa")]
    pub mrgn_sell_close_va: f64,
    /// ロング買い売買代金
    #[serde(rename = "LongBuyVa")]
    pub long_buy_va: f64,
    /// 信用新規買い売買代金
    #[serde(rename = "MrgnBuyNewVa")]
    pub mrgn_buy_new_va: f64,
    /// 信用返済買い売買代金
    #[serde(rename = "MrgnBuyCloseVa")]
    pub mrgn_buy_close_va: f64,
    /// ロング売り出来高
    #[serde(rename = "LongSellVo")]
    pub long_sell_vo: f64,
    /// 空売り（信用外）出来高
    #[serde(rename = "ShrtNoMrgnVo")]
    pub shrt_no_mrgn_vo: f64,
    /// 信用新規売り出来高
    #[serde(rename = "MrgnSellNewVo")]
    pub mrgn_sell_new_vo: f64,
    /// 信用返済売り出来高
    #[serde(rename = "MrgnSellCloseVo")]
    pub mrgn_sell_close_vo: f64,
    /// ロング買い出来高
    #[serde(rename = "LongBuyVo")]
    pub long_buy_vo: f64,
    /// 信用新規買い出来高
    #[serde(rename = "MrgnBuyNewVo")]
    pub mrgn_buy_new_vo: f64,
    /// 信用返済買い出来高
    #[serde(rename = "MrgnBuyCloseVo")]
    pub mrgn_buy_close_vo: f64,
}

/// 営業日カレンダー（`/markets/calendar`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Calendar {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 休日区分（0=営業日, 1=休業日）
    #[serde(rename = "HolDiv")]
    pub hol_div: String,
}

/// 日々公表信用取引残高の公表理由。各フィールドは該当有無を示すフラグ
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PubReason {
    /// 信用取引の規制措置に該当
    #[serde(rename = "Restricted")]
    pub restricted: String,
    /// 日々公表銘柄に指定
    #[serde(rename = "DailyPublication")]
    pub daily_publication: String,
    /// 監視銘柄に指定
    #[serde(rename = "Monitoring")]
    pub monitoring: String,
    /// 日本証券金融による制限措置に該当
    #[serde(rename = "RestrictedByJSF")]
    pub restricted_by_jsf: String,
    /// 日本証券金融による注意喚起に該当
    #[serde(rename = "PrecautionByJSF")]
    pub precaution_by_jsf: String,
    /// 不明確または注意銘柄に該当
    #[serde(rename = "UnclearOrSecOnAlert")]
    pub unclear_or_sec_on_alert: String,
}

/// 日々公表信用取引残高（`/markets/margin-alert`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarginAlert {
    /// 公表日 (YYYY-MM-DD)
    #[serde(rename = "PubDate")]
    pub pub_date: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 適用日 (YYYY-MM-DD)
    #[serde(rename = "AppDate")]
    pub app_date: String,
    /// 公表理由（Restricted/DailyPublication/Monitoring/RestrictedByJSF/PrecautionByJSF/UnclearOrSecOnAlert）
    #[serde(rename = "PubReason")]
    pub pub_reason: PubReason,
    /// 空売り残高
    #[serde(rename = "ShrtOut")]
    pub shrt_out: FlexString,
    /// 空売り残高変化
    #[serde(rename = "ShrtOutChg")]
    pub shrt_out_chg: FlexString,
    /// 空売り残高比率
    #[serde(rename = "ShrtOutRatio")]
    pub shrt_out_ratio: FlexString,
    /// ロング残高
    #[serde(rename = "LongOut")]
    pub long_out: FlexString,
    /// ロング残高変化
    #[serde(rename = "LongOutChg")]
    pub long_out_chg: FlexString,
    /// ロング残高比率
    #[serde(rename = "LongOutRatio")]
    pub long_out_ratio: FlexString,
    /// 空売り・ロング比率
    #[serde(rename = "SLRatio")]
    pub sl_ratio: FlexString,
    /// 制度空売り残高
    #[serde(rename = "ShrtNegOut")]
    pub shrt_neg_out: FlexString,
    /// 制度空売り残高変化
    #[serde(rename = "ShrtNegOutChg")]
    pub shrt_neg_out_chg: FlexString,
    /// 一般空売り残高
    #[serde(rename = "ShrtStdOut")]
    pub shrt_std_out: FlexString,
    /// 一般空売り残高変化
    #[serde(rename = "ShrtStdOutChg")]
    pub shrt_std_out_chg: FlexString,
    /// 制度ロング残高
    #[serde(rename = "LongNegOut")]
    pub long_neg_out: FlexString,
    /// 制度ロング残高変化
    #[serde(rename = "LongNegOutChg")]
    pub long_neg_out_chg: FlexString,
    /// 一般ロング残高
    #[serde(rename = "LongStdOut")]
    pub long_std_out: FlexString,
    /// 一般ロング残高変化
    #[serde(rename = "LongStdOutChg")]
    pub long_std_out_chg: FlexString,
    /// 東証信用規制区分
    #[serde(rename = "TSEMrgnRegCls")]
    pub tse_mrgn_reg_cls: String,
}

/// 信用取引週末残高（`/markets/margin-interest`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarginInterest {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 空売り残高株数
    #[serde(rename = "ShrtVol")]
    pub shrt_vol: FlexString,
    /// ロング残高株数
    #[serde(rename = "LongVol")]
    pub long_vol: FlexString,
    /// 制度空売り残高株数
    #[serde(rename = "ShrtNegVol")]
    pub shrt_neg_vol: FlexString,
    /// 制度ロング残高株数
    #[serde(rename = "LongNegVol")]
    pub long_neg_vol: FlexString,
    /// 一般空売り残高株数
    #[serde(rename = "ShrtStdVol")]
    pub shrt_std_vol: FlexString,
    /// 一般ロング残高株数
    #[serde(rename = "LongStdVol")]
    pub long_std_vol: FlexString,
    /// 銘柄種別（信用/貸借）
    #[serde(rename = "IssType")]
    pub iss_type: String,
}

/// 業種別空売り比率（`/markets/short-ratio`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShortRatio {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 33業種コード（例: 0050）
    #[serde(rename = "S33")]
    pub s33: String,
    /// 空売り除き売買代金
    #[serde(rename = "SellExShortVa")]
    pub sell_ex_short_va: FlexString,
    /// 有報空売り売買代金
    #[serde(rename = "ShrtWithResVa")]
    pub shrt_with_res_va: FlexString,
    /// 無報空売り売買代金
    #[serde(rename = "ShrtNoResVa")]
    pub shrt_no_res_va: FlexString,
}

/// 空売り残高報告（`/markets/short-sale-report`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShortSaleReport {
    /// 公表日 (YYYY-MM-DD)
    #[serde(rename = "DiscDate")]
    pub disc_date: String,
    /// 計算日 (YYYY-MM-DD)
    #[serde(rename = "CalcDate")]
    pub calc_date: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 空売り主体名
    #[serde(rename = "SSName")]
    pub ss_name: String,
    /// 空売り主体所在地
    #[serde(rename = "SSAddr")]
    pub ss_addr: String,
    /// 業務執行組合員名
    #[serde(rename = "DICName")]
    pub dic_name: String,
    /// 業務執行組合員所在地
    #[serde(rename = "DICAddr")]
    pub dic_addr: String,
    /// ファンド名
    #[serde(rename = "FundName")]
    pub fund_name: String,
    /// 空売りポジション（発行済株式比率）
    #[serde(rename = "ShrtPosToSO")]
    pub shrt_pos_to_so: FlexString,
    /// 空売りポジション（株数）
    #[serde(rename = "ShrtPosShares")]
    pub shrt_pos_shares: FlexString,
    /// 空売りポジション（単位数）
    #[serde(rename = "ShrtPosUnits")]
    pub shrt_pos_units: FlexString,
    /// 前回報告日 (YYYY-MM-DD)
    #[serde(rename = "PrevRptDate")]
    pub prev_rpt_date: String,
    /// 前回報告比率
    #[serde(rename = "PrevRptRatio")]
    pub prev_rpt_ratio: FlexString,
    /// 備考
    #[serde(rename = "Notes")]
    pub notes: String,
}
