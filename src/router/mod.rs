mod subscriptions;
use subscriptions::subscribe;

use axum::{
    extract::ConnectInfo,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http::{header::CONTENT_TYPE, request::Request};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use uuid::Uuid;

pub fn create_router(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let trace = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
        tracing::debug_span!(
            "request",
            method = ?request.method(),
            uri = %request.uri(),
            request_id = %Uuid::new_v4().to_string(),
            // version = ?request.version(),
        )
    });

    let middleware = ServiceBuilder::new().layer(trace).layer(cors);

    let assets = ServeDir::new("assets");

    Router::new()
        .route("/", get(health_check))
        .route("/subscribe", post(subscribe))
        .route("/json", post(json))
        .route("/connect-info", get(connect_info))
        .layer(middleware)
        .nest_service("/assets", assets)
        .with_state(pool)
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn json(payload: Json<serde_json::Value>) -> impl IntoResponse {
    Json(serde_json::json!({ "data": payload.0 }))
}

async fn connect_info(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    format!("Hi {addr}")
}
