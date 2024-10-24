use crate::model::dart;
use crate::utils::{db, error::Error, settings::Settings, Result};
use std::io::Read;

#[tracing::instrument(err)]
pub async fn build_code_db() -> Result<()> {
    let agent = Settings::instance().agent.common.clone() + "/" + env!("CARGO_PKG_VERSION");
    let key = Settings::instance().keys.dart.clone();
    let url = Settings::instance().urls.dart_code.clone();

    // Get a file from the internet
    let res = tokio::task::spawn_blocking(move || {
        let req_url = reqwest::Url::parse(&url).unwrap();
        let host = req_url.host_str().unwrap();
        let mut buf: Vec<u8> = Vec::new();
        reqwest::blocking::Client::new()
            .get(req_url.clone())
            .header(reqwest::header::HOST, host)
            .header(reqwest::header::USER_AGENT, agent)
            .header(reqwest::header::ACCEPT, "application/xml;charset=UTF-8")
            .query(&[("crtfc_key", key.as_str())])
            .send()
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        buf
    })
    .await?;

    // Unzip the downloaded file
    let res_reader = std::io::Cursor::new(res);
    let mut xml_file = String::new();
    zip::read::ZipArchive::new(res_reader)?
        .by_name("CORPCODE.xml")?
        .read_to_string(&mut xml_file)?;

    // Extract data from the file
    let doc = roxmltree::Document::parse(&xml_file)?;
    let format = time::macros::format_description!("[year][month][day]");
    let mut codes: Vec<dart::Code> = Vec::new();
    for child in doc.root_element().children() {
        if child.is_element() {
            let mut code = String::new();
            let mut name = String::new();
            let mut stock = String::new();
            let mut date = String::new();
            let elements: Vec<roxmltree::Node> =
                child.children().filter(move |v| v.is_element()).collect();

            for el in elements {
                match el.tag_name().name() {
                    "corp_code" => code = el.text().unwrap().to_string(),
                    "corp_name" => name = el.text().unwrap().to_string(),
                    "stock_code" => stock = el.text().unwrap().to_string(),
                    "modify_date" => date = el.text().unwrap().to_string(),
                    _ => (),
                }
            }

            // Add dart codes only if the company is listed on stock market
            if !stock.is_empty() || stock != " " {
                let date_parsed = match time::Date::parse(&date, &format) {
                    Ok(d) => Ok(d),
                    Err(e) => Err(Error::General(e.to_string())),
                }?;

                codes.push(dart::Code {
                    corp_code: code,
                    corp_name: name,
                    stock_code: stock,
                    modify_date: date_parsed,
                });
            }
        }
    }

    // Store data in DB
    const SQL_CLEAR: &str = "TRUNCATE TABLE dart_code RESTART IDENTITY";
    const SQL_INSERT: &str =
        "INSERT INTO dart_code(code,stock_code,name,date) VALUES ($1::CHAR(8), $2::CHAR(6), $3::TEXT, $4::DATE);";

    let mut db_client = db::pool().get().await?;
    let transaction = db_client.transaction().await?;
    let sql_insert = transaction.prepare(SQL_INSERT).await?;

    transaction.simple_query(SQL_CLEAR).await?; // Reset the table

    for item in &codes {
        transaction
            .query(
                &sql_insert,
                &[
                    &item.corp_code,
                    &item.stock_code,
                    &item.corp_name,
                    &item.modify_date,
                ],
            )
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}

#[tracing::instrument(err)]
pub async fn get_dart_code(stock_code: &str) -> Result<String> {
    const SQL: &str = "SELECT * FROM dart_code WHERE stock_code=$1::CHAR(6) ORDER BY date DESC;";

    let rows = db::query(SQL, &[&stock_code]).await?;
    if rows.is_empty() {
        return Err(Error::E404NotFound("dart code".into()));
    }

    let res = rows[0].get("code");

    Ok(res)
}

pub async fn get_index(
    corp_code: &str,
    report_code: &str,
    idx_code: &str,
) -> Result<dart::IndexRes> {
    let last_year = time::OffsetDateTime::now_utc().year() - 1;
    let agent = Settings::instance().agent.common.clone() + "/" + env!("CARGO_PKG_VERSION");
    let key = Settings::instance().keys.dart.clone();
    let url = Settings::instance().urls.dart_index.clone();
    let req_url = reqwest::Url::parse(&url).unwrap();
    let host = req_url.host_str().unwrap();

    let req_base = reqwest::Client::new()
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, agent)
        .header(reqwest::header::ACCEPT, "application/json;charset=UTF-8");

    let mut res = req_base
        .try_clone()
        .unwrap()
        .query(&[
            ("crtfc_key", key.as_str()),
            ("corp_code", corp_code),
            ("bsns_year", last_year.to_string().as_str()),
            ("reprt_code", report_code),
            ("idx_cl_code", idx_code),
        ])
        .send()
        .await?
        .json::<dart::IndexRes>()
        .await?;

    // status 013 means data NOT_FOUND
    if res.status == "013" {
        // try to get the report of the previous year
        res = req_base
            .query(&[
                ("crtfc_key", key.as_str()),
                ("corp_code", corp_code),
                ("bsns_year", (last_year - 1).to_string().as_str()),
                ("reprt_code", report_code),
                ("idx_cl_code", idx_code),
            ])
            .send()
            .await?
            .json::<dart::IndexRes>()
            .await?;
    }

    Ok(res)
}

pub async fn get_statement(
    corp_code: &str,
    report_code: &str,
    fs_div: &str,
) -> Result<dart::StatementRes> {
    let last_year = time::OffsetDateTime::now_utc().year() - 1;
    let agent = Settings::instance().agent.common.clone() + "/" + env!("CARGO_PKG_VERSION");
    let key = Settings::instance().keys.dart.clone();
    let url = Settings::instance().urls.dart_statement.clone();
    let req_url = reqwest::Url::parse(&url).unwrap();
    let host = req_url.host_str().unwrap();

    let req_base = reqwest::Client::new()
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, agent)
        .header(reqwest::header::ACCEPT, "application/json;charset=UTF-8");

    let mut res = req_base
        .try_clone()
        .unwrap()
        .query(&[
            ("crtfc_key", key.as_str()),
            ("corp_code", corp_code),
            ("bsns_year", last_year.to_string().as_str()),
            ("reprt_code", report_code),
            ("fs_div", fs_div),
        ])
        .send()
        .await?
        .json::<dart::StatementRes>()
        .await?;

    // status 013 means data NOT_FOUND
    if res.status == "013" {
        // try to get the report of the previous year
        res = req_base
            .query(&[
                ("crtfc_key", key.as_str()),
                ("corp_code", corp_code),
                ("bsns_year", (last_year - 1).to_string().as_str()),
                ("reprt_code", report_code),
                ("fs_div", fs_div),
            ])
            .send()
            .await?
            .json::<dart::StatementRes>()
            .await?;
    }

    Ok(res)
}
