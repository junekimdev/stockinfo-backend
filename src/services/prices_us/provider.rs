use rust_decimal::prelude::*;

use crate::model::{
    stockprice_us_from_yahoo, web, StockPriceUS, StockUSDayPriceRes, StockUSPriceExistsRes,
    StockUSWeekPrice, StockUSWeeklyPriceRes,
};
use crate::utils::{datetime::get_sunday_of_week, db, settings::Settings, Result};

type WeeklyPriceHashMap = std::collections::HashMap<(i32, u8), Vec<StockPriceUS>>;

#[tracing::instrument(err)]
pub async fn build_price_db(ticker: &str) -> Result<()> {
    // Get prices data from web
    let res = update_prices_web(ticker, None).await?;

    // Aggregate prices according to week
    let map = map_by_week(res);

    // Update DB
    update_weekly_price_db(ticker, map).await
}

#[tracing::instrument(err)]
pub async fn update_price_db(ticker: &str) -> Result<()> {
    // Check last date in DB
    const SQL_LATEST_DATE: &str = "SELECT MAX(date) FROM price_us WHERE ticker=$1::VARCHAR(10);";

    let rows = db::query(SQL_LATEST_DATE, &[&ticker]).await?;
    let last_date: Option<time::Date> = rows[0].get("max");
    let date_from = match last_date {
        None => None,
        Some(date) => Some(get_sunday_of_week(&date)?),
    };

    // Get prices data from web from last week
    let prices = update_prices_web(ticker, date_from).await?;

    // Aggregate prices according to week
    let map = map_by_week(prices);

    // Update DB
    update_weekly_price_db(ticker, map).await
}

#[tracing::instrument(err)]
pub async fn get_price_latest(ticker: &str) -> Result<StockUSDayPriceRes> {
    let agent = Settings::instance().agent.common.clone() + "/" + env!("CARGO_PKG_VERSION");
    let url = Settings::instance().urls.us_price.clone() + "/" + ticker;
    let req_url = reqwest::Url::parse(&url).unwrap();
    let host = req_url.host_str().unwrap();
    let prev_week = time::OffsetDateTime::now_utc()
        .saturating_sub(time::Duration::WEEK)
        .unix_timestamp()
        .to_string();
    let now = time::OffsetDateTime::now_utc().unix_timestamp().to_string();

    // Get all codes
    let res = reqwest::Client::new()
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, agent)
        .header(reqwest::header::ACCEPT, "application/json")
        .query(&[
            ("symbol", ticker),
            ("period1", prev_week.as_str()),
            ("period2", now.as_str()),
            ("interval", "1d"),
            ("useYfid", "true"),
            ("includePrePost", "true"),
            ("events", "div|split|earn"),
            ("lang", "en-US"),
            ("region", "US"),
            ("corsDomain", "finance.yahoo.com"),
        ])
        .send()
        .await?
        .json::<web::yahoo::ResBody>()
        .await?;

    let prices: Vec<StockPriceUS> = stockprice_us_from_yahoo(&res)?;

    Ok(StockUSDayPriceRes {
        ticker: ticker.to_string(),
        prices,
    })
}

#[tracing::instrument(err)]
pub async fn get_price_daily(ticker: &str) -> Result<StockUSDayPriceRes> {
    const SQL: &str =
        "SELECT * from price_us WHERE ticker=$1::CHAR(6) ORDER BY date DESC LIMIT 400;";

    let mut rows = db::query(SQL, &[&ticker]).await?;
    if rows.is_empty() {
        update_price_db(ticker).await?;
        rows = db::query(SQL, &[&ticker]).await?;
    }

    let res = rows
        .into_iter()
        .map(|row| StockPriceUS::from(&row))
        .collect();

    Ok(StockUSDayPriceRes {
        ticker: ticker.to_string(),
        prices: res,
    })
}

#[tracing::instrument(err)]
pub async fn get_price_weekly(ticker: &str) -> Result<StockUSWeeklyPriceRes> {
    const SQL: &str =
        "SELECT * from price_us_weekly WHERE ticker=$1::VARCHAR(10) ORDER BY opening_date DESC LIMIT 400;";

    let mut rows = db::query(SQL, &[&ticker]).await?;
    if rows.is_empty() {
        update_price_db(ticker).await?;
        rows = db::query(SQL, &[&ticker]).await?;
    }

    let res = rows
        .into_iter()
        .map(|row| StockUSWeekPrice::from(&row))
        .collect();

    Ok(StockUSWeeklyPriceRes {
        ticker: ticker.to_string(),
        prices: res,
    })
}

#[tracing::instrument(ret, err)]
pub async fn get_price_exists(ticker: &str) -> Result<StockUSPriceExistsRes> {
    const SQL: &str = "SELECT id from price_us WHERE ticker=$1::VARCHAR(10);";

    let rows = db::query(SQL, &[&ticker]).await?;

    Ok(StockUSPriceExistsRes {
        ticker: ticker.to_string(),
        exists: !rows.is_empty(),
    })
}

#[tracing::instrument(err)]
pub async fn clear_prices() -> Result<()> {
    const SQL: &str = "TRUNCATE TABLE price_us, price_us_weekly RESTART IDENTITY;";

    let db_client = db::pool().get().await?;
    db_client.simple_query(SQL).await?;

    Ok(())
}

#[tracing::instrument(err)]
async fn update_prices_web(
    ticker: &str,
    date_from: Option<time::Date>,
) -> Result<Vec<StockPriceUS>> {
    let agent = Settings::instance().agent.common.clone() + "/" + env!("CARGO_PKG_VERSION");
    let url = Settings::instance().urls.us_price.clone() + "/" + ticker;
    let req_url = reqwest::Url::parse(&url).unwrap();
    let host = req_url.host_str().unwrap();
    let start_day = match date_from {
        Some(date) => time::OffsetDateTime::new_in_offset(
            date,
            time::Time::from_hms(9, 30, 0).unwrap(), // 9:30ET is the opening time
            time::UtcOffset::from_hms(-5, 0, 0).unwrap(), // Use EST timezone; MIDNIGHT in EST comes later than in EDT
        )
        .unix_timestamp()
        .to_string(),
        None => "1577836800".to_string(), // 2020-01-01
    };
    let prev_day = time::OffsetDateTime::now_utc()
        .to_offset(time::macros::offset!(-5)) // Use EST timezone; MIDNIGHT in EST comes later than in EDT
        .replace_time(time::Time::MIDNIGHT)
        .saturating_sub(time::Duration::SECOND)
        .unix_timestamp()
        .to_string();

    // Get all codes
    let res = reqwest::Client::new()
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, agent)
        .header(reqwest::header::ACCEPT, "application/json")
        .query(&[
            ("symbol", ticker),
            ("period1", &start_day),
            ("period2", &prev_day),
            ("interval", "1d"),
            ("useYfid", "true"),
            ("includePrePost", "true"),
            ("events", "div|split|earn"),
            ("lang", "en-US"),
            ("region", "US"),
            ("corsDomain", "finance.yahoo.com"),
        ])
        .send()
        .await?
        .json::<web::yahoo::ResBody>()
        .await?;

    let prices: Vec<StockPriceUS> = stockprice_us_from_yahoo(&res)?;

    // Store in DB
    const SQL_INSERT: &str = "
        INSERT INTO price_us(ticker,date,open,high,low,close,adj_close,volume)
        VALUES ($1::VARCHAR(10),
                $2::DATE,
                $3::DECIMAL,
                $4::DECIMAL,
                $5::DECIMAL,
                $6::DECIMAL,
                $7::DECIMAL,
                $8::DECIMAL)
        ON CONFLICT (ticker,date) DO NOTHING;";

    let mut db_client = db::pool().get().await?;
    let transaction = db_client.transaction().await?;
    let sql_insert = transaction.prepare(SQL_INSERT).await?;

    for item in &prices {
        transaction
            .query(
                &sql_insert,
                &[
                    &ticker,
                    &item.date,
                    &item.open,
                    &item.high,
                    &item.low,
                    &item.close,
                    &item.adj_close,
                    &item.volume,
                ],
            )
            .await?;
    }
    transaction.commit().await?;

    Ok(prices)
}

#[tracing::instrument(err)]
async fn update_weekly_price_db(ticker: &str, map: WeeklyPriceHashMap) -> Result<()> {
    // Store in DB
    const SQL_INSERT_WEEKLY: &str = "
        INSERT INTO price_us_weekly(ticker,year,week,opening_date,closing_date,open,high,low,close,volume) 
        VALUES ($1::VARCHAR(10),
                $2::INTEGER,
                $3::INTEGER,
                $4::DATE,
                $5::DATE,
                $6::DECIMAL,
                $7::DECIMAL,
                $8::DECIMAL,
                $9::DECIMAL,
                $10::DECIMAL)
        ON CONFLICT (ticker,year,week) DO UPDATE SET
            closing_date = EXCLUDED.closing_date,
            open = EXCLUDED.open,
            high = EXCLUDED.high,
            low = EXCLUDED.low,
            close = EXCLUDED.close,
            volume = EXCLUDED.volume;";

    let mut db_client = db::pool().get().await?;
    let transaction = db_client.transaction().await?;
    let sql_insert_weekly = transaction.prepare(SQL_INSERT_WEEKLY).await?;

    for (k, mut v) in map {
        let (year, week) = k;
        v.sort_by(|a, b| a.date.cmp(&b.date));

        let (open, high, low, close, volume) = aggregate_to_weekly(&v);

        transaction
            .query(
                &sql_insert_weekly,
                &[
                    &ticker,
                    &year,
                    &i32::from(week),
                    &v[0].date,
                    &v[v.len() - 1].date,
                    &open,
                    &high,
                    &low,
                    &close,
                    &volume,
                ],
            )
            .await?;
    }

    Ok(transaction.commit().await?)
}

#[tracing::instrument(skip_all)]
fn map_by_week(prices: Vec<StockPriceUS>) -> WeeklyPriceHashMap {
    let mut map: WeeklyPriceHashMap = std::collections::HashMap::with_capacity(prices.len());

    for item in prices {
        if item.open == Decimal::ZERO {
            // exchange did not happen for the company on that day for a reason
            continue;
        }

        let k = (item.date.year(), item.date.sunday_based_week());
        if let Some(v) = map.get_mut(&k) {
            v.push(item);
        } else {
            map.insert(k, vec![item]);
        }
    }
    map
}

#[tracing::instrument(skip_all)]
fn aggregate_to_weekly(v: &Vec<StockPriceUS>) -> (Decimal, Decimal, Decimal, Decimal, Decimal) {
    let last_index = v.len() - 1;
    let open = (v[0].open * v[0].adj_close / v[0].close).round_dp(6);
    let close = v[last_index].adj_close;
    let mut high = Decimal::MIN;
    let mut low = Decimal::MAX;
    let mut volume = Decimal::ZERO;

    for item in v {
        let adj_scale = item.adj_close / item.close;
        let h = (item.high * adj_scale).round_dp(6);
        let l = (item.low * adj_scale).round_dp(6);
        volume += item.volume;

        if h > high {
            high = h;
        }
        if l < low {
            low = l;
        }
    }
    (open, high, low, close, volume)
}
