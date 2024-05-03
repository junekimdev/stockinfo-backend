use super::provider;
use crate::model::{DartIndexRes, DartStatementRes};
use crate::utils::{cache, error::Error, Result};

#[tracing::instrument(err)]
#[actix_web::get("/dart/code/{name}")]
pub async fn handler_get_code(
    req: actix_web::HttpRequest,
    name: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    let decoded_name = urlencoding::decode(&name)?;

    let res = provider::get_dart_code(&decoded_name).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().body(res))
}

#[tracing::instrument(err)]
#[actix_web::post("/dart/code")]
pub async fn handler_post_code(req: actix_web::HttpRequest) -> Result<actix_web::HttpResponse> {
    provider::build_code_db().await?;

    // Return result
    Ok(actix_web::HttpResponse::Created().finish())
}

#[derive(Debug, serde::Deserialize)]
struct ParamsIndex {
    corp_code: String,
    report_code: String,
    idx_code: String,
}

#[tracing::instrument(err)]
#[actix_web::get("/dart/index/{corp_code}/{report_code}/{idx_code}")]
pub async fn handler_get_index(
    req: actix_web::HttpRequest,
    params: actix_web::web::Path<ParamsIndex>,
) -> Result<actix_web::HttpResponse> {
    // Validate inputs
    if params.corp_code.len() != 8 {
        return Err(Error::E400BadRequest("invalid corp_code".into()));
    }

    if params.report_code != "11011"
        && params.report_code != "11012"
        && params.report_code != "11013"
        && params.report_code != "11014"
    {
        return Err(Error::E400BadRequest("invalid report_code".into()));
    }

    if params.idx_code != "M210000"
        && params.idx_code != "M220000"
        && params.idx_code != "M230000"
        && params.idx_code != "M240000"
    {
        return Err(Error::E400BadRequest("invalid idx_code".into()));
    }

    // cache first
    if let Some(data_in_cache) = cache::get::<Option<DartIndexRes>>(req.path()).await? {
        return Ok(actix_web::HttpResponse::Ok().json(data_in_cache));
    };

    let res = provider::get_index(&params.corp_code, &params.report_code, &params.idx_code).await?;

    // Store in cache
    cache::set(req.path(), &res).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}

#[derive(Debug, serde::Deserialize)]
struct ParamsStatement {
    corp_code: String,
    report_code: String,
    fs_div: String,
}

#[tracing::instrument(err)]
#[actix_web::get("/dart/statement/{corp_code}/{report_code}/{fs_div}")]
pub async fn handler_get_statement(
    req: actix_web::HttpRequest,
    params: actix_web::web::Path<ParamsStatement>,
) -> Result<actix_web::HttpResponse> {
    // Validate inputs
    if params.corp_code.len() != 8 {
        return Err(Error::E400BadRequest("invalid corp_code".into()));
    }

    if params.report_code != "11011"
        && params.report_code != "11012"
        && params.report_code != "11013"
        && params.report_code != "11014"
    {
        return Err(Error::E400BadRequest("invalid report_code".into()));
    }

    if params.fs_div != "OFS" && params.fs_div != "CFS" {
        return Err(Error::E400BadRequest("invalid fs_div".into()));
    }

    // cache first
    if let Some(data_in_cache) = cache::get::<Option<DartStatementRes>>(req.path()).await? {
        return Ok(actix_web::HttpResponse::Ok().json(data_in_cache));
    };

    let res =
        provider::get_statement(&params.corp_code, &params.report_code, &params.fs_div).await?;

    // Store in cache
    cache::set(req.path(), &res).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}
