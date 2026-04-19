use serde::Deserialize;
use validator::Validate;

// #[derive(Deserialize)] converts raw incoming JSON into this strict struct.
#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub salt: String,
    pub encrypted_private_key: String,
    pub public_key: String
}

#[derive(Deserialize, Validate)]
pub struct GetSaltPayload {
    #[validate(email)]
    pub email: String
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    pub email: String,
    pub password: String
}
