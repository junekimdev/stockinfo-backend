use crate::model::{DartCode, DartIndexRes, DartStatementRes};
use crate::utils::{
    datetime::parse_date_from,
    db,
    error::Error,
    settings::{Dart, Settings},
    Result,
};
use std::io::Read;

#[tracing::instrument(err)]
pub async fn build_code_db() -> Result<()> {
    let Dart { key, url } = Settings::instance().dart.clone();

    // Get a file from the internet
    let res = tokio::task::spawn_blocking(move || {
        let req_url = reqwest::Url::parse(&url.code).unwrap();
        let host = req_url.host_str().unwrap();
        let mut buf: Vec<u8> = Vec::new();
        reqwest::blocking::Client::new()
            .get(req_url.clone())
            .header(reqwest::header::HOST, host)
            .header(reqwest::header::USER_AGENT, "StockinfoRuntime/1.0.0")
            .header(reqwest::header::ACCEPT, "application/json;charset=UTF-8")
            .query(&[("crtfc_key", key.as_str())])
            .send()
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        buf
    })
    .await?;

    // Extract data from the file
    let res_reader = std::io::Cursor::new(res);
    let mut xml_file: Vec<u8> = Vec::new();
    zip::read::ZipArchive::new(res_reader)?
        .by_name("CORPCODE.xml")?
        .read_to_end(&mut xml_file)?;
    let xml_reader = std::io::Cursor::new(xml_file);
    let root = xmltree::Element::parse(xml_reader)?;

    let mut codes: Vec<DartCode> = Vec::new();
    for child in root.children {
        let code = extract_code(&child).ok_or("Failed to extract code out of XML Node")?;
        codes.push(code);
    }

    // Store data in DB
    const SQL_CLEAR: &str = "TRUNCATE TABLE dart_code RESTART IDENTITY";
    const SQL_INSERT: &str =
        "INSERT INTO dart_code(code,name,date) VALUES ($1::CHAR(8), $2::TEXT, $3::DATE);";

    let mut db_client = db::pool().get().await?;
    let transaction = db_client.transaction().await?;
    let sql_insert = transaction.prepare(SQL_INSERT).await?;

    transaction.simple_query(SQL_CLEAR).await?; // Reset the table

    for item in &codes {
        transaction
            .query(
                &sql_insert,
                &[&item.corp_code, &item.corp_name, &item.modify_date],
            )
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}

#[tracing::instrument(err)]
pub async fn get_dart_code(name: &str) -> Result<String> {
    const SQL: &str = "SELECT * FROM dart_code WHERE name=$1::TEXT ORDER BY date DESC;";

    let rows = db::query(SQL, &[&name]).await?;
    if rows.is_empty() {
        return Err(Error::E404NotFound("dart code".into()));
    }

    let res = rows[0].get("code");

    Ok(res)
}

pub async fn get_index(corp_code: &str, report_code: &str, idx_code: &str) -> Result<DartIndexRes> {
    let Dart { key, url } = Settings::instance().dart.clone();
    let last_year = time::OffsetDateTime::now_utc().year() - 1;
    let req_url = reqwest::Url::parse(&url.index).unwrap();
    let host = req_url.host_str().unwrap();

    let req_base = reqwest::Client::new()
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, "StockinfoRuntime/1.0.0")
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
        .json::<DartIndexRes>()
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
            .json::<DartIndexRes>()
            .await?;
    }

    Ok(res)
}

pub async fn get_statement(
    corp_code: &str,
    report_code: &str,
    fs_div: &str,
) -> Result<DartStatementRes> {
    let Dart { key, url } = Settings::instance().dart.clone();
    let last_year = time::OffsetDateTime::now_utc().year() - 1;
    let req_url = reqwest::Url::parse(&url.statement).unwrap();
    let host = req_url.host_str().unwrap();

    let req_base = reqwest::Client::new()
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, "StockinfoRuntime/1.0.0")
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
        .json::<DartStatementRes>()
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
            .json::<DartStatementRes>()
            .await?;
    }

    Ok(res)
}

fn extract_code(node: &xmltree::XMLNode) -> Option<DartCode> {
    let el = node.as_element()?;
    let corp_code = el.get_child("corp_code")?.get_text()?.to_string();
    let corp_name = el.get_child("corp_name")?.get_text()?.to_string();
    let modify_date = el.get_child("modify_date")?.get_text()?.to_string();
    Some(DartCode {
        corp_code,
        corp_name,
        modify_date: parse_date_from(&modify_date).unwrap(),
    })
}
