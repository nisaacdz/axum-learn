use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    serve, Json, Router,
};
use axum_learn::{CreateUserPayload, User};
use axum_learn::login;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .merge(login::routes());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> impl IntoResponse {
    return Html("Hello, <strong>World!</strong>");
}

async fn create_user(
    Json(CreateUserPayload { username }): Json<CreateUserPayload>,
) -> (StatusCode, Json<User>) {
    let user = User { id: 1336, username };

    (StatusCode::CREATED, Json(user))
}
