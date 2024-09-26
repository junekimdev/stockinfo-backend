use super::xbrl;
use crate::utils::datetime::{date_opt_deserialize, date_opt_serialize};
use deadpool_redis::redis;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Submissions {
    pub cik: String,
    pub name: String,
    pub tickers: Vec<String>,
    pub exchanges: Vec<String>,
    pub fiscal_year_end: String,
    pub filings: SubmissionsFilings,
    #[serde(flatten)]
    pub _extra: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionsFilings {
    pub recent: SubmissionsRecent,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionsRecent {
    pub form: Vec<String>,
    pub acceptance_date_time: Vec<String>,
    pub filing_date: Vec<String>,
    pub accession_number: Vec<String>,
    pub primary_document: Vec<String>,
    pub report_date: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementRes {
    pub cik: String,
    pub outstanding_stock: Vec<StatementItem>,
    pub assets: Vec<StatementItem>,
    pub equity: Vec<StatementItem>,
    pub liabilities: Vec<StatementItem>,
    pub revenue: Vec<StatementItem>,
    pub operating_income: Vec<StatementItem>,
    pub net_income: Vec<StatementItem>,
    pub comprehensive_income: Vec<StatementItem>,
    pub operating_cash_flow: Vec<StatementItem>,
    pub investing_cash_flow: Vec<StatementItem>,
    pub financing_cash_flow: Vec<StatementItem>,
}

impl redis::ToRedisArgs for StatementRes {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(serde_json::to_string(self).unwrap().as_bytes())
    }
}

impl redis::FromRedisValue for StatementRes {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::BulkString(ref bytes) => {
                let msg = std::str::from_utf8(bytes)?;
                let object = serde_json::from_str::<Self>(msg).unwrap();
                Ok(object)
            }
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Response was of incompatible type",
                format!(
                    "{:?} (response was {:?})",
                    "Response type not edgar::StatementRes compatible", v
                ),
            ))),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatementItem {
    #[serde(
        serialize_with = "date_opt_serialize",
        deserialize_with = "date_opt_deserialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub date: Option<time::Date>,
    #[serde(
        serialize_with = "date_opt_serialize",
        deserialize_with = "date_opt_deserialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub start_date: Option<time::Date>,
    #[serde(
        serialize_with = "date_opt_serialize",
        deserialize_with = "date_opt_deserialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub end_date: Option<time::Date>,
    pub value: String,
}

impl From<(xbrl::Period, String)> for StatementItem {
    fn from(value: (xbrl::Period, String)) -> Self {
        let (period, v) = value;
        let xbrl::Period {
            date,
            start_date,
            end_date,
        } = period;

        Self {
            date,
            start_date,
            end_date,
            value: v,
        }
    }
}
