use axum::Json;

use crate::models::user::{CreateUser, UserResponse};

pub async fn health() -> Json<&'static str> {
    Json("ok")
}

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> Json<UserResponse> {
    Json(UserResponse {
        id: 1,
        username: payload.username,
        email: payload.email,
    })
}
