use axum::{
    Router, routing::{get, post}, middleware
};
use crate::{handlers::auth::{get_asymmetric_key, get_salt, login, register}, state::AppState};
use crate::middlewares::auth::auth_middleware;

pub fn auth_router(state: AppState) -> Router<AppState> {
	// Group public and protected auth endpoints in separate routers.
    let auth_public_routes = Router::new()
        .route("/register", post(register))
        .route("/salt", get(get_salt))
        .route("/login", get(login));

    let auth_protected_routes = Router::new()
        .route("/keys", get(get_asymmetric_key))
        .layer(middleware::from_fn_with_state(state, auth_middleware));

    let auth_routes = auth_public_routes.merge(auth_protected_routes);
	auth_routes
}