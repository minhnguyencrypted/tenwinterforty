use crate::database::schemas;
use axum::{extract::Path, Json};

pub async fn get_motorcycle(Path(id): Path<u64>) -> Json<schemas::Motorcycle> {}
