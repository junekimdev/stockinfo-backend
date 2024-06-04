// https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2022001
use deadpool_redis::redis;

#[allow(unused)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Code {
    pub corp_code: String,
    pub corp_name: String,
    pub modify_date: time::Date,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexRes {
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<IndexItem>>,
}

impl redis::ToRedisArgs for IndexRes {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(serde_json::to_string(self).unwrap().as_bytes())
    }
}

impl redis::FromRedisValue for IndexRes {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::Data(ref bytes) => {
                let msg = std::str::from_utf8(bytes)?;
                let object = serde_json::from_str::<Self>(msg).unwrap();
                Ok(object)
            }
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Response was of incompatible type",
                format!(
                    "{:?} (response was {:?})",
                    "Response type not dart::IndexRes compatible", v
                ),
            ))),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexItem {
    pub bsns_year: String,
    pub corp_code: String,
    pub stock_code: String,
    pub reprt_code: String,
    pub idx_cl_code: String,
    pub idx_cl_nm: String,
    pub idx_code: String,
    pub idx_nm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idx_val: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatementRes {
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<StatementItem>>,
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
            redis::Value::Data(ref bytes) => {
                let msg = std::str::from_utf8(bytes)?;
                let object = serde_json::from_str::<Self>(msg).unwrap();
                Ok(object)
            }
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Response was of incompatible type",
                format!(
                    "{:?} (response was {:?})",
                    "Response type not dart::StatementRes compatible", v
                ),
            ))),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatementItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcept_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reprt_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsns_year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sj_div: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sj_nm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_nm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thstrm_nm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thstrm_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thstrm_add_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frmtrm_nm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frmtrm_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frmtrm_q_nm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frmtrm_q_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frmtrm_add_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bfefrmtrm_nm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bfefrmtrm_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
