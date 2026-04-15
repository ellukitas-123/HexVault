use axum::{
    body::Body, extract::State, http::{Request, header}, middleware::Next, response::Response
};
use uuid::Uuid;

use crate::{auth::claims::Claims};
use crate::state::AppState;
use crate::error::AppError;

pub async fn auth_middleware(
	State(state): State<AppState>,
    mut req: Request<Body>, 
    next: Next
) -> Result<Response, AppError> {
    // Get the Authorization header
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    // Use your Claims logic (Make sure you have a way to get your secret here)
    let claims = Claims::decode(auth_header, &state.jwt_secret)?;

    // Set the UUID in the "Request Flow" (Extensions)
    let user_id: Uuid = claims.sub;
    req.extensions_mut().insert(user_id);

    // Continue to the next middleware or handler
    Ok(next.run(req).await)
}
