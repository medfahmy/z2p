use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug)]
pub struct Error {
    pub status: StatusCode,
    pub error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.status, self.error).into_response()
    }
}
