use crate::database::connect_db;
use crate::database::schemas;
use axum::{
    routing::{get, post, put},
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
        components(schemas(schemas::MaintenanceActivity, schemas::Motorcycle))
    )]
    struct ApiDoc;

    let _ = connect_db("localhost:8000", "root", "root", "test", "app").await;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/motorcycle/:id", get(api::handlers::get_motorcycle_by_id))
        .route("/motorcycle", post(api::handlers::create_motorcycle))
        .route("/motorcycle/:id", put(api::handlers::update_motorcycle));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
