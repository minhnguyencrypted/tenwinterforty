use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub mod queries;
pub mod schemas;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn connect_db(
    address: &str,
    username: &str,
    password: &str,
    namespace: &str,
    database: &str,
) -> surrealdb::Result<()> {
    DB.connect::<Ws>(address).await?;
    DB.signin(Root {
        username: username,
        password: password,
    })
    .await?;
    DB.use_ns(namespace).use_db(database).await?;
    Ok(())
}
