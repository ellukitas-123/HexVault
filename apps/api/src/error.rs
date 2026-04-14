use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{fatal_str};

pub enum AppError {
    BadRequest(String),      // 400: Client payload is not valid
    Unauthorized,            // 401: Bad/Missing JWT
    Forbidden,               // 403: Authenticated but not allowed
    NotFound,                // 404: Resource missing
    InternalServer(String),  // 500: Database/Server crash
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized access".into()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Access denied".into()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".into()),
            AppError::InternalServer(msg) => {
                // Real error in the server...
                println!("{}", fatal_str!("Internal Failure: {}", msg));
                
                // Generic message to the user for security.
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred".into())
            }
        };

        let body = Json(json!({
            "status": "error",
            "message": error_message,
			"data": null
        }));

        (status, body).into_response()
    }
}
