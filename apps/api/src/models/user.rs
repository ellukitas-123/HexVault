use serde::Deserialize;

// #[derive(Deserialize)] converts raw incoming JSON into this strict struct.
#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
    pub salt: String,
}

#[derive(Deserialize)]
pub struct GetSaltPayload {
    pub email: String
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String
}
