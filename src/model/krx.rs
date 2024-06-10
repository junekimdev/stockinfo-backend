use deadpool_redis::redis;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResBody {
    #[serde(
        deserialize_with = "time::serde::rfc3339::deserialize",
        serialize_with = "time::serde::rfc3339::serialize"
    )]
    pub current_datetime: time::OffsetDateTime,

    pub prices: Vec<Price>,
}

impl From<super::web::krx::ResBody> for ResBody {
    fn from(value: super::web::krx::ResBody) -> Self {
        Self {
            current_datetime: value.current_datetime,
            prices: value.prices.into_iter().map(Price::from).collect(),
        }
    }
}

impl redis::ToRedisArgs for ResBody {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(serde_json::to_string(self).unwrap().as_bytes())
    }
}

impl redis::FromRedisValue for ResBody {
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
                    "Response type not krx::ResBody compatible", v
                ),
            ))),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Price {
    pub sect_tp_nm: String,
    pub isu_srt_cd: String,
    pub isu_cd: String,
    pub isu_abbrv: String,
    pub tdd_opnprc: String, // numeric string with commas for thousands
    pub tdd_hgprc: String,  // numeric string with commas for thousands
    pub tdd_lwprc: String,  // numeric string with commas for thousands
    pub tdd_clsprc: String, // numeric string with commas for thousands
    pub cmpprevdd_prc: String,
    pub fluc_rt: String,
    pub fluc_tp_cd: String,
    pub acc_trdvol: String, // numeric string with commas for thousands
    pub acc_trdval: String, // numeric string with commas for thousands
    pub list_shrs: String,  // numeric string with commas for thousands
    pub mktcap: String,     // numeric string with commas for thousands
    pub mkt_id: String,
    pub mkt_nm: String,
}

impl From<super::web::krx::Price> for Price {
    fn from(value: super::web::krx::Price) -> Self {
        let super::web::krx::Price {
            sect_tp_nm,
            isu_srt_cd,
            isu_cd,
            isu_abbrv,
            tdd_opnprc,
            tdd_hgprc,
            tdd_lwprc,
            tdd_clsprc,
            cmpprevdd_prc,
            fluc_rt,
            fluc_tp_cd,
            acc_trdvol,
            acc_trdval,
            list_shrs,
            mktcap,
            mkt_id,
            mkt_nm,
        } = value;
        Self {
            sect_tp_nm,
            isu_srt_cd,
            isu_cd,
            isu_abbrv,
            tdd_opnprc,
            tdd_hgprc,
            tdd_lwprc,
            tdd_clsprc,
            cmpprevdd_prc,
            fluc_rt,
            fluc_tp_cd,
            acc_trdvol,
            acc_trdval,
            list_shrs,
            mktcap,
            mkt_id,
            mkt_nm,
        }
    }
}

impl redis::ToRedisArgs for Price {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(serde_json::to_string(self).unwrap().as_bytes())
    }
}

impl redis::FromRedisValue for Price {
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
                    "Response type not krx::Price compatible", v
                ),
            ))),
        }
    }
}
