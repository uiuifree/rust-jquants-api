use super::flex::FlexString;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 財務諸表サマリー（`/fins/summary`）
#[derive(Debug, Deserialize, Serialize)]
pub struct FinsSummary {
    /// 開示日 (YYYY-MM-DD)
    #[serde(rename = "DiscDate")]
    pub disc_date: String,
    /// 開示時刻 (HH:MM:SS)
    #[serde(rename = "DiscTime")]
    pub disc_time: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 開示番号
    #[serde(rename = "DiscNo")]
    pub disc_no: String,
    /// 書類種別
    #[serde(rename = "DocType")]
    pub doc_type: String,
    /// 当期種別（通期/四半期）
    #[serde(rename = "CurPerType")]
    pub cur_per_type: String,
    /// 当期開始日 (YYYY-MM-DD)
    #[serde(rename = "CurPerSt")]
    pub cur_per_st: String,
    /// 当期終了日 (YYYY-MM-DD)
    #[serde(rename = "CurPerEn")]
    pub cur_per_en: String,
    /// 当会計年度開始日 (YYYY-MM-DD)
    #[serde(rename = "CurFYSt")]
    pub cur_fy_st: String,
    /// 当会計年度終了日 (YYYY-MM-DD)
    #[serde(rename = "CurFYEn")]
    pub cur_fy_en: String,
    /// 翌会計年度開始日 (YYYY-MM-DD)
    #[serde(rename = "NxtFYSt")]
    pub nxt_fy_st: String,
    /// 翌会計年度終了日 (YYYY-MM-DD)
    #[serde(rename = "NxtFYEn")]
    pub nxt_fy_en: String,
    /// 売上高（実績）
    #[serde(rename = "Sales")]
    pub sales: FlexString,
    /// 営業利益（実績）
    #[serde(rename = "OP")]
    pub op: FlexString,
    /// 経常利益（実績）
    #[serde(rename = "OdP")]
    pub od_p: FlexString,
    /// 純利益（実績）
    #[serde(rename = "NP")]
    pub np: FlexString,
    /// 1株当たり純利益（実績）
    #[serde(rename = "EPS")]
    pub eps: FlexString,
    /// 希薄化後EPS（実績）
    #[serde(rename = "DEPS")]
    pub deps: FlexString,
    /// 総資産（実績）
    #[serde(rename = "TA")]
    pub ta: FlexString,
    /// 純資産（実績）
    #[serde(rename = "Eq")]
    pub eq: FlexString,
    /// 自己資本比率（実績）
    #[serde(rename = "EqAR")]
    pub eq_ar: FlexString,
    /// 1株当たり純資産（実績）
    #[serde(rename = "BPS")]
    pub bps: FlexString,
    /// 営業CF（実績）
    #[serde(rename = "CFO")]
    pub cfo: FlexString,
    /// 投資CF（実績）
    #[serde(rename = "CFI")]
    pub cfi: FlexString,
    /// 財務CF（実績）
    #[serde(rename = "CFF")]
    pub cff: FlexString,
    /// 期末現金・現金同等物（実績）
    #[serde(rename = "CashEq")]
    pub cash_eq: FlexString,
    /// 第1四半期配当（実績）
    #[serde(rename = "Div1Q")]
    pub div_1q: FlexString,
    /// 第2四半期配当（実績）
    #[serde(rename = "Div2Q")]
    pub div_2q: FlexString,
    /// 第3四半期配当（実績）
    #[serde(rename = "Div3Q")]
    pub div_3q: FlexString,
    /// 期末配当（実績）
    #[serde(rename = "DivFY")]
    pub div_fy: FlexString,
    /// 年間配当（実績）
    #[serde(rename = "DivAnn")]
    pub div_ann: FlexString,
    /// 配当単位（実績）
    #[serde(rename = "DivUnit")]
    pub div_unit: FlexString,
    /// 年間配当総額（実績）
    #[serde(rename = "DivTotalAnn")]
    pub div_total_ann: FlexString,
    /// 配当性向（実績）
    #[serde(rename = "PayoutRatioAnn")]
    pub payout_ratio_ann: FlexString,
    /// 第1四半期配当（当期予想）
    #[serde(rename = "FDiv1Q")]
    pub f_div_1q: FlexString,
    /// 第2四半期配当（当期予想）
    #[serde(rename = "FDiv2Q")]
    pub f_div_2q: FlexString,
    /// 第3四半期配当（当期予想）
    #[serde(rename = "FDiv3Q")]
    pub f_div_3q: FlexString,
    /// 期末配当（当期予想）
    #[serde(rename = "FDivFY")]
    pub f_div_fy: FlexString,
    /// 年間配当（当期予想）
    #[serde(rename = "FDivAnn")]
    pub f_div_ann: FlexString,
    /// 配当単位（当期予想）
    #[serde(rename = "FDivUnit")]
    pub f_div_unit: FlexString,
    /// 年間配当総額（当期予想）
    #[serde(rename = "FDivTotalAnn")]
    pub f_div_total_ann: FlexString,
    /// 配当性向（当期予想）
    #[serde(rename = "FPayoutRatioAnn")]
    pub f_payout_ratio_ann: FlexString,
    /// 第1四半期配当（翌期予想）
    #[serde(rename = "NxFDiv1Q")]
    pub nx_f_div_1q: FlexString,
    /// 第2四半期配当（翌期予想）
    #[serde(rename = "NxFDiv2Q")]
    pub nx_f_div_2q: FlexString,
    /// 第3四半期配当（翌期予想）
    #[serde(rename = "NxFDiv3Q")]
    pub nx_f_div_3q: FlexString,
    /// 期末配当（翌期予想）
    #[serde(rename = "NxFDivFY")]
    pub nx_f_div_fy: FlexString,
    /// 年間配当（翌期予想）
    #[serde(rename = "NxFDivAnn")]
    pub nx_f_div_ann: FlexString,
    /// 配当単位（翌期予想）
    #[serde(rename = "NxFDivUnit")]
    pub nx_f_div_unit: FlexString,
    /// 配当性向（翌期予想）
    #[serde(rename = "NxFPayoutRatioAnn")]
    pub nx_f_payout_ratio_ann: FlexString,
    /// 売上高第2四半期予想（当期）
    #[serde(rename = "FSales2Q")]
    pub f_sales_2q: FlexString,
    /// 営業利益第2四半期予想（当期）
    #[serde(rename = "FOP2Q")]
    pub f_op_2q: FlexString,
    /// 経常利益第2四半期予想（当期）
    #[serde(rename = "FOdP2Q")]
    pub f_od_p_2q: FlexString,
    /// 純利益第2四半期予想（当期）
    #[serde(rename = "FNP2Q")]
    pub f_np_2q: FlexString,
    /// EPS第2四半期予想（当期）
    #[serde(rename = "FEPS2Q")]
    pub f_eps_2q: FlexString,
    /// 売上高第2四半期予想（翌期）
    #[serde(rename = "NxFSales2Q")]
    pub nx_f_sales_2q: FlexString,
    /// 営業利益第2四半期予想（翌期）
    #[serde(rename = "NxFOP2Q")]
    pub nx_f_op_2q: FlexString,
    /// 経常利益第2四半期予想（翌期）
    #[serde(rename = "NxFOdP2Q")]
    pub nx_f_od_p_2q: FlexString,
    /// 純利益第2四半期予想（翌期）
    #[serde(rename = "NxFNp2Q")]
    pub nx_f_np_2q: FlexString,
    /// EPS第2四半期予想（翌期）
    #[serde(rename = "NxFEPS2Q")]
    pub nx_f_eps_2q: FlexString,
    /// 売上高通期予想（当期）
    #[serde(rename = "FSales")]
    pub f_sales: FlexString,
    /// 営業利益通期予想（当期）
    #[serde(rename = "FOP")]
    pub f_op: FlexString,
    /// 経常利益通期予想（当期）
    #[serde(rename = "FOdP")]
    pub f_od_p: FlexString,
    /// 純利益通期予想（当期）
    #[serde(rename = "FNP")]
    pub f_np: FlexString,
    /// EPS通期予想（当期）
    #[serde(rename = "FEPS")]
    pub f_eps: FlexString,
    /// 売上高通期予想（翌期）
    #[serde(rename = "NxFSales")]
    pub nx_f_sales: FlexString,
    /// 営業利益通期予想（翌期）
    #[serde(rename = "NxFOP")]
    pub nx_f_op: FlexString,
    /// 経常利益通期予想（翌期）
    #[serde(rename = "NxFOdP")]
    pub nx_f_od_p: FlexString,
    /// 純利益通期予想（翌期）
    #[serde(rename = "NxFNp")]
    pub nx_f_np: FlexString,
    /// EPS通期予想（翌期）
    #[serde(rename = "NxFEPS")]
    pub nx_f_eps: FlexString,
    /// 実質的な支配の変更フラグ
    #[serde(rename = "MatChgSub")]
    pub mat_chg_sub: String,
    /// 連結範囲重要変更フラグ
    #[serde(rename = "SigChgInC")]
    pub sig_chg_in_c: String,
    /// 会計基準変更フラグ
    #[serde(rename = "ChgByASRev")]
    pub chg_by_as_rev: String,
    /// 会計基準変更以外フラグ
    #[serde(rename = "ChgNoASRev")]
    pub chg_no_as_rev: String,
    /// 会計上の見積変更フラグ
    #[serde(rename = "ChgAcEst")]
    pub chg_ac_est: String,
    /// 遡及修正フラグ
    #[serde(rename = "RetroRst")]
    pub retro_rst: String,
    /// 期末発行済株式数
    #[serde(rename = "ShOutFY")]
    pub sh_out_fy: FlexString,
    /// 期末自己株式数
    #[serde(rename = "TrShFY")]
    pub tr_sh_fy: FlexString,
    /// 期中平均株式数
    #[serde(rename = "AvgSh")]
    pub avg_sh: FlexString,
    /// 連結売上高（実績）
    #[serde(rename = "NCSales")]
    pub nc_sales: FlexString,
    /// 連結営業利益（実績）
    #[serde(rename = "NCOP")]
    pub nc_op: FlexString,
    /// 連結経常利益（実績）
    #[serde(rename = "NCOdP")]
    pub nc_od_p: FlexString,
    /// 連結純利益（実績）
    #[serde(rename = "NCNP")]
    pub nc_np: FlexString,
    /// 連結EPS（実績）
    #[serde(rename = "NCEPS")]
    pub nc_eps: FlexString,
    /// 連結総資産（実績）
    #[serde(rename = "NCTA")]
    pub nc_ta: FlexString,
    /// 連結純資産（実績）
    #[serde(rename = "NCEq")]
    pub nc_eq: FlexString,
    /// 連結自己資本比率（実績）
    #[serde(rename = "NCEqAR")]
    pub nc_eq_ar: FlexString,
    /// 連結BPS（実績）
    #[serde(rename = "NCBPS")]
    pub nc_bps: FlexString,
    /// 連結売上高第2四半期予想（当期）
    #[serde(rename = "FNCSales2Q")]
    pub f_nc_sales_2q: FlexString,
    /// 連結営業利益第2四半期予想（当期）
    #[serde(rename = "FNCOP2Q")]
    pub f_nc_op_2q: FlexString,
    /// 連結経常利益第2四半期予想（当期）
    #[serde(rename = "FNCOdP2Q")]
    pub f_nc_od_p_2q: FlexString,
    /// 連結純利益第2四半期予想（当期）
    #[serde(rename = "FNCNP2Q")]
    pub f_nc_np_2q: FlexString,
    /// 連結EPS第2四半期予想（当期）
    #[serde(rename = "FNCEPS2Q")]
    pub f_nc_eps_2q: FlexString,
    /// 連結売上高第2四半期予想（翌期）
    #[serde(rename = "NxFNCSales2Q")]
    pub nx_f_nc_sales_2q: FlexString,
    /// 連結営業利益第2四半期予想（翌期）
    #[serde(rename = "NxFNCOP2Q")]
    pub nx_f_nc_op_2q: FlexString,
    /// 連結経常利益第2四半期予想（翌期）
    #[serde(rename = "NxFNCOdP2Q")]
    pub nx_f_nc_od_p_2q: FlexString,
    /// 連結純利益第2四半期予想（翌期）
    #[serde(rename = "NxFNCNP2Q")]
    pub nx_f_nc_np_2q: FlexString,
    /// 連結EPS第2四半期予想（翌期）
    #[serde(rename = "NxFNCEPS2Q")]
    pub nx_f_nc_eps_2q: FlexString,
    /// 連結売上高通期予想（当期）
    #[serde(rename = "FNCSales")]
    pub f_nc_sales: FlexString,
    /// 連結営業利益通期予想（当期）
    #[serde(rename = "FNCOP")]
    pub f_nc_op: FlexString,
    /// 連結経常利益通期予想（当期）
    #[serde(rename = "FNCOdP")]
    pub f_nc_od_p: FlexString,
    /// 連結純利益通期予想（当期）
    #[serde(rename = "FNCNP")]
    pub f_nc_np: FlexString,
    /// 連結EPS通期予想（当期）
    #[serde(rename = "FNCEPS")]
    pub f_nc_eps: FlexString,
    /// 連結売上高通期予想（翌期）
    #[serde(rename = "NxFNCSales")]
    pub nx_f_nc_sales: FlexString,
    /// 連結営業利益通期予想（翌期）
    #[serde(rename = "NxFNCOP")]
    pub nx_f_nc_op: FlexString,
    /// 連結経常利益通期予想（翌期）
    #[serde(rename = "NxFNCOdP")]
    pub nx_f_nc_od_p: FlexString,
    /// 連結純利益通期予想（翌期）
    #[serde(rename = "NxFNCNP")]
    pub nx_f_nc_np: FlexString,
    /// 連結EPS通期予想（翌期）
    #[serde(rename = "NxFNCEPS")]
    pub nx_f_nc_eps: FlexString,
}

/// 財務諸表詳細（`/fins/details`）。`fs` は勘定科目の可変な集合のため JSON のまま保持する
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FinsDetails {
    /// 開示日 (YYYY-MM-DD)
    #[serde(rename = "DiscDate")]
    pub disc_date: String,
    /// 開示時刻 (HH:MM:SS)
    #[serde(rename = "DiscTime")]
    pub disc_time: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 開示番号
    #[serde(rename = "DiscNo")]
    pub disc_no: String,
    /// 書類種別（決算短信等）
    #[serde(rename = "DocType")]
    pub doc_type: String,
    /// 財務諸表データ（BS/PL/CF等。キーは銘柄・決算種別により異なる。--output json で完全表示）
    #[serde(rename = "FS")]
    pub fs: Value,
}

/// 決算発表予定日（`/fins/earnings-date`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EarningsDate {
    /// 公表日 (YYYY-MM-DD)
    #[serde(rename = "PubDate")]
    pub pub_date: String,
    /// 決算発表予定日 (YYYY-MM-DD)。未定の場合は空文字列
    #[serde(rename = "SchDate")]
    pub sch_date: String,
    /// 決算区分（1Q / 2Q / 3Q / FY）
    #[serde(rename = "FQName")]
    pub fq_name: String,
    /// 決算期末 (MMDD)
    #[serde(rename = "FYE")]
    pub fye: String,
    /// 銘柄コード（5桁）
    #[serde(rename = "Code")]
    pub code: String,
    /// 会社名
    #[serde(rename = "CoName")]
    pub co_name: String,
    /// 会社名（英語）
    #[serde(rename = "CoNameEn")]
    pub co_name_en: String,
}

/// 配当金情報（`/fins/dividend`）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FinsDividend {
    /// 公表日 (YYYY-MM-DD)
    #[serde(rename = "PubDate")]
    pub pub_date: String,
    /// 公表時刻 (HH:MM:SS)
    #[serde(rename = "PubTime")]
    pub pub_time: String,
    /// 銘柄コード
    #[serde(rename = "Code")]
    pub code: String,
    /// 参照番号
    #[serde(rename = "RefNo")]
    pub ref_no: String,
    /// 状態コード
    #[serde(rename = "StatCode")]
    pub stat_code: String,
    /// 取締役会決議日 (YYYY-MM-DD)
    #[serde(rename = "BoardDate")]
    pub board_date: String,
    /// 配当種類コード
    #[serde(rename = "IFCode")]
    pub if_code: String,
    /// 決算期区分コード
    #[serde(rename = "FRCode")]
    pub fr_code: String,
    /// 配当対象期間
    #[serde(rename = "IFTerm")]
    pub if_term: String,
    /// 1株当たり配当金額
    #[serde(rename = "DivRate")]
    pub div_rate: FlexString,
    /// 権利確定日 (YYYY-MM-DD)
    #[serde(rename = "RecDate")]
    pub rec_date: String,
    /// 権利落ち日 (YYYY-MM-DD)
    #[serde(rename = "ExDate")]
    pub ex_date: String,
    /// 実際の権利確定日 (YYYY-MM-DD)
    #[serde(rename = "ActRecDate")]
    pub act_rec_date: String,
    /// 配当支払日 (YYYY-MM-DD)
    #[serde(rename = "PayDate")]
    pub pay_date: String,
    /// コーポレートアクション参照番号
    #[serde(rename = "CARefNo")]
    pub ca_ref_no: String,
    /// 分配金額
    #[serde(rename = "DistAmt")]
    pub dist_amt: FlexString,
    /// 利益剰余金
    #[serde(rename = "RetEarn")]
    pub ret_earn: FlexString,
    /// みなし配当
    #[serde(rename = "DeemDiv")]
    pub deem_div: FlexString,
    /// みなしキャピタルゲイン
    #[serde(rename = "DeemCapGains")]
    pub deem_cap_gains: FlexString,
    /// 純資産減少率
    #[serde(rename = "NetAssetDecRatio")]
    pub net_asset_dec_ratio: FlexString,
    /// 普通配当・特別配当区分コード
    #[serde(rename = "CommSpecCode")]
    pub comm_spec_code: String,
    /// 普通配当額
    #[serde(rename = "CommDivRate")]
    pub comm_div_rate: FlexString,
    /// 特別配当額
    #[serde(rename = "SpecDivRate")]
    pub spec_div_rate: FlexString,
}
