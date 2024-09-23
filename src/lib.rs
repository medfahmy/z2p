use axum::{
    extract::ConnectInfo,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http::header::CONTENT_TYPE;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = create_router();
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}

pub async fn spawn() -> SocketAddr {
    let router = create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, router.into_make_service())
            .await
            .unwrap()
    });

    addr
}

fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);
    let assets = ServeDir::new("assets");

    Router::new()
        .route("/", get(health_check))
        .route("/subscribe", post(health_check))
        .route("/json", post(json))
        .route("/connect-info", get(connect_info))
        // .nest_service("/assets", assets)
        // .layer(cors)
        // We can still add middleware
        .layer(TraceLayer::new_for_http())
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
