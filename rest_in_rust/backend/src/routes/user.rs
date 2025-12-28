use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::user::{health, create_user};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Backend running!" }))
        .route("/health", get(health))
        .route("/users", post(create_user))
}
