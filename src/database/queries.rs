use super::schemas::{MaintenanceLog, Motorcycle};
use super::DB;
use surrealdb::sql::Thing;
use surrealdb::Error;

pub struct AppDatabase {
    motorcycle_table: String,
    maintenance_log: String,
}

impl AppDatabase {
    pub fn new() -> Self {
        AppDatabase {
            motorcycle_table: String::from("motorcycles"),
            maintenance_log: String::from("maintenance"),
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

    pub async fn delete_motorcycle(&self, id: &str) -> Result<Option<Thing>, Error> {
        let response: Result<Option<Motorcycle>, Error> =
            DB.delete((&self.motorcycle_table, id)).await;
        match response {
            Ok(mc) => match mc {
                Some(mc) => Ok(mc.id),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn create_maintenance_log(
        &self,
        maintenance_log: MaintenanceLog,
    ) -> Result<Vec<Thing>, Error> {
        let response: Result<Vec<MaintenanceLog>, Error> = DB
            .create(&self.maintenance_log)
            .content(maintenance_log)
            .await;
        match response {
            Ok(logs) => {
                let log_things: Vec<Thing> =
                    logs.iter().map(|log| log.id.to_owned().unwrap()).collect();
                Ok(log_things)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get_maintenance_log(&self, id: &str) -> Result<Option<MaintenanceLog>, Error> {
        let response: Result<Option<MaintenanceLog>, Error> =
            DB.select((&self.maintenance_log, id)).await;
        match response {
            Ok(opt_log) => Ok(opt_log),
            Err(err) => Err(err),
        }
    }
}
