use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, LoginError>;

#[derive(Debug)]
pub enum LoginError {
    LoginFailed,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}