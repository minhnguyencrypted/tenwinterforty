use super::{
    schemas::{MaintenanceRecord, Motorcycle},
    DB,
};
use surrealdb::sql::Id;
use surrealdb::{sql::Thing, Error};

pub struct AppDatabase {
    mc_table: String,
    mtn_table: String,
}

impl AppDatabase {
    pub fn new() -> Self {
        AppDatabase {
            mc_table: String::from("motorcycles"),
            mtn_table: String::from("maintenance"),
        }
    }

    pub async fn get_motorcycle(&self, id: &str) -> Result<Option<Motorcycle>, Error> {
        let response: Result<Option<Motorcycle>, surrealdb::Error> =
            DB.select((&self.mc_table, String::from(id))).await;
        response
    }

    pub async fn create_motorcycle(&self, motorcycle: Motorcycle) -> Result<Vec<Thing>, Error> {
        let response: Result<Vec<Motorcycle>, surrealdb::Error> =
            DB.create(&self.mc_table).content(motorcycle).await;
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
        let response: Result<Option<Motorcycle>, Error> =
            DB.update((&self.mc_table, id)).content(motorcycle).await;
        match response {
            Ok(mc) => match mc {
                Some(mc) => Ok(Some(mc.id.unwrap())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn delete_motorcycle(&self, id: &str) -> Result<Option<Thing>, Error> {
        let response: Result<Option<Motorcycle>, Error> = DB.delete((&self.mc_table, id)).await;
        match response {
            Ok(mc) => match mc {
                Some(mc) => Ok(mc.id),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn create_maintenance_record(
        &self,
        maintenance_log: MaintenanceRecord,
    ) -> Result<Vec<Thing>, Error> {
        let response: Result<Vec<MaintenanceRecord>, Error> =
            DB.create(&self.mtn_table).content(maintenance_log).await;
        match response {
            Ok(logs) => {
                let log_things: Vec<Thing> =
                    logs.iter().map(|log| log.id.to_owned().unwrap()).collect();
                Ok(log_things)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get_maintenance_record(
        &self,
        id: &str,
    ) -> Result<Option<MaintenanceRecord>, Error> {
        let response: Result<Option<MaintenanceRecord>, Error> =
            DB.select((&self.mtn_table, id)).await;
        match response {
            Ok(opt_log) => Ok(opt_log),
            Err(err) => Err(err),
        }
    }

    pub async fn get_maintenance_record_by_mc_id(
        &self,
        mc_id: &str,
    ) -> Result<Vec<MaintenanceRecord>, Error> {
        let mc_thing = Thing {
            tb: self.mc_table.clone(),
            id: Id::String(mc_id.to_string()),
        };
        let mut result = DB
            .query("select * from type::table($mtn_table) where motorcycle_id = $mc_thing")
            .bind(("mtn_table", &self.mtn_table))
            .bind(("mc_thing", mc_thing))
            .await;
        match result {
            Ok(mut resp) => match resp.take(0) {
                Ok(records) => {
                    let mtn_records: Vec<MaintenanceRecord> = records;
                    Ok(mtn_records)
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}
