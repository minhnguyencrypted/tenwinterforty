use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize)]
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

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct MaintenanceRecord {
    pub id: Option<Thing>,
    pub motorcycle_id: Option<Thing>,
    #[schema(value_type = String)]
    pub date: Option<DateTime<Local>>,
    pub odometer_reading: Option<u64>,
    pub description: Option<String>,
    #[schema(value_type = Vec<ReplacedPart>, inline)]
    pub replaced_parts: Option<Vec<ReplacedPart>>,
}
