use super::schemas::Motorcycle;
use super::DB;
use surrealdb::sql::Thing;
use surrealdb::Error;

pub struct AppDatabase {
    motorcycle_table: String,
}

impl AppDatabase {
    pub fn new() -> Self {
        AppDatabase {
            motorcycle_table: String::from("motorcycles"),
        }
    }

    pub async fn get_motorcycle(&self, id: &str) -> Result<Option<Motorcycle>, Error> {
        let response: Result<Option<Motorcycle>, surrealdb::Error> =
            DB.select((&self.motorcycle_table, String::from(id))).await;
        response
    }

    pub async fn create_motorcycle(&self, motorcycle: Motorcycle) -> Result<Vec<Thing>, Error> {
        let response: Result<Vec<Motorcycle>, surrealdb::Error> =
            DB.create(&self.motorcycle_table).content(motorcycle).await;
        match response {
            Ok(mcs) => {
                let mc_things: Vec<Thing> =
                    mcs.iter().map(|mc| mc.id.to_owned().unwrap()).collect();
                Ok(mc_things)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn update_motorcycle(
        &self,
        id: &str,
        motorcycle: Motorcycle,
    ) -> Result<Option<Thing>, Error> {
        let response: Result<Option<Motorcycle>, Error> = DB
            .update((&self.motorcycle_table, id))
            .content(motorcycle)
            .await;
        match response {
            Ok(mc) => match mc {
                Some(mc) => Ok(Some(mc.id.unwrap())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }
}
