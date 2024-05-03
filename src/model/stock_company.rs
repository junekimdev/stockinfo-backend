// See: https://www.data.go.kr/tcs/dss/selectApiDataDetailView.do?publicDataPk=15094775

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockCompany {
    pub response: StockCompanyRes,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockCompanyRes {
    pub header: StockCompanyHeader,
    pub body: StockCompanyBody,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockCompanyHeader {
    pub result_code: String,
    pub result_msg: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockCompanyBody {
    pub num_of_rows: u16,
    pub page_no: u16,
    pub total_count: u16,
    pub items: StockCompanyItems,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockCompanyItems {
    pub item: Vec<StockCompanyItem>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockCompanyItem {
    pub bas_dt: String,
    pub srtn_cd: String,
    pub isin_cd: String,
    pub mrkt_ctg: String,
    pub itms_nm: String,
    pub crno: String,
    pub corp_nm: String,
}
