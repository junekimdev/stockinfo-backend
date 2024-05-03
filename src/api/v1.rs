pub fn build() -> actix_web::Scope {
    actix_web::web::scope("/v1")
        .service(crate::services::prices::handler_post)
        .service(crate::services::prices::handler_put)
        .service(crate::services::prices::handler_get_daily)
        .service(crate::services::prices::handler_get_weekly)
        .service(crate::services::prices::handler_get_exists)
        .service(crate::services::companies::handler_post)
        .service(crate::services::companies::handler_get)
        .service(crate::services::dart::handler_get_code)
        .service(crate::services::dart::handler_post_code)
        .service(crate::services::dart::handler_get_index)
        .service(crate::services::dart::handler_get_statement)
}
