use super::provider;
use crate::utils::{error::Error, Result};

#[tracing::instrument(err)]
#[actix_web::post("/prices/{short_code}")]
pub async fn handler_post(
    req: actix_web::HttpRequest,
    short_code: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if short_code.len() != 6 {
        return Err(Error::E400BadRequest("invalid short_code".into()));
    }

    provider::build_price_db(&short_code).await?;

    // Return result
    Ok(actix_web::HttpResponse::Created().finish())
}

#[tracing::instrument(err)]
#[actix_web::put("/prices/{short_code}")]
pub async fn handler_put(
    req: actix_web::HttpRequest,
    short_code: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if short_code.len() != 6 {
        return Err(Error::E400BadRequest("invalid short_code".into()));
    }

    provider::update_price_db(&short_code).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().finish())
}

#[tracing::instrument(err)]
#[actix_web::get("/prices/{short_code}/daily")]
pub async fn handler_get_daily(
    req: actix_web::HttpRequest,
    short_code: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if short_code.len() != 6 {
        return Err(Error::E400BadRequest("invalid short_code".into()));
    }

    let res = provider::get_price_daily(&short_code).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}

#[tracing::instrument(err)]
#[actix_web::get("/prices/{short_code}/weekly")]
pub async fn handler_get_weekly(
    req: actix_web::HttpRequest,
    short_code: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if short_code.len() != 6 {
        return Err(Error::E400BadRequest("invalid short_code".into()));
    }

    let res = provider::get_price_weekly(&short_code).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}

#[tracing::instrument(err)]
#[actix_web::get("/prices/{short_code}/exists")]
pub async fn handler_get_exists(
    req: actix_web::HttpRequest,
    short_code: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    if short_code.len() != 6 {
        return Err(Error::E400BadRequest("invalid short_code".into()));
    }

    let res = provider::get_price_exists(&short_code).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}

#[tracing::instrument(err)]
#[actix_web::delete("/prices")]
pub async fn handler_del(req: actix_web::HttpRequest) -> Result<actix_web::HttpResponse> {
    provider::clear_prices().await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().finish())
}
