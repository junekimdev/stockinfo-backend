use crate::utils::datetime::krx_datetime_deserialize;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ResBody {
    #[serde(
        rename(deserialize = "CURRENT_DATETIME"),
        deserialize_with = "krx_datetime_deserialize"
    )]
    pub current_datetime: time::OffsetDateTime,

    #[serde(rename(deserialize = "OutBlock_1"))]
    pub prices: Vec<Price>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
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

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LatestDateRes {
    // pub controller: String,
    // pub dir: String,
    pub result: MaxDateResult,
    // pub cmd: String,
    #[serde(flatten)]
    _extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MaxDateResult {
    pub output: Vec<std::collections::HashMap<String, String>>,
}
