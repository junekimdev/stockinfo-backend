use super::{settings, Result};
use deadpool_postgres::Pool;
use tokio_postgres::{types::ToSql, Row};

#[allow(unused)]
static POOL: std::sync::OnceLock<Pool> = std::sync::OnceLock::new();

#[tracing::instrument]
pub fn pool() -> &'static Pool {
    POOL.get_or_init(|| {
        let app_settings = settings::Settings::instance();
        app_settings
            .pg
            .create_pool(
                Some(deadpool_postgres::Runtime::Tokio1),
                tokio_postgres::NoTls,
            )
            .unwrap()
    })
}

#[tracing::instrument(err)]
pub async fn query(sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>> {
    let client = pool().get().await?;
    let prepped_sql = client.prepare(sql).await?;
    let rows = client.query(&prepped_sql, params).await?;
    Ok(rows)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn works() {
        let client = pool().get().await.unwrap();
        assert!(!client.is_closed())
    }
}
