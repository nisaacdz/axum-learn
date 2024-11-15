use serde::{Deserialize, Serialize};

pub mod login;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
}
