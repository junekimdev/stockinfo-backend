use crate::model::{
    krx, web, StockDayPrice, StockDayPriceRes, StockPrice, StockPriceExistsRes, StockPriceItem,
    StockWeekPrice, StockWeeklyPriceRes,
};
use crate::utils::{
    cache, datetime::get_sunday_of_week, db, error::Error, settings::Settings, Result,
};
use rust_decimal::prelude::*;

type WeeklyPriceHashMap = std::collections::HashMap<(i32, u8), Vec<StockPriceItem>>;

#[tracing::instrument(err)]
pub async fn build_price_db(stock_code: &str) -> Result<()> {
    // Get prices data from web
    let prices = update_prices_web(stock_code, None).await?;

    // Aggregate prices according to week
    let map = map_by_week(prices);

    // Update DB
    update_weekly_price_db(map).await
}

#[tracing::instrument(err)]
pub async fn update_price_db(stock_code: &str) -> Result<()> {
    // Check last date in DB
    const SQL_LATEST_DATE: &str = "SELECT MAX(bas_dt) FROM price WHERE srtn_cd=$1::CHAR(6);";

    let rows = db::query(SQL_LATEST_DATE, &[&stock_code]).await?;
    let last_date: Option<time::Date> = rows[0].get("max");
    let date_from = match last_date {
        None => None,
        Some(date) => Some(get_sunday_of_week(&date)?),
    };

    // Get prices data from web from last week
    let prices = update_prices_web(stock_code, date_from).await?;

    // Aggregate prices according to week
    let map = map_by_week(prices);

    // Update DB
    update_weekly_price_db(map).await
}

#[tracing::instrument(err)]
pub async fn get_price_all_latest() -> Result<krx::ResBody> {
    let cache_key = "dbms/MDC/STAT/standard/MDCSTAT01501";
    let cache_time = 600; // 10 minutes

    // Check cache first
    let res = match cache::get::<Option<krx::ResBody>>(cache_key).await? {
        Some(data) => data,
        None => {
            // Fetch current price data of all companies in krx
            let data = fetch_krx_prices_all().await?;

            // Save data in cache
            cache::set_with_timer(cache_key, &data, cache_time).await?;

            data
        }
    };

    Ok(res)
}

#[tracing::instrument(err)]
pub async fn get_price_latest(stock_code: &str) -> Result<krx::ResBody> {
    // Check cache first
    let res = get_price_all_latest().await?;

    // Filter data to find price for the company
    let prices: Vec<krx::Price> = res
        .prices
        .into_iter()
        .filter(|v| v.isu_srt_cd == stock_code)
        .collect();

    Ok(krx::ResBody {
        current_datetime: res.current_datetime,
        prices,
    })
}

#[tracing::instrument(err)]
pub async fn get_price_daily(stock_code: &str) -> Result<StockDayPriceRes> {
    const SQL: &str =
        "SELECT * from price WHERE srtn_cd=$1::CHAR(6) ORDER BY bas_dt DESC LIMIT 400;";

    let mut rows = db::query(SQL, &[&stock_code]).await?;
    if rows.is_empty() {
        update_price_db(stock_code).await?;
        rows = db::query(SQL, &[&stock_code]).await?;
    }

    let res = rows
        .into_iter()
        .map(|row| StockDayPrice::from(&row))
        .collect();

    Ok(StockDayPriceRes {
        srtn_cd: stock_code.to_string(),
        prices: res,
    })
}

#[tracing::instrument(err)]
pub async fn get_price_weekly(stock_code: &str) -> Result<StockWeeklyPriceRes> {
    const SQL: &str =
        "SELECT * from price_weekly WHERE srtn_cd=$1::CHAR(6) ORDER BY opening_date DESC LIMIT 400;";

    let mut rows = db::query(SQL, &[&stock_code]).await?;
    if rows.is_empty() {
        update_price_db(stock_code).await?;
        rows = db::query(SQL, &[&stock_code]).await?;
    }

    let res = rows
        .into_iter()
        .map(|row| StockWeekPrice::from(&row))
        .collect();

    Ok(StockWeeklyPriceRes {
        srtn_cd: stock_code.to_string(),
        prices: res,
    })
}

#[tracing::instrument(ret, err)]
pub async fn get_price_exists(stock_code: &str) -> Result<StockPriceExistsRes> {
    const SQL: &str = "SELECT id from price WHERE srtn_cd=$1::CHAR(6);";

    let rows = db::query(SQL, &[&stock_code]).await?;

    Ok(StockPriceExistsRes {
        srtn_cd: stock_code.to_string(),
        exists: !rows.is_empty(),
    })
}

#[tracing::instrument(err)]
pub async fn clear_prices() -> Result<()> {
    const SQL: &str = "TRUNCATE TABLE price, price_weekly RESTART IDENTITY;";

    let db_client = db::pool().get().await?;
    db_client.simple_query(SQL).await?;

    Ok(())
}

#[tracing::instrument(err)]
async fn fetch_krx_prices_all() -> Result<krx::ResBody> {
    let agent = Settings::instance().agent.common.clone() + "/" + env!("CARGO_PKG_VERSION");
    let url_login = Settings::instance().urls.kr_krx_login.clone();
    let url_date = Settings::instance().urls.kr_krx_price_date.clone();
    let url_price = Settings::instance().urls.kr_krx_price.clone();
    let referer_login = Settings::instance().urls.kr_krx_login_referer.clone();
    let referer_price = Settings::instance().urls.kr_krx_price_referer.clone();
    let login_id = Settings::instance().keys.krx_id.clone();
    let login_pw = Settings::instance().keys.krx_pw.clone();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, agent.parse().unwrap());
    headers.insert(
        reqwest::header::ACCEPT,
        "application/json;charset=UTF-8".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::HOST,
        reqwest::Url::parse(&url_price)
            .unwrap()
            .host_str()
            .unwrap()
            .parse()
            .unwrap(),
    );

    // Get a client with cookie store
    let web_client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .default_headers(headers)
        .build()?;

    // Login to data.krx.co.kr
    web_client
        .post(url_login)
        .header(reqwest::header::REFERER, &referer_login)
        .form(&[("mbrId", login_id), ("pw", login_pw)])
        .send()
        .await?
        .error_for_status()?;

    // Get latest date of the data from KRX
    let res = web_client
        .get(url_date)
        .header(reqwest::header::REFERER, &referer_price)
        .query(&[
            ("baseName", "krx.mdc.i18n.component"),
            ("key", "B128.bld"),
            ("locale", "ko"),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<web::krx::LatestDateRes>()
        .await?;

    let date_latest = res.result.output[0].get("max_work_dt").unwrap();

    // Fetch data from internet
    let res = web_client
        .post(url_price)
        .header(reqwest::header::REFERER, &referer_price)
        .form(&[
            ("bld", "dbms/MDC/STAT/standard/MDCSTAT01501"),
            ("locale", "ko_KR"),
            ("mktId", "ALL"),
            ("share", "1"),
            ("money", "1"),
            ("csvxls_isNo", "false"),
            ("trdDd", date_latest),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<web::krx::ResBody>()
        .await?;

    Ok(krx::ResBody::from(res))
}

#[tracing::instrument(err)]
async fn update_prices_web(
    stock_code: &str,
    date_from: Option<time::Date>,
) -> Result<Vec<StockPriceItem>> {
    let agent = Settings::instance().agent.common.clone() + "/" + env!("CARGO_PKG_VERSION");
    let key = Settings::instance().keys.data_go_kr.clone();
    let url = Settings::instance().urls.kr_price.clone();
    let req_url = reqwest::Url::parse(&url).unwrap();
    let host = req_url.host_str().unwrap();

    // Get all prices
    let mut req = reqwest::Client::new()
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, agent)
        .header(reqwest::header::ACCEPT, "application/json;charset=UTF-8");

    if let Some(from) = date_from {
        let date_format = time::macros::format_description!("[year][month][day]");
        req = req.query(&[
            ("serviceKey", key.as_str()),
            ("resultType", "json"),
            ("numOfRows", "1000000"),
            ("likeSrtnCd", stock_code),
            ("beginBasDt", from.format(&date_format)?.as_str()),
        ]);
    } else {
        req = req.query(&[
            ("serviceKey", key.as_str()),
            ("resultType", "json"),
            ("numOfRows", "1000000"),
            ("likeSrtnCd", stock_code),
        ]);
    }

    let res = req
        .send()
        .await?
        .error_for_status()?
        .json::<StockPrice>()
        .await?;
    if res.response.body.total_count < 1 {
        return Err(Error::E404NotFound("No data found from web".into()));
    }

    let prices = res.response.body.items.item;

    // Store in DB
    const SQL_INSERT: &str = "
        INSERT INTO price(bas_dt,srtn_cd,isin_cd,itms_nm,mrkt_ctg,
            clpr,vs,flt_rt,mkp,hipr,lopr,trqu,tr_prc,lstg_st_cnt,mrkt_tot_amt)
        VALUES ($1::DATE,
                $2::CHAR(6),
                $3::CHAR(12),
                $4::VARCHAR(120),
                $5::VARCHAR(6),
                $6::INTEGER,
                $7::INTEGER,
                $8::REAL,
                $9::INTEGER,
                $10::INTEGER,
                $11::INTEGER,
                $12::DECIMAL,
                $13::DECIMAL,
                $14::DECIMAL,
                $15::DECIMAL)
        ON CONFLICT (bas_dt,srtn_cd) DO NOTHING;";

    let mut db_client = db::pool().get().await?;
    let transaction = db_client.transaction().await?;
    let sql_insert = transaction.prepare(SQL_INSERT).await?;

    for item in &prices {
        transaction
            .query(
                &sql_insert,
                &[
                    &item.bas_dt,
                    &item.srtn_cd,
                    &item.isin_cd,
                    &item.itms_nm,
                    &item.mrkt_ctg,
                    &item.clpr,
                    &item.vs,
                    &item.flt_rt,
                    &item.mkp,
                    &item.hipr,
                    &item.lopr,
                    &item.trqu,
                    &item.tr_prc,
                    &item.lstg_st_cnt,
                    &item.mrkt_tot_amt,
                ],
            )
            .await?;
    }
    transaction.commit().await?;

    Ok(prices)
}

#[tracing::instrument(err)]
async fn update_weekly_price_db(map: WeeklyPriceHashMap) -> Result<()> {
    // Store in DB
    const SQL_INSERT_WEEKLY: &str = "
        INSERT INTO price_weekly(srtn_cd,year,week,opening_date,closing_date,
            open,close,high,low,volume,trading_value,base_stock_cnt) 
        VALUES ($1::CHAR(6),
                $2::INTEGER,
                $3::INTEGER,
                $4::DATE,
                $5::DATE,
                $6::DECIMAL,
                $7::DECIMAL,
                $8::DECIMAL,
                $9::DECIMAL,
                $10::DECIMAL,
                $11::DECIMAL,
                $12::DECIMAL)
        ON CONFLICT (srtn_cd,year,week) DO UPDATE SET
            closing_date = EXCLUDED.closing_date,
            open = EXCLUDED.open,
            close = EXCLUDED.close,
            high = EXCLUDED.high,
            low = EXCLUDED.low,
            volume = EXCLUDED.volume,
            base_stock_cnt = EXCLUDED.base_stock_cnt;";

    let mut db_client = db::pool().get().await?;
    let transaction = db_client.transaction().await?;
    let sql_insert_weekly = transaction.prepare(SQL_INSERT_WEEKLY).await?;

    for (k, mut v) in map {
        let (year, week) = k;
        v.sort_by(|a, b| a.bas_dt.cmp(&b.bas_dt));

        let (open, close, high, low, volume, trading_value, base) = aggregate_to_weekly(&v);

        transaction
            .query(
                &sql_insert_weekly,
                &[
                    &v[0].srtn_cd,
                    &year,
                    &i32::from(week),
                    &v[0].bas_dt,
                    &v[v.len() - 1].bas_dt,
                    &open,
                    &close,
                    &high,
                    &low,
                    &volume,
                    &trading_value,
                    &base,
                ],
            )
            .await?;
    }

    Ok(transaction.commit().await?)
}

#[tracing::instrument(skip_all)]
fn map_by_week(prices: Vec<StockPriceItem>) -> WeeklyPriceHashMap {
    let mut map: WeeklyPriceHashMap = std::collections::HashMap::with_capacity(prices.len());

    for item in prices {
        if item.mkp == 0 {
            // exchange did not happen for the company on that day for a reason
            continue;
        }

        let k = (item.bas_dt.year(), item.bas_dt.sunday_based_week());
        if let Some(v) = map.get_mut(&k) {
            v.push(item);
        } else {
            map.insert(k, vec![item]);
        }
    }
    map
}

fn aggregate_to_weekly(
    v: &Vec<StockPriceItem>,
) -> (
    Decimal,
    Decimal,
    Decimal,
    Decimal,
    Decimal,
    Decimal,
    Decimal,
) {
    let last_index = v.len() - 1;
    let base = v[last_index].lstg_st_cnt;
    let init_adj_scale = v[0].lstg_st_cnt / base;
    let open = (Decimal::from(v[0].mkp) * init_adj_scale).round();
    let close = Decimal::from(v[last_index].clpr);
    let mut high = (Decimal::from(v[0].hipr) * init_adj_scale).round();
    let mut low = (Decimal::from(v[0].lopr) * init_adj_scale).round();
    let mut volume = Decimal::ZERO;
    let mut trading_value = Decimal::ZERO;

    for item in v {
        let adj_scale = item.lstg_st_cnt / base;
        let h = (Decimal::from(item.hipr) * adj_scale).round();
        let l = (Decimal::from(item.lopr) * adj_scale).round();
        volume += item.trqu;
        trading_value += item.tr_prc;

        if h > high {
            high = h;
        }
        if l < low {
            low = l;
        }
    }
    (open, close, high, low, volume, trading_value, base)
}
