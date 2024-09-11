use rust_decimal::prelude::FromPrimitive;

use crate::utils::datetime::{date_deserialize, date_serialize};
use crate::utils::Result;

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

#[tracing::instrument(err)]
pub fn stockprice_us_from_yahoo(res: &super::web::yahoo::ResBody) -> Result<Vec<StockPriceUS>> {
    let size = res.chart.result[0].timestamp.len();
    let mut r: Vec<StockPriceUS> = Vec::with_capacity(size);

    for i in 0..size {
        let t = res.chart.result[0].timestamp[i];
        let datetime = time::OffsetDateTime::from_unix_timestamp(t).expect("invalid timestamp");
        let open = res.chart.result[0].indicators.quote[0].open[i];
        let high = res.chart.result[0].indicators.quote[0].high[i];
        let low = res.chart.result[0].indicators.quote[0].low[i];
        let close = res.chart.result[0].indicators.quote[0].close[i];
        let volume = res.chart.result[0].indicators.quote[0].volume[i];
        let adj_close = res.chart.result[0].indicators.adj_close[0].adj_close[i];

        r.push(StockPriceUS {
            date: datetime.date(),
            open: rust_decimal::Decimal::from_f32(open).expect("invalid open"),
            high: rust_decimal::Decimal::from_f32(high).expect("invalid high"),
            low: rust_decimal::Decimal::from_f32(low).expect("invalid low"),
            close: rust_decimal::Decimal::from_f32(close).expect("invalid close"),
            adj_close: rust_decimal::Decimal::from_f32(adj_close).expect("invalid adj_close"),
            volume: rust_decimal::Decimal::from_u64(volume).expect("invalid volume"),
        })
    }

    Ok(r)
}
