use crate::utils::datetime::{date_deserialize, date_serialize};
use crate::utils::from_str_deserialize;

// See: https://www.data.go.kr/tcs/dss/selectApiDataDetailView.do?publicDataPk=15094808

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPrice {
    pub response: StockPriceRes,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPriceRes {
    pub header: StockPriceRHeader,
    pub body: StockPriceBody,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPriceRHeader {
    pub result_code: String,
    pub result_msg: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPriceBody {
    pub num_of_rows: u16,
    pub page_no: u16,
    pub total_count: u16,
    pub items: StockPriceItems,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPriceItems {
    pub item: Vec<StockPriceItem>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPriceItem {
    #[serde(
        serialize_with = "date_serialize",
        deserialize_with = "date_deserialize"
    )]
    pub bas_dt: time::Date,
    pub srtn_cd: String,
    pub isin_cd: String,
    pub itms_nm: String,
    pub mrkt_ctg: String,
    #[serde(deserialize_with = "from_str_deserialize")]
    pub clpr: i32,
    #[serde(deserialize_with = "from_str_deserialize")]
    pub vs: i32,
    #[serde(deserialize_with = "from_str_deserialize")]
    pub flt_rt: f32,
    #[serde(deserialize_with = "from_str_deserialize")]
    pub mkp: i32,
    #[serde(deserialize_with = "from_str_deserialize")]
    pub hipr: i32,
    #[serde(deserialize_with = "from_str_deserialize")]
    pub lopr: i32,
    #[serde(with = "rust_decimal::serde::str")]
    pub trqu: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub tr_prc: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub lstg_st_cnt: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub mrkt_tot_amt: rust_decimal::Decimal,
}
