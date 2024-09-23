use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Sub {
    name: String,
    email: String,
}

#[axum::debug_handler]
pub async fn sub(State(pool): State<PgPool>, Form(form): Form<Sub>) -> impl IntoResponse {
    sqlx::query!(
        "insert into subs (id, email, name, subscribed_at) values ($1, $2, $3, $4)",
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&pool)
    .await
    .unwrap();
}

// pub async fn subs() -> impl IntoResponse {
//     todo!()
// }
