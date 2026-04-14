use hmac::{Hmac, KeyInit, Mac};
use sha2::{Sha256, Digest};

// Alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

// Protects against "Pass-the-Hash" attacks.
// Takes the Argon2 hash sent by the client and hashes it again using SHA-256
pub fn hash_password(password_hash: &str) -> String {
    // Initialize the SHA-256 engine
    let mut hasher = Sha256::new();
    hasher.update(password_hash.as_bytes());
    let result = hasher.finalize();
    
    // Convert the raw bytes into readable Hex string
    hex::encode(result)
}

/// Protects against Email Unmasking if the database is stolen.
/// Mixes the client's SHA-256 email hash with server Pepper.
pub fn hash_email(email_hash: &str, peeper: &str) -> String {
    // Initialize the HMAC engine with Peeper
    let mut mac = HmacSha256::new_from_slice(peeper.as_bytes())
        .expect("HMAC can take key of any size");
        
    // Mix in the email hash from the frontend
    mac.update(email_hash.as_bytes());
    
    // Finalize and convert to a hex string
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}
