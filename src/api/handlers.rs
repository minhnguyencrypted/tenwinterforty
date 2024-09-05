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
            Some(mc) => Ok((StatusCode::OK, Json(mc))),
            None => Err((
                StatusCode::NOT_FOUND,
                Json(format!("Motorcycle with id {} not found", id)),
            )),
        },
        Err(err) => {
            println!("{:#?}", err);
            Err((StatusCode::BAD_REQUEST, Json(String::from("Bad request"))))
        }
    }
}

pub async fn create_motorcycle(
    Json(payload): Json<Motorcycle>,
) -> Result<impl IntoResponse, (StatusCode, Json<String>)> {
    let db = AppDatabase::new();
    println!("{:#?}", &payload);
    let response = db.create_motorcycle(payload).await;
    match response {
        Ok(mc_things) => {
            let mc_ids: Vec<String> = mc_things
                .iter()
                .map(|item| item.id.to_owned().to_string())
                .collect();
            Ok((StatusCode::CREATED, Json(mc_ids)))
        }
        Err(err) => {
            println!("{:#?}", err);
            Err((StatusCode::BAD_REQUEST, Json(String::from("Bad request"))))
        }
    }
}

pub async fn update_motorcycle(
    Path(id): Path<String>,
    Json(payload): Json<Motorcycle>,
) -> Result<impl IntoResponse, (StatusCode, Json<String>)> {
    let db = AppDatabase::new();
    match db.get_motorcycle(&id).await {
        Ok(mc) => match mc {
            Some(_) => match db.update_motorcycle(&id, payload).await {
                Ok(response) => Ok((StatusCode::OK, response.unwrap().id.to_owned().to_string())),
                Err(err) => {
                    println!("{:#?}", err);
                    Err((StatusCode::BAD_REQUEST, Json(String::from("Bad request"))))
                }
            },
            None => Err((
                StatusCode::NOT_FOUND,
                Json(format!("Motorcycle with id {} not found", &id)),
            )),
        },
        Err(err) => {
            println!("{:#?}", err);
            Err((StatusCode::BAD_REQUEST, Json(String::from("Bad request"))))
        }
    }
}

pub async fn delete_motorcycle(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db = AppDatabase::new();
    match db.delete_motorcycle(&id).await {
        Ok(mc_thing) => match mc_thing {
            Some(_) => Ok(StatusCode::NO_CONTENT),
            None => Err((
                StatusCode::NOT_FOUND,
                Json(format!("Motorcycle with id {} not found", &id)),
            )),
        },
        Err(err) => {
            println!("{:#?}", err);
            Err((StatusCode::BAD_REQUEST, Json(String::from("Bad request"))))
        }
    }
}
