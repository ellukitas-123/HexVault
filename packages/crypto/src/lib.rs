use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2, Params, Algorithm, Version
};

// Argon2id parameters: 
// - Memory: 64 MB (65536 KB)
// - Iterations: 3
// - Parallelism: 4
const MEMORY_SIZE: u32 = 65536;
const ITERATIONS: u32 = 3;
const PARALLELISM: u32 = 4;

// Hashes a the Argon2id hashed hash (double hashed) using Argon2id (third-hash)
pub fn hash_password(password_hash: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    
    let params = Params::new(MEMORY_SIZE, ITERATIONS, PARALLELISM, None)
        .expect("Invalid Argon2 parameters");
    
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    argon2.hash_password(password_hash.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

/// Verifies a password against a provided Argon2id hash.
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).expect("Invalid hash format");
    
    // Argon2::verify_password uses the parameters stored in the hash string, 
    // so we don't strictly need to pass the custom params here for verification,
    // but initializing with Argon2id is correct.
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

