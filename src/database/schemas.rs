use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use time::Date;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct ReplacedPart {
    name: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct Motorcycle {
    #[schema(value_type = String)]
    pub id: Option<Thing>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<u64>,
    pub displacement: Option<u64>,
    pub license_plate: Option<String>,
}

#[derive(ToSchema, Serialize)]
pub struct MaintenanceActivity {
    id: Option<Thing>,
    motorcycle_id: Option<Thing>,
    #[schema(value_type = String)]
    date: Date,
    odometer_reading: u64,
    description: String,
    #[schema(value_type = Vec<ReplacedPart>, inline)]
    replaced_parts: Vec<ReplacedPart>,
}
