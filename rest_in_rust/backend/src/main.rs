use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};

use backend::routes::user::user_routes;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(user_routes())
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
