use super::flex::FlexString;
use serde::{Deserialize, Serialize};

/// 上場銘柄マスタ（`/equities/master`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StockMaster {
    /// 基準日 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 銘柄名（日本語）
    #[serde(rename = "CoName")]
    pub co_name: String,
    /// 銘柄名（英語）
    #[serde(rename = "CoNameEn")]
    pub co_name_en: String,
    /// 17業種コード
    #[serde(rename = "S17")]
    pub sector17_code: String,
    /// 17業種名
    #[serde(rename = "S17Nm")]
    pub sector17_code_name: String,
    /// 33業種コード
    #[serde(rename = "S33")]
    pub sector33_code: String,
    /// 33業種名
    #[serde(rename = "S33Nm")]
    pub sector33_code_name: String,
    /// 規模区分（TOPIX Large70等）
    #[serde(rename = "ScaleCat")]
    pub scale_category: String,
    /// 市場区分コード
    #[serde(rename = "Mkt")]
    pub market_code: String,
    /// 市場区分名（プライム等）
    #[serde(rename = "MktNm")]
    pub market_code_name: String,
    /// 信用区分コード
    #[serde(rename = "Mrgn")]
    pub margin_code: String,
    /// 信用区分名（信用/貸借）
    #[serde(rename = "MrgnNm")]
    pub margin_code_name: String,
}

/// 株価日足（`/equities/bars/daily`）。前後場・調整値を含む
#[derive(Debug, Deserialize, Serialize)]
pub struct DailyBar {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 始値
    #[serde(rename = "O")]
    pub open: Option<f64>,
    /// 高値
    #[serde(rename = "H")]
    pub high: Option<f64>,
    /// 安値
    #[serde(rename = "L")]
    pub low: Option<f64>,
    /// 終値
    #[serde(rename = "C")]
    pub close: Option<f64>,
    /// ストップ高フラグ
    #[serde(rename = "UL")]
    pub upper_limit: Option<String>,
    /// ストップ安フラグ
    #[serde(rename = "LL")]
    pub lower_limit: Option<String>,
    /// 出来高（株）
    #[serde(rename = "Vo")]
    pub volume: Option<f64>,
    /// 売買代金（円）
    #[serde(rename = "Va")]
    pub turnover: Option<f64>,
    /// 調整係数（分割・併合等）
    #[serde(rename = "AdjFactor")]
    pub adj_factor: Option<f64>,
    /// 調整後始値
    #[serde(rename = "AdjO")]
    pub adj_open: Option<f64>,
    /// 調整後高値
    #[serde(rename = "AdjH")]
    pub adj_high: Option<f64>,
    /// 調整後安値
    #[serde(rename = "AdjL")]
    pub adj_low: Option<f64>,
    /// 調整後終値
    #[serde(rename = "AdjC")]
    pub adj_close: Option<f64>,
    /// 調整後出来高
    #[serde(rename = "AdjVo")]
    pub adj_volume: Option<f64>,
    /// 前場始値
    #[serde(rename = "MO")]
    pub morning_open: Option<f64>,
    /// 前場高値
    #[serde(rename = "MH")]
    pub morning_high: Option<f64>,
    /// 前場安値
    #[serde(rename = "ML")]
    pub morning_low: Option<f64>,
    /// 前場終値
    #[serde(rename = "MC")]
    pub morning_close: Option<f64>,
    /// 前場ストップ高フラグ
    #[serde(rename = "MUL")]
    pub morning_upper_limit: Option<String>,
    /// 前場ストップ安フラグ
    #[serde(rename = "MLL")]
    pub morning_lower_limit: Option<String>,
    /// 前場出来高
    #[serde(rename = "MVo")]
    pub morning_volume: Option<f64>,
    /// 前場売買代金
    #[serde(rename = "MVa")]
    pub morning_turnover: Option<f64>,
    /// 前場調整後始値
    #[serde(rename = "MAdjO")]
    pub morning_adj_open: Option<f64>,
    /// 前場調整後高値
    #[serde(rename = "MAdjH")]
    pub morning_adj_high: Option<f64>,
    /// 前場調整後安値
    #[serde(rename = "MAdjL")]
    pub morning_adj_low: Option<f64>,
    /// 前場調整後終値
    #[serde(rename = "MAdjC")]
    pub morning_adj_close: Option<f64>,
    /// 前場調整後出来高
    #[serde(rename = "MAdjVo")]
    pub morning_adj_volume: Option<f64>,
    /// 後場始値
    #[serde(rename = "AO")]
    pub afternoon_open: Option<f64>,
    /// 後場高値
    #[serde(rename = "AH")]
    pub afternoon_high: Option<f64>,
    /// 後場安値
    #[serde(rename = "AL")]
    pub afternoon_low: Option<f64>,
    /// 後場終値
    #[serde(rename = "AC")]
    pub afternoon_close: Option<f64>,
    /// 後場ストップ高フラグ
    #[serde(rename = "AUL")]
    pub afternoon_upper_limit: Option<String>,
    /// 後場ストップ安フラグ
    #[serde(rename = "ALL")]
    pub afternoon_lower_limit: Option<String>,
    /// 後場出来高
    #[serde(rename = "AVo")]
    pub afternoon_volume: Option<f64>,
    /// 後場売買代金
    #[serde(rename = "AVa")]
    pub afternoon_turnover: Option<f64>,
    /// 後場調整後始値
    #[serde(rename = "AAdjO")]
    pub afternoon_adj_open: Option<f64>,
    /// 後場調整後高値
    #[serde(rename = "AAdjH")]
    pub afternoon_adj_high: Option<f64>,
    /// 後場調整後安値
    #[serde(rename = "AAdjL")]
    pub afternoon_adj_low: Option<f64>,
    /// 後場調整後終値
    #[serde(rename = "AAdjC")]
    pub afternoon_adj_close: Option<f64>,
    /// 後場調整後出来高
    #[serde(rename = "AAdjVo")]
    pub afternoon_adj_volume: Option<f64>,
}

/// 前場終値時点の株価（`/equities/bars/daily/am`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AmBar {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 前場始値
    #[serde(rename = "MO")]
    pub morning_open: Option<f64>,
    /// 前場高値
    #[serde(rename = "MH")]
    pub morning_high: Option<f64>,
    /// 前場安値
    #[serde(rename = "ML")]
    pub morning_low: Option<f64>,
    /// 前場終値
    #[serde(rename = "MC")]
    pub morning_close: Option<f64>,
    /// 前場出来高（株）
    #[serde(rename = "MVo")]
    pub morning_volume: Option<f64>,
    /// 前場売買代金（円）
    #[serde(rename = "MVa")]
    pub morning_turnover: Option<f64>,
}

/// 株価分足（`/equities/bars/minute`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MinuteBar {
    /// 日付 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 時刻 (HH:MM:SS)
    #[serde(rename = "Time")]
    pub time: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 始値
    #[serde(rename = "O")]
    pub open: f64,
    /// 高値
    #[serde(rename = "H")]
    pub high: f64,
    /// 安値
    #[serde(rename = "L")]
    pub low: f64,
    /// 終値
    #[serde(rename = "C")]
    pub close: f64,
    /// 出来高（株）
    #[serde(rename = "Vo")]
    pub volume: f64,
    /// 売買代金（円）
    #[serde(rename = "Va")]
    pub turnover: f64,
}

/// 決算発表予定（`/equities/earnings-calendar`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EarningsCalendar {
    /// 決算発表予定日 (YYYY-MM-DD)
    #[serde(rename = "Date")]
    pub date: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 銘柄名
    #[serde(rename = "CoName")]
    pub co_name: String,
    /// 決算期（会計年度）
    #[serde(rename = "FY")]
    pub fy: String,
    /// セクター名
    #[serde(rename = "SectorNm")]
    pub sector_nm: String,
    /// 決算四半期区分（1Q/2Q/3Q/通期）
    #[serde(rename = "FQ")]
    pub fq: String,
    /// 市場区分
    #[serde(rename = "Section")]
    pub section: String,
}

/// 投資部門別売買状況（`/equities/investor-types`）
#[derive(Debug, Deserialize, Serialize)]
pub struct InvestorType {
    /// 公表日 (YYYY-MM-DD)
    #[serde(rename = "PubDate")]
    pub pub_date: String,
    /// 集計開始日 (YYYY-MM-DD)
    #[serde(rename = "StDate")]
    pub st_date: String,
    /// 集計終了日 (YYYY-MM-DD)
    #[serde(rename = "EnDate")]
    pub en_date: String,
    /// 市場区分（TSEPrime等）
    #[serde(rename = "Section")]
    pub section: String,
    /// 自己売り
    #[serde(rename = "PropSell")]
    pub prop_sell: FlexString,
    /// 自己買い
    #[serde(rename = "PropBuy")]
    pub prop_buy: FlexString,
    /// 自己合計
    #[serde(rename = "PropTot")]
    pub prop_tot: FlexString,
    /// 自己差引
    #[serde(rename = "PropBal")]
    pub prop_bal: FlexString,
    /// 委託売り
    #[serde(rename = "BrkSell")]
    pub brk_sell: FlexString,
    /// 委託買い
    #[serde(rename = "BrkBuy")]
    pub brk_buy: FlexString,
    /// 委託合計
    #[serde(rename = "BrkTot")]
    pub brk_tot: FlexString,
    /// 委託差引
    #[serde(rename = "BrkBal")]
    pub brk_bal: FlexString,
    /// 総売り
    #[serde(rename = "TotSell")]
    pub tot_sell: FlexString,
    /// 総買い
    #[serde(rename = "TotBuy")]
    pub tot_buy: FlexString,
    /// 総合計
    #[serde(rename = "TotTot")]
    pub tot_tot: FlexString,
    /// 総差引
    #[serde(rename = "TotBal")]
    pub tot_bal: FlexString,
    /// 個人売り
    #[serde(rename = "IndSell")]
    pub ind_sell: FlexString,
    /// 個人買い
    #[serde(rename = "IndBuy")]
    pub ind_buy: FlexString,
    /// 個人合計
    #[serde(rename = "IndTot")]
    pub ind_tot: FlexString,
    /// 個人差引
    #[serde(rename = "IndBal")]
    pub ind_bal: FlexString,
    /// 外国人売り
    #[serde(rename = "FrgnSell")]
    pub frgn_sell: FlexString,
    /// 外国人買い
    #[serde(rename = "FrgnBuy")]
    pub frgn_buy: FlexString,
    /// 外国人合計
    #[serde(rename = "FrgnTot")]
    pub frgn_tot: FlexString,
    /// 外国人差引
    #[serde(rename = "FrgnBal")]
    pub frgn_bal: FlexString,
    /// 証券会社売り
    #[serde(rename = "SecCoSell")]
    pub sec_co_sell: FlexString,
    /// 証券会社買い
    #[serde(rename = "SecCoBuy")]
    pub sec_co_buy: FlexString,
    /// 証券会社合計
    #[serde(rename = "SecCoTot")]
    pub sec_co_tot: FlexString,
    /// 証券会社差引
    #[serde(rename = "SecCoBal")]
    pub sec_co_bal: FlexString,
    /// 投資信託売り
    #[serde(rename = "InvTrSell")]
    pub inv_tr_sell: FlexString,
    /// 投資信託買い
    #[serde(rename = "InvTrBuy")]
    pub inv_tr_buy: FlexString,
    /// 投資信託合計
    #[serde(rename = "InvTrTot")]
    pub inv_tr_tot: FlexString,
    /// 投資信託差引
    #[serde(rename = "InvTrBal")]
    pub inv_tr_bal: FlexString,
    /// 事業法人売り
    #[serde(rename = "BusCoSell")]
    pub bus_co_sell: FlexString,
    /// 事業法人買い
    #[serde(rename = "BusCoBuy")]
    pub bus_co_buy: FlexString,
    /// 事業法人合計
    #[serde(rename = "BusCoTot")]
    pub bus_co_tot: FlexString,
    /// 事業法人差引
    #[serde(rename = "BusCoBal")]
    pub bus_co_bal: FlexString,
    /// その他法人売り
    #[serde(rename = "OthCoSell")]
    pub oth_co_sell: FlexString,
    /// その他法人買い
    #[serde(rename = "OthCoBuy")]
    pub oth_co_buy: FlexString,
    /// その他法人合計
    #[serde(rename = "OthCoTot")]
    pub oth_co_tot: FlexString,
    /// その他法人差引
    #[serde(rename = "OthCoBal")]
    pub oth_co_bal: FlexString,
    /// 生損保売り
    #[serde(rename = "InsCoSell")]
    pub ins_co_sell: FlexString,
    /// 生損保買い
    #[serde(rename = "InsCoBuy")]
    pub ins_co_buy: FlexString,
    /// 生損保合計
    #[serde(rename = "InsCoTot")]
    pub ins_co_tot: FlexString,
    /// 生損保差引
    #[serde(rename = "InsCoBal")]
    pub ins_co_bal: FlexString,
    /// 都銀・地銀等売り
    #[serde(rename = "BankSell")]
    pub bank_sell: FlexString,
    /// 都銀・地銀等買い
    #[serde(rename = "BankBuy")]
    pub bank_buy: FlexString,
    /// 都銀・地銀等合計
    #[serde(rename = "BankTot")]
    pub bank_tot: FlexString,
    /// 都銀・地銀等差引
    #[serde(rename = "BankBal")]
    pub bank_bal: FlexString,
    /// 信託銀行売り
    #[serde(rename = "TrstBnkSell")]
    pub trst_bnk_sell: FlexString,
    /// 信託銀行買い
    #[serde(rename = "TrstBnkBuy")]
    pub trst_bnk_buy: FlexString,
    /// 信託銀行合計
    #[serde(rename = "TrstBnkTot")]
    pub trst_bnk_tot: FlexString,
    /// 信託銀行差引
    #[serde(rename = "TrstBnkBal")]
    pub trst_bnk_bal: FlexString,
    /// その他金融機関売り
    #[serde(rename = "OthFinSell")]
    pub oth_fin_sell: FlexString,
    /// その他金融機関買い
    #[serde(rename = "OthFinBuy")]
    pub oth_fin_buy: FlexString,
    /// その他金融機関合計
    #[serde(rename = "OthFinTot")]
    pub oth_fin_tot: FlexString,
    /// その他金融機関差引
    #[serde(rename = "OthFinBal")]
    pub oth_fin_bal: FlexString,
}
