pub mod state;

pub mod routes;
pub mod middlewares;
pub mod handlers;
pub mod error;
pub mod models;
pub mod utilities;
pub mod auth;

use std::env;
use axum::{
    Router, response::Json, routing::{get}, middleware
};
use sqlx::postgres::PgPoolOptions;
use serde_json::{Value, json};

use crate::state::AppState;


// The #[tokio::main] macro tells Rust to run this main function 
// using the Tokio async engine.
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect(&fatal_str!("env DATABASE_URL is not set"));
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
        jwt_secret: jwt_secret
    };

    // Application router
    let app = Router::new()
        .route("/", get(health_check))
        .nest("/auth", routes::auth::auth_router(state.clone()))
        .layer(middleware::from_fn(middlewares::response::response_middleware))
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
