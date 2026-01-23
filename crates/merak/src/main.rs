use std::net::Ipv4Addr;

use axum::http::StatusCode;
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[derive(ToSchema, Serialize)]
struct HelloResponse {
    message: String,
}

#[utoipa::path(method(get, head), path = "/hello", operation_id = "hello", responses(
    (status = 200, description = "Successful response", body = HelloResponse),
))]
async fn hello() -> axum::Json<HelloResponse> {
    axum::Json(HelloResponse {
        message: "Hello, World!".to_string(),
    })
}

async fn not_found() -> (StatusCode, axum::Json<HelloResponse>) {
    (
        StatusCode::NOT_FOUND,
        axum::Json(HelloResponse {
            message: "Not Found".to_string(),
        }),
    )
}

#[derive(OpenApi)]
#[openapi(paths(hello))]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(hello))
        .fallback(not_found)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8080)).await?;
    println!("Serving on http://127.0.0.1:8080...");
    axum::serve(listener, router).await?;
    Ok(())
}
