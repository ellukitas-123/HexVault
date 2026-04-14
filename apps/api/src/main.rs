pub mod state;

pub mod error;
pub mod handlers;
pub mod models;
pub mod utilities;
pub mod auth;

use std::env;
use axum::{
    Router, response::Json, routing::{get, post}
};
use sqlx::postgres::PgPoolOptions;
use serde_json::{Value, json};

use crate::state::AppState;
use crate::handlers::auth::{register, get_salt, login};

// The #[tokio::main] macro tells Rust to run this main function 
// using the Tokio async engine.
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect(&fatal_str!("env DATABASE_URL is not set"));
    let email_peeper = env::var("EMAIL_PEEPER")
        .expect(&fatal_str!("env EMAIL_PEEPER is not set"));
    let jwt_secret = env::var("JWT_SECRET")
        .expect(&fatal_str!("env JWT_SECRET is not set"));

    // Database connection
    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect(&fatal_str!("Failed to connect to the database. Maybe the DATABASE_URL is not correctly set"));

    // Set state
    let state = AppState { 
        db: pool,
        email_peeper: email_peeper,
        jwt_secret: jwt_secret
    };

    // Application router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/salt", get(get_salt))
        .route("/auth/login", get(login))
        .with_state(state);

    // Listening address + port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 HexVault API is running on http://127.0.0.1:3000");

    // Start the server
    axum::serve(listener, app).await.unwrap();
}

// ---------------------------------------------------------
// BASIC ROUTE HANDLERS
// ---------------------------------------------------------

async fn health_check() -> Json<Value> {
    let response = json!({
        "status": "online",
        "service": "HexVault API",
        "version": "0.1.0",
    });

    // Return the JSON response
    Json(response)
}
