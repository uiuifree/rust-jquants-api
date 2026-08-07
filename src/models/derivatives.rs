use super::flex::FlexString;
use serde::{Deserialize, Serialize};

/// 先物四本値（`/derivatives/bars/daily/futures`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FuturesBar {
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 商品区分（TOPIXF/NK225F等）
    #[serde(rename = "ProdCat")]
    pub prod_cat: String,
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 始値
    #[serde(rename = "O")]
    pub open: FlexString,
    /// 高値
    #[serde(rename = "H")]
    pub high: FlexString,
    /// 安値
    #[serde(rename = "L")]
    pub low: FlexString,
    /// 終値
    #[serde(rename = "C")]
    pub close: FlexString,
    /// 前場始値
    #[serde(rename = "MO")]
    pub morning_open: FlexString,
    /// 前場高値
    #[serde(rename = "MH")]
    pub morning_high: FlexString,
    /// 前場安値
    #[serde(rename = "ML")]
    pub morning_low: FlexString,
    /// 前場終値
    #[serde(rename = "MC")]
    pub morning_close: FlexString,
    /// 夕場始値
    #[serde(rename = "EO")]
    pub evening_open: FlexString,
    /// 夕場高値
    #[serde(rename = "EH")]
    pub evening_high: FlexString,
    /// 夕場安値
    #[serde(rename = "EL")]
    pub evening_low: FlexString,
    /// 夕場終値
    #[serde(rename = "EC")]
    pub evening_close: FlexString,
    /// 後場始値
    #[serde(rename = "AO")]
    pub afternoon_open: FlexString,
    /// 後場高値
    #[serde(rename = "AH")]
    pub afternoon_high: FlexString,
    /// 後場安値
    #[serde(rename = "AL")]
    pub afternoon_low: FlexString,
    /// 後場終値
    #[serde(rename = "AC")]
    pub afternoon_close: FlexString,
    /// 出来高
    #[serde(rename = "Vo")]
    pub volume: FlexString,
    /// 建玉（オープンインタレスト）
    #[serde(rename = "OI")]
    pub open_interest: FlexString,
    /// 売買代金
    #[serde(rename = "Va")]
    pub turnover: FlexString,
    /// 限月 (YYYY-MM)
    #[serde(rename = "CM")]
    pub contract_month: String,
    /// 立会外出来高
    #[serde(rename = "VoOA")]
    pub volume_on_auction: FlexString,
    /// 緊急証拠金徴収区分
    #[serde(rename = "EmMrgnTrgDiv")]
    pub em_mrgn_trg_div: String,
    /// 最終取引日 (YYYY-MM-DD)
    #[serde(rename = "LTD")]
    pub last_trading_date: String,
    /// SQ日 (YYYY-MM-DD)
    #[serde(rename = "SQD")]
    pub sq_date: String,
    /// 清算価格
    #[serde(rename = "Settle")]
    pub settle: FlexString,
    /// 中心限月フラグ（1=中心限月）
    #[serde(rename = "CCMFlag")]
    pub ccm_flag: String,
}

/// オプション四本値（`/derivatives/bars/daily/options`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OptionsBar {
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 商品区分
    #[serde(rename = "ProdCat")]
    pub prod_cat: String,
    /// 原資産の有価証券コード
    #[serde(rename = "UndSSO")]
    pub und_sso: String,
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 始値
    #[serde(rename = "O")]
    pub open: FlexString,
    /// 高値
    #[serde(rename = "H")]
    pub high: FlexString,
    /// 安値
    #[serde(rename = "L")]
    pub low: FlexString,
    /// 終値
    #[serde(rename = "C")]
    pub close: FlexString,
    /// 前場始値
    #[serde(rename = "MO")]
    pub morning_open: FlexString,
    /// 前場高値
    #[serde(rename = "MH")]
    pub morning_high: FlexString,
    /// 前場安値
    #[serde(rename = "ML")]
    pub morning_low: FlexString,
    /// 前場終値
    #[serde(rename = "MC")]
    pub morning_close: FlexString,
    /// 夕場始値
    #[serde(rename = "EO")]
    pub evening_open: FlexString,
    /// 夕場高値
    #[serde(rename = "EH")]
    pub evening_high: FlexString,
    /// 夕場安値
    #[serde(rename = "EL")]
    pub evening_low: FlexString,
    /// 夕場終値
    #[serde(rename = "EC")]
    pub evening_close: FlexString,
    /// 後場始値
    #[serde(rename = "AO")]
    pub afternoon_open: FlexString,
    /// 後場高値
    #[serde(rename = "AH")]
    pub afternoon_high: FlexString,
    /// 後場安値
    #[serde(rename = "AL")]
    pub afternoon_low: FlexString,
    /// 後場終値
    #[serde(rename = "AC")]
    pub afternoon_close: FlexString,
    /// 出来高
    #[serde(rename = "Vo")]
    pub volume: FlexString,
    /// 建玉（オープンインタレスト）
    #[serde(rename = "OI")]
    pub open_interest: FlexString,
    /// 売買代金
    #[serde(rename = "Va")]
    pub turnover: FlexString,
    /// 限月 (YYYY-MM)
    #[serde(rename = "CM")]
    pub contract_month: String,
    /// 行使価格
    #[serde(rename = "Strike")]
    pub strike: FlexString,
    /// 立会外出来高
    #[serde(rename = "VoOA")]
    pub volume_on_auction: FlexString,
    /// 緊急証拠金徴収区分
    #[serde(rename = "EmMrgnTrgDiv")]
    pub em_mrgn_trg_div: String,
    /// プット・コール区分（P/C）
    #[serde(rename = "PCDiv")]
    pub pc_div: String,
    /// 最終取引日 (YYYY-MM-DD)
    #[serde(rename = "LTD")]
    pub last_trading_date: String,
    /// SQ日 (YYYY-MM-DD)
    #[serde(rename = "SQD")]
    pub sq_date: String,
    /// 清算価格
    #[serde(rename = "Settle")]
    pub settle: FlexString,
    /// 理論価格
    #[serde(rename = "Theo")]
    pub theo: FlexString,
    /// 基準ボラティリティ
    #[serde(rename = "BaseVol")]
    pub base_vol: FlexString,
    /// 原資産価格
    #[serde(rename = "UnderPx")]
    pub under_px: FlexString,
    /// インプライドボラティリティ
    #[serde(rename = "IV")]
    pub iv: FlexString,
    /// 金利（リスクフリーレート）
    #[serde(rename = "IR")]
    pub ir: FlexString,
    /// 中心限月フラグ（1=中心限月）
    #[serde(rename = "CCMFlag")]
    pub ccm_flag: String,
}

/// 日経225オプション四本値（`/derivatives/bars/daily/options/225`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Options225Bar {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 始値
    #[serde(rename = "O")]
    pub open: FlexString,
    /// 高値
    #[serde(rename = "H")]
    pub high: FlexString,
    /// 安値
    #[serde(rename = "L")]
    pub low: FlexString,
    /// 終値
    #[serde(rename = "C")]
    pub close: FlexString,
    /// 夕場始値
    #[serde(rename = "EO")]
    pub evening_open: FlexString,
    /// 夕場高値
    #[serde(rename = "EH")]
    pub evening_high: FlexString,
    /// 夕場安値
    #[serde(rename = "EL")]
    pub evening_low: FlexString,
    /// 夕場終値
    #[serde(rename = "EC")]
    pub evening_close: FlexString,
    /// 後場始値
    #[serde(rename = "AO")]
    pub afternoon_open: FlexString,
    /// 後場高値
    #[serde(rename = "AH")]
    pub afternoon_high: FlexString,
    /// 後場安値
    #[serde(rename = "AL")]
    pub afternoon_low: FlexString,
    /// 後場終値
    #[serde(rename = "AC")]
    pub afternoon_close: FlexString,
    /// 出来高
    #[serde(rename = "Vo")]
    pub volume: FlexString,
    /// 建玉（オープンインタレスト）
    #[serde(rename = "OI")]
    pub open_interest: FlexString,
    /// 売買代金
    #[serde(rename = "Va")]
    pub turnover: FlexString,
    /// 限月 (YYYY-MM)
    #[serde(rename = "CM")]
    pub contract_month: String,
    /// 行使価格
    #[serde(rename = "Strike")]
    pub strike: FlexString,
    /// 立会外出来高
    #[serde(rename = "VoOA")]
    pub volume_on_auction: FlexString,
    /// 緊急証拠金徴収区分
    #[serde(rename = "EmMrgnTrgDiv")]
    pub em_mrgn_trg_div: String,
    /// プット・コール区分（P/C）
    #[serde(rename = "PCDiv")]
    pub pc_div: String,
    /// 最終取引日 (YYYY-MM-DD)
    #[serde(rename = "LTD")]
    pub last_trading_date: String,
    /// SQ日 (YYYY-MM-DD)
    #[serde(rename = "SQD")]
    pub sq_date: String,
    /// 清算価格
    #[serde(rename = "Settle")]
    pub settle: FlexString,
    /// 理論価格
    #[serde(rename = "Theo")]
    pub theo: FlexString,
    /// 基準ボラティリティ
    #[serde(rename = "BaseVol")]
    pub base_vol: FlexString,
    /// 原資産価格
    #[serde(rename = "UnderPx")]
    pub under_px: FlexString,
    /// インプライドボラティリティ
    #[serde(rename = "IV")]
    pub iv: FlexString,
    /// 金利（リスクフリーレート）
    #[serde(rename = "IR")]
    pub ir: FlexString,
}
