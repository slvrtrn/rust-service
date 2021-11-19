use std::str::FromStr;
use std::time::Duration;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Error, Pool, Postgres};

use crate::globals::CONFIG;
use std::sync::Arc;

pub async fn connect() -> Result<Arc<Pool<Postgres>>, Error> {
    let pg_connect_options = PgConnectOptions::from_str(&CONFIG.database_url)?;
    let pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(
            CONFIG.database_connection_timeout_seconds,
        ))
        .max_connections(CONFIG.database_max_connections)
        .connect_with(pg_connect_options)
        .await?;
    Ok(Arc::new(pool))
}
