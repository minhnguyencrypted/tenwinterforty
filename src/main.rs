use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod api;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "tenwinterforty - Motorcycle Maintenance Tracking System",
            description = "tenwinterforty is a Motorcycle Maintenace Tracking System (MMTS) used to track all maintenance activities of your motorcycles",
        ),
        components(schemas(api::MaintenanceActivity, api::Motorcycle))
    )]
    struct ApiDoc;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Hello, World!"
}
