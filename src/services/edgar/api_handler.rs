use super::provider;
use crate::model::edgar;
use crate::utils::{cache, error::Error, Result};

#[tracing::instrument(err)]
#[actix_web::get("/edgar/{cik}")]
pub async fn handler_get(
    req: actix_web::HttpRequest,
    cik: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    // Validate inputs
    if cik.is_empty() {
        return Err(Error::E400BadRequest("invalid CIK".into()));
    }

    // cache first
    if let Some(data_in_cache) = cache::get::<Option<edgar::StatementRes>>(req.path()).await? {
        return Ok(actix_web::HttpResponse::Ok().json(data_in_cache));
    };

    let res = provider::get_statement(&cik).await?;

    // Store in cache
    cache::set(req.path(), &res).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}
