pub mod config;
mod routes;

use axum::{
    extract::ConnectInfo,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
pub use config::Config;
// use http::header::CONTENT_TYPE;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    // cors::{Any, CorsLayer},
    // services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use sqlx::{Connection, PgConnection, Executor};

pub async fn run() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::get();
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    let pool = PgPool::connect(&config.db_url).await.unwrap();
    let router = create_router(pool);

    tracing::info!("server listening on {}:{}", &config.host, &config.port);
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

pub async fn spawn() -> (SocketAddr, PgPool) {
    let pg_url = "postgres://postgres:postgres@localhost:5432";
    let db_name = Uuid::new_v4().to_string();
    let db_url = format!("{}/{}", pg_url, db_name);

    let mut connection = PgConnection::connect(pg_url).await.unwrap();

    connection
        .execute(format!("create database \"{}\";", db_name).as_str())
        .await
        .unwrap();

    let pool = PgPool::connect(&db_url).await.unwrap();
    let router = create_router(pool.clone());

    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, router.into_make_service())
            .await
            .unwrap()
    });

    (addr, pool)
}

fn create_router(pool: PgPool) -> Router {
    // let cors = CorsLayer::new()
    //     .allow_origin(Any)
    //     .allow_headers([CONTENT_TYPE]);
    // let assets = ServeDir::new("assets");

    Router::new()
        .route("/", get(health_check))
        .route("/sub", post(routes::sub))
        .route("/json", post(json))
        .route("/connect-info", get(connect_info))
        // .nest_service("/assets", assets)
        // .layer(cors)
        .layer(TraceLayer::new_for_http())
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
