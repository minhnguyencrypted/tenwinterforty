use crate::database::{
    connect_db,
    schemas::{MaintenanceRecord, Motorcycle},
};
use api::handlers;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod database;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "tenwinterforty - Motorcycle Maintenance Tracking System",
            description = "tenwinterforty is a Motorcycle Maintenace Tracking System (MMTS) used to track all maintenance activities of your motorcycles",
        ),
        components(schemas(MaintenanceRecord, Motorcycle))
    )]
    struct ApiDoc;

    let _ = connect_db("localhost:8000", "root", "root", "test", "app").await;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/motorcycle/:id", get(handlers::get_motorcycle_by_id))
        .route("/motorcycle", post(handlers::create_motorcycle))
        .route("/motorcycle/:id", put(handlers::update_motorcycle))
        .route("/motorcycle/:id", delete(handlers::delete_motorcycle))
        .route("/maintenance/:id", get(handlers::get_maintenance_record))
        .route("/maintenance", post(handlers::create_maintenance_record))
        .route(
            "/motorcycle/:id/maintenance",
            post(handlers::create_maintenance_record_by_mc_id),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
