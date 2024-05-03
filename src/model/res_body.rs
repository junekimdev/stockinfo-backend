use crate::utils::datetime::date_serialize;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockCompanySearchRes {
    pub itms_nm: String,
    pub srtn_cd: String,
    pub isin_cd: String,
    pub mrkt_ctg: String,
    pub crno: String,
    pub corp_nm: String,
}

impl From<&tokio_postgres::Row> for StockCompanySearchRes {
    fn from(value: &tokio_postgres::Row) -> Self {
        Self {
            itms_nm: value.get("itms_nm"),
            srtn_cd: value.get("srtn_cd"),
            isin_cd: value.get("isin_cd"),
            mrkt_ctg: value.get("mrkt_ctg"),
            crno: value.get("crno"),
            corp_nm: value.get("corp_nm"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StockWeeklyPriceRes {
    pub srtn_cd: String,
    pub prices: Vec<StockWeekPrice>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StockWeekPrice {
    pub year: i32,
    pub week: i32,
    #[serde(serialize_with = "date_serialize")]
    pub opening_date: time::Date,
    #[serde(serialize_with = "date_serialize")]
    pub closing_date: time::Date,
    pub open: rust_decimal::Decimal,
    pub close: rust_decimal::Decimal,
    pub high: rust_decimal::Decimal,
    pub low: rust_decimal::Decimal,
    pub volume: rust_decimal::Decimal,
    pub trading_value: rust_decimal::Decimal,
    pub base_stock_cnt: rust_decimal::Decimal,
}

impl From<&tokio_postgres::Row> for StockWeekPrice {
    fn from(value: &tokio_postgres::Row) -> Self {
        Self {
            year: value.get("year"),
            week: value.get("week"),
            opening_date: value.get("opening_date"),
            closing_date: value.get("closing_date"),
            open: value.get("open"),
            close: value.get("close"),
            high: value.get("high"),
            low: value.get("low"),
            volume: value.get("volume"),
            trading_value: value.get("trading_value"),
            base_stock_cnt: value.get("base_stock_cnt"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StockDayPriceRes {
    pub srtn_cd: String,
    pub prices: Vec<StockDayPrice>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StockDayPrice {
    #[serde(serialize_with = "date_serialize")]
    pub date: time::Date,
    pub open: i32,
    pub close: i32,
    pub high: i32,
    pub low: i32,
    pub volume: rust_decimal::Decimal,
    pub trading_value: rust_decimal::Decimal,
    pub base_stock_cnt: rust_decimal::Decimal,
}

impl From<&tokio_postgres::Row> for StockDayPrice {
    fn from(value: &tokio_postgres::Row) -> Self {
        Self {
            date: value.get("bas_dt"),
            open: value.get("mkp"),
            close: value.get("clpr"),
            high: value.get("hipr"),
            low: value.get("lopr"),
            volume: value.get("trqu"),
            trading_value: value.get("tr_prc"),
            base_stock_cnt: value.get("lstg_st_cnt"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StockPriceExistsRes {
    pub srtn_cd: String,
    pub exists: bool,
}
