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
pub async fn set<V, T>(k: &str, v: &V) -> Result<T>
where
    V: ToRedisArgs + std::marker::Sync + std::fmt::Debug,
    T: FromRedisValue,
{
    let mut conn = pool().get().await?;
    let res = conn.set_ex::<&str, &V, T>(k, v, 86400).await?; // 86400 = 24*60*60 : 1 day in sec
    Ok(res)
}

#[tracing::instrument(err)]
pub async fn get<T: FromRedisValue>(k: &str) -> Result<T> {
    let mut conn = pool().get().await?;
    let res = conn.get::<&str, T>(k).await?;
    Ok(res)
}
