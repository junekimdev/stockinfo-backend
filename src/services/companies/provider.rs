use crate::model::{StockCompany, StockCompanySearchRes};
use crate::utils::{db, error::Error, settings::Settings, Result};

#[tracing::instrument(err)]
pub async fn build_company_db() -> Result<()> {
    let web_client = reqwest::Client::new();
    let key = Settings::instance().keys.data_go_kr.clone();
    let url = Settings::instance().urls.kr_company.clone();
    let req_url = reqwest::Url::parse(&url).unwrap();
    let host = req_url.host_str().unwrap();
    let format = time::macros::format_description!("[year][month][day]");
    let mut base_date = time::OffsetDateTime::now_utc()
        .date()
        .previous_day()
        .unwrap();

    // Get all codes
    let mut res = web_client
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, "StockinfoRuntime/1.0.0")
        .header(reqwest::header::ACCEPT, "application/json;charset=UTF-8")
        .query(&[
            ("serviceKey", key.as_str()),
            ("resultType", "json"),
            ("basDt", base_date.format(&format)?.as_str()),
            ("numOfRows", "1000000"),
        ])
        .send()
        .await?
        .json::<StockCompany>()
        .await?;

    // If total_count in body is 0, retry with previous date until data comes in
    while res.response.body.total_count < 1 {
        base_date = base_date.previous_day().unwrap();

        res = web_client
            .get(req_url.clone())
            .header(reqwest::header::HOST, host)
            .header(reqwest::header::USER_AGENT, "StockinfoRuntime/1.0.0")
            .header(reqwest::header::ACCEPT, "application/json;charset=UTF-8")
            .query(&[
                ("serviceKey", key.as_str()),
                ("resultType", "json"),
                ("basDt", base_date.format(&format)?.as_str()),
                ("numOfRows", "1000000"),
            ])
            .send()
            .await?
            .json::<StockCompany>()
            .await?;
    }

    // Store in DB
    const SQL_CLEAR: &str = "TRUNCATE TABLE company RESTART IDENTITY";
    const SQL_INSERT: &str = "
    INSERT INTO company(srtn_cd, isin_cd, mrkt_ctg, itms_nm, crno, corp_nm)
    VALUES ($1::CHAR(6), $2::CHAR(12), $3::VARCHAR(6), $4::VARCHAR(240), $5::VARCHAR(20), $6::VARCHAR(240));";

    let mut db_client = db::pool().get().await?;
    let transaction = db_client.transaction().await?;
    let sql_insert = transaction.prepare(SQL_INSERT).await?;

    transaction.simple_query(SQL_CLEAR).await?;

    for item in &res.response.body.items.item {
        transaction
            .query(
                &sql_insert,
                &[
                    &&item.srtn_cd[1..],
                    &item.isin_cd,
                    &item.mrkt_ctg,
                    &item.itms_nm,
                    &item.crno,
                    &item.corp_nm,
                ],
            )
            .await?;
    }

    Ok(transaction.commit().await?)
}

#[tracing::instrument]
pub async fn get_company(search_word: &str) -> Result<Vec<StockCompanySearchRes>> {
    const SQL: &str = "SELECT * FROM company WHERE itms_nm ILIKE $1 OR corp_nm ILIKE $1;";

    let word = format!("%{}%", search_word);

    let rows = db::query(SQL, &[&word]).await?;
    if rows.is_empty() {
        return Err(Error::E404NotFound("stock company".into()));
    }

    let res = rows
        .into_iter()
        .map(|row| StockCompanySearchRes::from(&row))
        .collect();

    Ok(res)
}
