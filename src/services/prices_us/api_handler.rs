use super::provider;
use crate::utils::{error::Error, Result};

#[tracing::instrument(err)]
#[actix_web::post("/prices_us/{ticker}")]
pub async fn handler_post(
    req: actix_web::HttpRequest,
    ticker: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if ticker.is_empty() {
        return Err(Error::E400BadRequest("invalid ticker".into()));
    }

    provider::build_price_db(&ticker).await?;

    // Return result
    Ok(actix_web::HttpResponse::Created().finish())
}

#[tracing::instrument(err)]
#[actix_web::put("/prices_us/{ticker}")]
pub async fn handler_put(
    req: actix_web::HttpRequest,
    ticker: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if ticker.is_empty() {
        return Err(Error::E400BadRequest("invalid ticker".into()));
    }

    provider::update_price_db(&ticker).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().finish())
}

#[tracing::instrument(err)]
#[actix_web::get("/prices_us/{ticker}/daily")]
pub async fn handler_get_daily(
    req: actix_web::HttpRequest,
    ticker: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if ticker.is_empty() {
        return Err(Error::E400BadRequest("invalid ticker".into()));
    }

    let res = provider::get_price_daily(&ticker).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}

#[tracing::instrument(err)]
#[actix_web::get("/prices_us/{ticker}/weekly")]
pub async fn handler_get_weekly(
    req: actix_web::HttpRequest,
    ticker: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if ticker.is_empty() {
        return Err(Error::E400BadRequest("invalid ticker".into()));
    }

    let res = provider::get_price_weekly(&ticker).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}

#[tracing::instrument(err)]
#[actix_web::get("/prices_us/{ticker}/exists")]
pub async fn handler_get_exists(
    req: actix_web::HttpRequest,
    ticker: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if ticker.is_empty() {
        return Err(Error::E400BadRequest("invalid ticker".into()));
    }

    let res = provider::get_price_exists(&ticker).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}

#[tracing::instrument(err)]
#[actix_web::delete("/prices_us")]
pub async fn handler_del(req: actix_web::HttpRequest) -> Result<actix_web::HttpResponse> {
    provider::clear_prices().await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().finish())
}
