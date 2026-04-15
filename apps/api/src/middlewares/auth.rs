use axum::{
    body::Body, extract::State, http::Request, middleware::Next, response::Response
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
    // Parse all cookies into a typed CookieJar
    let jar = axum_extra::extract::cookie::CookieJar::from_headers(req.headers());

    // Extract the exact "token" cookie by name
    let token = jar.get("HV_tk")
        .map(|cookie| cookie.value())
        .ok_or(AppError::Unauthorized)?;

    // Decode the claims
    let claims = Claims::decode(token, &state.jwt_secret)?;

    // Set the UUID in the "Request Flow" (Extensions)
    let user_id: Uuid = claims.sub;
    req.extensions_mut().insert(user_id);

    // Continue to the next middleware or handler
    Ok(next.run(req).await)
}
