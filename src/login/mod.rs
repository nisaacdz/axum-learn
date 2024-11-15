mod error;
use axum::{routing::post, Json, Router};
pub use error::*;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
struct LoginPayload {
    username: String,
    password: String,
}

const pws: [&str; 2] = ["isaac", "newton"];

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if !pws.contains(&&payload.username[..]) && !pws.contains(&&payload.password[..]) {
        return Err(LoginError::LoginFailed);
    }

    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}
