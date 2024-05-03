use super::settings;
use actix_cors::Cors;

pub fn build() -> Cors {
    let mode = std::env::var("RUST_MODE").unwrap_or_else(|_| "development".into());
    if mode == "development" {
        Cors::permissive()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
    } else {
        let app_settings = settings::Settings::instance();
        let mut cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".junekim.xyz"))
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        if let Some(origins) = &app_settings.cors.origins {
            for origin in origins {
                cors = cors.allowed_origin(origin);
            }
        }
        if let Some(domains) = &app_settings.cors.allow_all_subdomains_of {
            for domain in domains {
                cors = cors.allowed_origin_fn(|origin, _req_head| {
                    origin.as_bytes().ends_with(domain.as_bytes())
                });
            }
        }
        cors
    }
}
