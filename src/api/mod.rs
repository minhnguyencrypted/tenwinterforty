use serde::Serialize;
use time::Date;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct ReplacedPart {
    name: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(ToSchema)]
pub struct Motorcycle {
    id: u64,
    make: String,
    model: String,
    year: u16,
    displacement: u64,
}

#[derive(ToSchema)]
pub struct MaintenanceActivity {
    id: u64,
    motorcycle_id: u64,
    #[schema(value_type = String)]
    date: Date,
    odometer_reading: u64,
    description: String,
    #[schema(value_type = Vec<ReplacedPart>, inline)]
    replaced_parts: Vec<ReplacedPart>,
}
