use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug)]
pub struct Error(String);

// impl Error {
//     fn from(err: impl std::fmt::Display) -> Self {
//         Error(err.to_string())
//     }
// }

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = Json(json!({ "error": self.0 }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub struct Response<T>(Result<T, Error>);

impl<T> Response<T> where T: IntoResponse {
    // pub fn 
}
