use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn list_files() -> &'static str {
    "File list (TODO: connect to DB)"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/files", get(list_files));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}