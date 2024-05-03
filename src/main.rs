mod api;
mod model;
mod services;
mod utils;

use tracing_subscriber::prelude::*;

#[actix_web::main]
#[tracing::instrument]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let app_name = std::env::var("CARGO_PKG_NAME").unwrap_or("app".to_string());
    let app_version = std::env::var("CARGO_PKG_VERSION").unwrap_or("0.0.0".to_string());

    // trace with jaeger
    let otel = utils::telemetry::OpenTelemetry::new(&app_name, &app_version);
    let env_filter = tracing_subscriber::filter::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();
    let subscriber = tracing_subscriber::registry()
        .with(otel.jaeger_layer)
        .with(tracing_subscriber::fmt::Layer::default())
        .with(env_filter);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.");

    let app_settings = utils::settings::Settings::instance();
    let server_addr = format!("{}:{}", app_settings.server.host, app_settings.server.port);

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(utils::cors::build())
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(api::build())
    })
    .bind(server_addr)?
    .run()
    .await
}
