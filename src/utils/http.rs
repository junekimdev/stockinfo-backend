use crate::utils::settings::Settings;

#[allow(unused)]
static CLIENT: std::sync::OnceLock<reqwest_middleware::ClientWithMiddleware> =
    std::sync::OnceLock::new();

#[tracing::instrument]
pub fn client() -> &'static reqwest_middleware::ClientWithMiddleware {
    CLIENT.get_or_init(|| {
        let agent = Settings::instance().agent.common.clone() + "/" + env!("CARGO_PKG_VERSION");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, agent.parse().unwrap());

        reqwest_middleware::ClientBuilder::new(
            reqwest::Client::builder()
                .cookie_store(true)
                .default_headers(headers)
                .build()
                .unwrap(),
        )
        .with(reqwest_tracing::TracingMiddleware::default())
        .build()
    })
}
