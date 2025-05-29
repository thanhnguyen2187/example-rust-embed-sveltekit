mod frontend;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/api/data",
            get(async || {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                "some arbitrary data"
            }),
        )
        .fallback(frontend::static_handler);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
