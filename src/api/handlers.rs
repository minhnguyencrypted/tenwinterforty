use crate::database::{
    queries::AppDatabase,
    schemas::{MaintenanceRecord, Motorcycle},
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::Local;
use surrealdb::sql::Thing;

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

pub async fn get_maintenance_record(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db = AppDatabase::new();
    match db.get_maintenance_record(&id).await {
        Ok(log) => match log {
            Some(log_entry) => Ok((StatusCode::OK, Json(log_entry))),
            None => Err((
                StatusCode::NOT_FOUND,
                Json(format!("Maintenance log entry with id {} not found", id)),
            )),
        },
        Err(err) => {
            eprint!("{:#?}", err);
            Err((StatusCode::BAD_REQUEST, Json("Bad request".to_string())))
        }
    }
}

pub async fn create_maintenance_record(
    Json(mut payload): Json<MaintenanceRecord>,
) -> Result<(StatusCode, Json<Vec<Thing>>), (StatusCode, Json<String>)> {
    let db = AppDatabase::new();
    match payload.date {
        None => payload.date = Some(Local::now()),
        Some(_) => (),
    }
    match db.create_maintenance_record(payload).await {
        Ok(things) => Ok((StatusCode::CREATED, Json(things))),
        Err(err) => {
            eprint!("{:#?}", err);
            Err((StatusCode::BAD_REQUEST, Json("Bad request".to_string())))
        }
    }
}

pub async fn create_maintenance_record_by_mc_id(
    Path(id): Path<String>,
    Json(mut payload): Json<MaintenanceRecord>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db = AppDatabase::new();
    match db.get_motorcycle(&id).await {
        Ok(opt_mc) => match opt_mc {
            Some(mc) => {
                payload.motorcycle_id = mc.id;
                create_maintenance_record(Json(payload)).await
            }
            None => Err((
                StatusCode::NOT_FOUND,
                Json(format!("Motorcycle with id {} not found", id)),
            )),
        },
        Err(err) => {
            eprint!("{:#?}", err);
            Err((StatusCode::BAD_REQUEST, Json("Bad request".to_string())))
        }
    }
}

pub async fn get_maintenance_records_by_mc_id(
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Vec<MaintenanceRecord>>), (StatusCode, Json<String>)> {
    let db = AppDatabase::new();
    match db.get_motorcycle(&id).await {
        Ok(opt_mc) => match opt_mc {
            Some(_) => match db.get_maintenance_record_by_mc_id(&id).await {
                Ok(mtn_records) => Ok((StatusCode::OK, Json(mtn_records))),
                Err(err) => {
                    eprintln!("{:#?}", err);
                    Err((StatusCode::BAD_REQUEST, Json("Bad request".to_string())))
                }
            },
            None => Err((
                StatusCode::NOT_FOUND,
                Json(format!("Motorcycle with id {} not found", id)),
            )),
        },
        Err(err) => {
            eprintln!("{:#?}", err);
            Err((StatusCode::BAD_REQUEST, Json("Bad request".to_string())))
        }
    }
}
