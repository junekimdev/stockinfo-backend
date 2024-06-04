use crate::utils::datetime::{date_deserialize, date_serialize, parse_date_from};

use std::str::FromStr;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StockPriceUS {
    #[serde(
        serialize_with = "date_serialize",
        deserialize_with = "date_deserialize"
    )]
    pub date: time::Date,
    #[serde(with = "rust_decimal::serde::str")]
    pub open: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub high: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub low: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub close: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub adj_close: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub volume: rust_decimal::Decimal,
}

impl From<Vec<&str>> for StockPriceUS {
    fn from(value: Vec<&str>) -> Self {
        if value.len() != 7 {
            panic!("StockPriceUS expects 7 strings in vector to construct");
        }

        Self {
            date: parse_date_from(value[0]).expect("invalid date"),
            open: rust_decimal::Decimal::from_str(value[1]).expect("invalid open"),
            high: rust_decimal::Decimal::from_str(value[2]).expect("invalid high"),
            low: rust_decimal::Decimal::from_str(value[3]).expect("invalid low"),
            close: rust_decimal::Decimal::from_str(value[4]).expect("invalid close"),
            adj_close: rust_decimal::Decimal::from_str(value[5]).expect("invalid adj_close"),
            volume: rust_decimal::Decimal::from_str(value[6]).expect("invalid volume"),
        }
    }
}

impl From<&tokio_postgres::Row> for StockPriceUS {
    fn from(value: &tokio_postgres::Row) -> Self {
        Self {
            date: value.get("date"),
            open: value.get("open"),
            high: value.get("high"),
            low: value.get("low"),
            close: value.get("close"),
            adj_close: value.get("adj_close"),
            volume: value.get("volume"),
        }
    }
}
