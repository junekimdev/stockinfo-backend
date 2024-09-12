#[derive(Debug, Clone, serde::Deserialize)]
pub struct ResBody {
    pub chart: Chart,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Chart {
    pub result: Vec<Result>,
    #[serde(flatten)]
    pub _error: serde_json::Value,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Result {
    pub timestamp: Vec<i64>,
    pub indicators: Indicators,
    #[serde(flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Indicators {
    pub quote: Vec<Quote>,
    #[serde(alias = "adjclose")]
    pub adj_close: Vec<AdjClose>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Quote {
    pub open: Vec<f32>,
    pub close: Vec<f32>,
    pub high: Vec<f32>,
    pub low: Vec<f32>,
    pub volume: Vec<u64>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AdjClose {
    #[serde(alias = "adjclose")]
    pub adj_close: Vec<f32>,
}
