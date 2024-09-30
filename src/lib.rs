mod config;
mod db;
mod router;
mod extract;

pub use extract::{ApiError, ApiResult, Form, Json};
pub use config::{Config, CONFIG};

use db::init_db;
use router::create_router;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run() {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let addr = CONFIG.addr();
    let listener = TcpListener::bind(&addr).await.unwrap();
    let pool = init_db(&CONFIG, false).await;
    let router = create_router(pool);

    tracing::info!("server listening on '{}'", addr);
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

pub async fn spawn() -> (SocketAddr, PgPool) {
    let pool = init_db(&CONFIG, true).await;
    let router = create_router(pool.clone());

    let listener = TcpListener::bind(&format!("{}:0", CONFIG.server.host))
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, router.into_make_service())
            .await
            .unwrap()
    });

    (addr, pool)
}
