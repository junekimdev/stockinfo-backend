pub mod cache;
pub mod cors;
pub mod datetime;
pub mod db;
pub mod error;
pub mod hex;
pub mod settings;
pub mod telemetry;

use serde::Deserialize;

pub type Result<T> = std::result::Result<T, error::Error>;

#[allow(unused)]
#[tracing::instrument(skip_all)]
pub fn deserialize_from_bytes<'a, T>(data: &'a [u8]) -> Result<T>
where
    T: serde::Deserialize<'a>,
{
    let msg = std::str::from_utf8(data)?;
    Ok(serde_json::from_str::<T>(msg)?)
}

#[allow(unused)]
#[tracing::instrument(skip_all)]
pub fn serialize_to_bytes(object: &impl serde::Serialize) -> Result<std::sync::Arc<[u8]>> {
    let v = serde_json::to_string(object)?.as_bytes().to_owned();
    Ok(std::sync::Arc::from(v))
}

#[allow(unused)]
#[tracing::instrument(skip_all)]
pub fn deserialize<'a, T>(data: &'a str) -> Result<T>
where
    T: serde::Deserialize<'a>,
{
    Ok(serde_json::from_str::<T>(data)?)
}

#[tracing::instrument(skip_all)]
pub fn serialize(object: &impl serde::Serialize) -> Result<String> {
    Ok(serde_json::to_string(object)?)
}

pub fn from_str_deserialize<'de, T, D>(de: D) -> std::result::Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    String::deserialize(de)?
        .parse()
        .map_err(serde::de::Error::custom)
}
