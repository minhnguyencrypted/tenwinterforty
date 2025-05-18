use diesel::prelude::*;
use diesel_async::pooled_connection::{bb8::*, AsyncDieselConnectionManager, PoolError};
use diesel_async::AsyncPgConnection;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    let pg_conn_string = env::var("PG_CONN_STRING").expect("PG_CONN_STRING must be set");
    PgConnection::establish(&pg_conn_string)
        .unwrap_or_else(|_| panic!("Error connecting to {}", pg_conn_string))
}

pub async fn create_connection_pool(
    database_url: &str,
) -> Result<Pool<AsyncPgConnection>, PoolError> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder().build(config).await
}
