//! Uses `axum::extract::FromRequest` to wrap another extractor and customize the
//! rejection
//!
//! + Easy learning curve: Deriving `FromRequest` generates a `FromRequest`
//!   implementation for your type using another extractor. You only need
//!   to provide a `From` impl between the original rejection type and the
//!   target rejection. Crates like [`thiserror`] can provide such conversion
//!   using derive macros.
//! - Boilerplate: Requires deriving `FromRequest` for every custom rejection
//! - There are some known limitations: [FromRequest#known-limitations]
//!
//! [`thiserror`]: https://crates.io/crates/thiserror
//! [FromRequest#known-limitations]: https://docs.rs/axum-macros/*/axum_macros/derive.FromRequest.html#known-limitations

use axum::{
    extract::rejection::{JsonRejection, FormRejection}, extract::FromRequest, http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
// use serde_json::{json, Value};

// create an extractor that internally uses `axum::Json` but has a custom rejection
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

#[derive(FromRequest)]
#[from_request(via(axum::Form), rejection(ApiError))]
pub struct Form<T>(pub T);

// We implement `IntoResponse` for our extractor so it can be used as a response
impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

impl<T: Serialize> IntoResponse for Form<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Form(value).into_response()
    }
}

// We create our own rejection type
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub error: String,
}

// We implement `From<JsonRejection> for ApiError`
impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            error: rejection.body_text(),
        }
    }
}

impl From<FormRejection> for ApiError {
    fn from(rejection: FormRejection) -> Self {
        Self {
            status: rejection.status(),
            error: rejection.body_text(),
        }
    }
}

// We implement `IntoResponse` so `ApiError` can be used as a response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = serde_json::json!({
            "error": self.error,
        });

        (self.status, axum::Json(payload)).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
