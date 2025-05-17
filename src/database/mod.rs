use diesel::prelude::*;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    let pg_conn_string = env::var("PG_CONN_STRING").expect("PG_CONN_STRING must be set");
    PgConnection::establish(&pg_conn_string)
        .unwrap_or_else(|_| panic!("Error connecting to {}", pg_conn_string))
}
