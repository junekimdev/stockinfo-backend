use crate::model::Ticker;
use crate::utils::{db, error::Error, settings::Settings, Result};

#[tracing::instrument(err)]
pub async fn build_ticker_db() -> Result<()> {
    let url = Settings::instance().us_stock.url.clone();
    let req_url = reqwest::Url::parse(&url.ticker).unwrap();
    let host = req_url.host_str().unwrap();

    // Get all codes
    let res = reqwest::Client::new()
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, "StockinfoRuntime/1.0.0")
        .header(reqwest::header::ACCEPT, "application/json;charset=UTF-8")
        .send()
        .await?
        .json::<std::collections::HashMap<String, Ticker>>()
        .await?;

    // Store in DB
    const SQL_CLEAR: &str = "TRUNCATE TABLE ticker RESTART IDENTITY";
    const SQL_INSERT: &str =
        "INSERT INTO ticker(cik_str, ticker, title) VALUES ($1::CHAR(10), $2::TEXT, $3::TEXT);";

    let mut db_client = db::pool().get().await?;
    let transaction = db_client.transaction().await?;
    let sql_insert = transaction.prepare(SQL_INSERT).await?;

    transaction.simple_query(SQL_CLEAR).await?;

    for ticker in res.values() {
        transaction
            .query(
                &sql_insert,
                &[&ticker.cik_str, &ticker.ticker, &ticker.title],
            )
            .await?;
    }

    Ok(transaction.commit().await?)
}

#[tracing::instrument]
pub async fn get_ticker(search_word: &str) -> Result<Vec<Ticker>> {
    const SQL: &str = "SELECT * FROM ticker WHERE title ILIKE $1;";

    let word = format!("%{}%", search_word);

    let rows = db::query(SQL, &[&word]).await?;
    if rows.is_empty() {
        return Err(Error::E404NotFound("ticker".into()));
    }

    let res = rows.into_iter().map(|row| Ticker::from(&row)).collect();

    Ok(res)
}
