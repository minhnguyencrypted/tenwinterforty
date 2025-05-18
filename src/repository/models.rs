use chrono::{DateTime, Utc};
use diesel::prelude::*;

use super::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = app::motorcycles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Motorcycle {
    pub id: i32,
    pub make: String,
    pub model: String,
    pub year: Option<i32>,
    pub license_plate: String,
    pub displacement: Option<i32>,
    pub created_at: DateTime<Utc>,
}
