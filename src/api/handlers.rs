use crate::database::{queries::AppDatabase, schemas::Motorcycle};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn get_motorcycle_by_id(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<String>)> {
    let db = AppDatabase::new();

    let response = db.get_motorcycle(&id).await;
    match response {
        Ok(record) => match record {
            Some(motorcycle) => Ok((StatusCode::OK, Json(motorcycle))),
            None => Err((
                StatusCode::NOT_FOUND,
                Json(format!("Motorcycle with id {} not found", id)),
            )),
        },
        Err(err) => {
            println!("{:#?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(String::from("Internal server error")),
            ))
        }
    }
}
