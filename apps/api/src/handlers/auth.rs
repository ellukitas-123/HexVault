use axum::{extract::State, Json};
use serde_json::{json, Value};
use crypto::{hash_email, hash_password};
use crate::{audit, error};
use crate::state::AppState;
use crate::models::user::{GetSaltPayload, RegisterPayload};

pub async fn register(
    State(state): State<AppState>, // Get database from AppState
    Json(payload): Json<RegisterPayload>, // Extract + validate req body
) -> Json<Value> {
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
            Json(json!({
                "status": "success",
                "message": "User securely registered",
                "user_id": record.id
            }))
        },
        Err(e) => {
            error!("Failed to register user: {}", e);
            Json(json!({
                "status": "error",
                "message": "Registration failed. User might already exist."
            }))
        }
    }
}

pub async fn get_salt(
    State(state): State<AppState>, // Get database from AppState
    Json(payload): Json<GetSaltPayload>, // Extract + validate req body
) -> Json<Value> {
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
        Ok(record) => {
            Json(json!({
                "status": "success",
                "message": "Salt retrieved",
                "salt": record.salt
            }))
        },
        Err(e) => {
            error!("Failed to get salt: {}", e);
            Json(json!({
                "status": "error",
                "message": "Couldn't obtain salt. Email might not be correct"
            }))
        }
    }
}
