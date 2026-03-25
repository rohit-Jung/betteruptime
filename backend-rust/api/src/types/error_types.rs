use poem::{http::StatusCode, Body, Error, IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> poem::Response {
        // You can customize how the error is formatted
        let body = json!({
            "code": self.code,
            "error": self.error,
        });

        // Return an HTTP response with the appropriate status code and JSON body
        poem::Response::builder()
            .status(StatusCode::from_u16(self.code as u16).unwrap())
            .header("Content-Type", "application/json")
            .body(Body::from_json(&body).unwrap())
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Error::from_response(err.into_response())
    }
}
