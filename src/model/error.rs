#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorRes {
    pub code: u16,
    pub message: String,
}
