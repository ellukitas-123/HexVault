use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;
use crate::{audit, error};
use crate::error::{AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // Subject (usually the User ID)
    pub exp: usize,  // Expiration time (as UTC timestamp)
    pub iat: usize,  // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::hours(24); // Token valid for 24 hours

        Self {
            sub: user_id,
            iat: now.timestamp() as usize,
            exp: expiration.timestamp() as usize,
        }
    }

    pub fn encode(&self, secret: &str) -> Result<String, AppError> {
        let key = EncodingKey::from_secret(secret.as_ref());
        
        encode(&Header::default(), self, &key)
            .map_err(|e| {
                // Use your audit macro to log the real error
                error!("JWT Encoding Failed: {}", e);
                AppError::InternalServer("Could not generate session token".into())
            })
    }

    pub fn decode(token: &str, secret: &str) -> Result<Self, AppError> {
        let key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);

        decode::<Self>(token, &key, &validation)
            .map(|token_data| token_data.claims) // Extract the claims from wrapper
            .map_err(|e| {
                match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        // Expired JWT
                        AppError::Unauthorized
                    }
                    _ => {
                        // Log crypto errors internally.
                        // Bad signature or internal errors
                        audit!("JWT Decode Error: {}", e); 
                        AppError::Unauthorized
                    }
                }
            })
    }
}
