use crate::{ApiError, ApiResult, Form};
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub struct Subscription {
    id: Uuid,
    name: String,
    email: String,
    subscribed_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        // request_id = %Uuid::new_v4().to_string(),
        email = %form.email,
        name = %form.name
    )
)]
pub async fn subscribe(State(pool): State<PgPool>, Form(form): Form<SubscriptionForm>,
) -> ApiResult<(StatusCode, Json<Subscription>)> {
    let subscription = Subscription {
        id: Uuid::new_v4(),
        email: form.email,
        name: form.name,
        subscribed_at: Utc::now(),
    };

    let subscription = sqlx::query_as!(
        Subscription,
        r#"
            insert into subscriptions 
            (id, email, name, subscribed_at) 
            values ($1, $2, $3, $4)
            returning *
        "#,
        subscription.id,
        subscription.email,
        subscription.name,
        subscription.subscribed_at,
    )
    .fetch_one(&pool)
    .await;

    match subscription {
        Ok(subscription) => {
            tracing::info!("Subscriber with email '{}' created", subscription.email);
            Ok((StatusCode::CREATED, Json(subscription)))
        }
        Err(err) => {
            tracing::error!("{}", err);
            Err(ApiError {
                status: StatusCode::CONFLICT,
                error: err.to_string(),
            })
        }
    }
}
