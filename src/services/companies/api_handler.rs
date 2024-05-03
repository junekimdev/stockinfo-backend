use super::provider;
use crate::utils::Result;

#[tracing::instrument(err)]
#[actix_web::post("/companies")]
pub async fn handler_post(req: actix_web::HttpRequest) -> Result<actix_web::HttpResponse> {
    provider::build_company_db().await?;

    // Return result
    Ok(actix_web::HttpResponse::Created().finish())
}

#[tracing::instrument]
#[actix_web::get("/companies/{search_word}")]
pub async fn handler_get(
    req: actix_web::HttpRequest,
    search_word: actix_web::web::Path<String>,
) -> Result<actix_web::HttpResponse> {
    let decoded_search_word = urlencoding::decode(&search_word)?;

    let res = provider::get_company(&decoded_search_word).await?;

    // Return result
    Ok(actix_web::HttpResponse::Ok().json(res))
}
