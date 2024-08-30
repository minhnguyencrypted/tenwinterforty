use super::schemas::Motorcycle;
use super::DB;

pub struct AppDatabase {
    motorcycle_table: String,
}

impl AppDatabase {
    pub fn new() -> Self {
        AppDatabase {
            motorcycle_table: String::from("motorcycles"),
        }
    }

    pub async fn get_motorcycle(&self, id: &str) -> Result<Option<Motorcycle>, surrealdb::Error> {
        let response: Result<Option<Motorcycle>, surrealdb::Error> =
            DB.select((&self.motorcycle_table, String::from(id))).await;
        response
    }
}
