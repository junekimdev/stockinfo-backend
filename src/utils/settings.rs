use super::Result;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Jaeger {
    pub agent_endpoint: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cors {
    pub origins: Option<Vec<String>>,
    pub allow_all_subdomains_of: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovdataUrl {
    pub price: String,
    pub company: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DartUrl {
    pub statement: String,
    pub index: String,
    pub code: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsStockUrl {
    pub ticker: String,
    pub submissions: String,
    pub edgar: String,
    pub price: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Keys {
    pub data_go_kr: String,
    pub dart: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Urls {
    pub kr_price: String,
    pub kr_company: String,
    pub us_ticker: String,
    pub us_submissions: String,
    pub us_price: String,
    pub dart_statement: String,
    pub dart_index: String,
    pub dart_code: String,
    pub edgar: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub server: Server,
    pub jaeger: Jaeger,
    pub cors: Cors,
    pub pg: deadpool_postgres::Config,
    pub redis: deadpool_redis::Config,
    pub keys: Keys,
    pub urls: Urls,
}

static SETTINGS: std::sync::OnceLock<Settings> = std::sync::OnceLock::new();

impl Settings {
    #[tracing::instrument]
    pub fn new() -> Result<Self> {
        dotenv::dotenv().ok();
        let mode = std::env::var("RUST_MODE").unwrap_or_else(|_| "default".into());
        let config_filename = format!("config/{}", mode.to_lowercase());
        let s = config::Config::builder()
            .add_source(config::File::with_name(&config_filename).required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        println!("{:?}", s);
        Ok(s.try_deserialize()?)
    }

    #[tracing::instrument]
    pub fn instance() -> &'static Self {
        SETTINGS.get_or_init(|| Self::new().unwrap())
    }
}
