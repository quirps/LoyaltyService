use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(database_url: &Secret<String>) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(database_url.expose_secret())?;
    tracing::info!("Database connection pool created!");

    Ok(pool)
}
