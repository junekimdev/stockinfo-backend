use super::{settings, Result};
use deadpool_redis::{
    redis::{AsyncCommands, FromRedisValue, ToRedisArgs},
    Pool,
};

#[allow(unused)]
static POOL: std::sync::OnceLock<Pool> = std::sync::OnceLock::new();

#[tracing::instrument]
pub fn pool() -> &'static Pool {
    POOL.get_or_init(|| {
        let app_settings = settings::Settings::instance();
        app_settings
            .redis
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .unwrap()
    })
}

#[tracing::instrument(err)]
pub async fn set<V>(k: &str, v: &V) -> Result<()>
where
    V: ToRedisArgs + std::marker::Sync + std::fmt::Debug,
{
    let key = get_key_prefixed(k);
    let mut conn = pool().get().await?;
    conn.set_ex::<&str, &V, ()>(&key, v, 86400).await?; // 86400 = 24*60*60 : 1 day in sec
    Ok(())
}

#[allow(unused)]
#[tracing::instrument(err)]
pub async fn set_with_timer<V>(k: &str, v: &V, seconds: u64) -> Result<()>
where
    V: ToRedisArgs + std::marker::Sync + std::fmt::Debug,
{
    let key = get_key_prefixed(k);
    let mut conn = pool().get().await?;
    conn.set_ex::<&str, &V, ()>(&key, v, seconds).await?;
    Ok(())
}

#[tracing::instrument(err)]
pub async fn get<T: FromRedisValue>(k: &str) -> Result<T> {
    let key = get_key_prefixed(k);
    let mut conn = pool().get().await?;
    let res = conn.get::<&str, T>(&key).await?;
    Ok(res)
}

fn get_key_prefixed(k: &str) -> String {
    let app_name = std::env::var("CARGO_PKG_NAME").unwrap_or("app".to_string());
    let app_version = std::env::var("CARGO_PKG_VERSION").unwrap_or("0.0.0".to_string());
    app_name + "_" + &app_version + ":" + k
}
