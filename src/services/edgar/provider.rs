use crate::model::{edgar, xbrl};
use crate::utils::{settings::Settings, Result};

pub async fn get_statement(cik: &str) -> Result<edgar::StatementRes> {
    let client = reqwest::Client::new();
    let agent = Settings::instance().agent.sec_gov.clone();
    let urls = Settings::instance().urls.clone();
    let submission_url = format!("{}{}.json", urls.us_submissions, cik);
    let req_url = reqwest::Url::parse(&submission_url).unwrap();
    let host = req_url.host_str().unwrap();

    // Get a list of edgar reports from the internet
    let res = client
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, &agent)
        .header(reqwest::header::ACCEPT, "application/json;charset=UTF-8")
        .send()
        .await?
        .json::<edgar::Submissions>()
        .await?;

    // Find index of the latest annual report
    let mut index = 0;
    for (i, v) in res.filings.recent.form.iter().enumerate() {
        if v == "10-K" {
            // "10-K" means Annual Report
            index = i;
            break;
        }
    }

    // Get the edgar report from the internet
    let url = format!(
        "{}/{}/{}/{}",
        urls.edgar,
        &res.cik, // shorter cik without zero paddings
        res.filings.recent.accession_number[index].replace('-', ""),
        res.filings.recent.primary_document[index].replace(".htm", "_htm.xml")
    );
    let req_url = reqwest::Url::parse(&url).unwrap();
    let host = req_url.host_str().unwrap();

    let res = client
        .get(req_url.clone())
        .header(reqwest::header::HOST, host)
        .header(reqwest::header::USER_AGENT, &agent)
        .header(reqwest::header::ACCEPT, "application/xml;charset=UTF-8")
        .send()
        .await?
        .text()
        .await?;

    // Parse report to extract data
    let doc = roxmltree::Document::parse(&res)?;
    let mut outstanding_stock = extract(&doc, "CommonStockSharesOutstanding");
    if outstanding_stock.is_empty() {
        outstanding_stock = xbrl::Group::extract(&doc, "CommonStockSharesOutstanding", false)
            .to_vec_date_and_value_with_segment("us-gaap:CommonClassAMember")
            .into_iter()
            .map(edgar::StatementItem::from)
            .collect::<Vec<edgar::StatementItem>>();
    }

    let assets = extract(&doc, "Assets");
    let equity = extract(&doc, "StockholdersEquity");
    let liabilities = extract(&doc, "Liabilities");

    let mut revenue = xbrl::Group::extract(&doc, "Revenues", true)
        .to_vec_date_and_value()
        .into_iter()
        .map(edgar::StatementItem::from)
        .collect::<Vec<edgar::StatementItem>>();
    if revenue.is_empty() {
        revenue = xbrl::Group::extract(&doc, "Revenue", false) // Notice: Revenues vs Revenue
            .to_vec_date_and_value()
            .into_iter()
            .map(edgar::StatementItem::from)
            .collect::<Vec<edgar::StatementItem>>();
    }

    let operating_income = extract(&doc, "OperatingIncomeLoss");
    let net_income = extract(&doc, "NetIncomeLoss");
    let comprehensive_income = extract(&doc, "ComprehensiveIncomeNetOfTax");
    let operating_cash_flow = extract(&doc, "NetCashProvidedByUsedInOperatingActivities");
    let investing_cash_flow = extract(&doc, "NetCashProvidedByUsedInInvestingActivities");
    let financing_cash_flow = extract(&doc, "NetCashProvidedByUsedInFinancingActivities");

    // Return statement
    Ok(edgar::StatementRes {
        cik: cik.to_string(),
        outstanding_stock,
        assets,
        equity,
        liabilities,
        revenue,
        operating_income,
        net_income,
        comprehensive_income,
        operating_cash_flow,
        investing_cash_flow,
        financing_cash_flow,
    })
}

fn extract(doc: &roxmltree::Document, tag: &str) -> Vec<edgar::StatementItem> {
    let mut result = xbrl::Group::extract(doc, tag, true)
        .to_vec_date_and_value()
        .into_iter()
        .map(edgar::StatementItem::from)
        .collect::<Vec<edgar::StatementItem>>();
    if result.is_empty() {
        result = xbrl::Group::extract(doc, tag, false)
            .to_vec_date_and_value()
            .into_iter()
            .map(edgar::StatementItem::from)
            .collect::<Vec<edgar::StatementItem>>();
    }
    result
}
