use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};

pub async fn response_middleware(req: Request<Body>, next: Next) -> Response<Body> {
    // Run the handler
    let response = next.run(req).await;

    // Extract parts and body
    let (parts, body) = response.into_parts();
    let status = parts.status;

    // Convert body to bytes so we can parse it as JSON
    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // Try to parse the body as JSON. If it's not JSON, we treat it as a raw string.
    let original_json: Value = serde_json::from_slice(&bytes).unwrap_or_else(|_| {
        json!(String::from_utf8_lossy(&bytes))
    });

    // Success?   
    let unified_body = if status.is_success() {
        json!({
            "status": "success",
            "data": original_json
        })
    } else {
        // If it's an error, the body is likely our AppError's message
        json!({
            "status": "error",
            "message": original_json,
            "data": null
        })
    };

    // 6. Re-build the response with the new body
    (status, Json(unified_body)).into_response()
}