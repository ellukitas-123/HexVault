use sqlx::PgPool;

// Clone because Axum needs to give a copy of this state 
// to every request.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String
}
