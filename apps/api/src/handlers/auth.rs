use axum::{extract::State, Json};
use serde_json::{json, Value};
use crypto::{hash_email, hash_password};
use crate::error::AppError;
use crate::{audit, error};
use crate::state::AppState;
use crate::models::user::{GetSaltPayload, LoginPayload, RegisterPayload};
use crate::auth::claims::Claims;

pub async fn register(
    State(state): State<AppState>, // Get database from AppState
    Json(payload): Json<RegisterPayload>, // Extract + validate req body
) -> Result<Json<Value>, AppError> {
    // Query
    let result = sqlx::query!(
        r#"
        INSERT INTO users (email_hash, master_password_hash, salt)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        hash_email(&payload.email, &state.email_peeper),
        hash_password(&payload.password),
        payload.salt
    )
    .fetch_one(&state.db) // Execute the query using our DB pool
    .await;

    // Check and send result
    match result {
        Ok(record) => {
            audit!("New User Registered: {}", record.id);

            Ok(Json(json!({
                "status": "success",
                "message": "User securely registered",
                "user_id": record.id
            })))
        },
        Err(e) => {
            Err(AppError::InternalServer(format!("Registration failed. {}", e)))
        }
    }
}

pub async fn get_salt(
    State(state): State<AppState>, // Get database from AppState
    Json(payload): Json<GetSaltPayload>, // Extract + validate req body
) -> Result<Json<Value>, AppError> {
    // Query
    let result = sqlx::query!(
        r#"
        SELECT salt
        FROM users
        WHERE email_hash = $1
        "#,
        hash_email(&payload.email, &state.email_peeper),
    )
    .fetch_one(&state.db) // Execute the query using our DB pool
    .await;

    // Check and send result
    match result {
        Ok(record) => Ok(Json(json!({
                "status": "success",
                "message": "Salt retrieved",
                "salt": record.salt
            }))),
        Err(e) => {
            error!("Failed to get salt: {}", e);
            Err(AppError::NotFound)
        }
    }
}

pub async fn login(
    State(state): State<AppState>, // Get database from AppState
    Json(payload): Json<LoginPayload>, // Extract + validate req body
) -> Result<Json<Value>, AppError> {
    // Query
    let result = sqlx::query!(
        r#"
        SELECT master_password_hash, id
        FROM users
        WHERE email_hash = $1
        "#,
        hash_email(&payload.email, &state.email_peeper),
    )
    .fetch_one(&state.db) // Execute the query using our DB pool
    .await;

    // Check and send result
    match result {
        Ok(record) => {
            if record.master_password_hash != hash_password(&payload.password) {
                error!("Failed to login (bad password)");
                Err(AppError::Unauthorized)
            } else {
                let claims = Claims::new(record.id);
                
                match claims.encode(&state.jwt_secret) {
                    Ok(token) => Ok(Json(json!({
                        "status": "success",
                        "message": "Logged in with token",
                        "token": token
                    }))),
                    Err(e) => Err(e)
                }
            }
        },
        Err(e) => {
            error!("Failed to login: {}", e);
            Err(AppError::Unauthorized)
        }
    }
}
