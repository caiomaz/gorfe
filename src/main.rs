use axum::{Router, Server};
use std::net::SocketAddr;
mod routes;
mod models;
mod services;

#[tokio::main]
async fn main() {
    // Create the application router
    let app: Router = routes::create_router();

    // Define the address and port for the server
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Server running at http://{}", addr);

    // Bind the server to the address and serve the application
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
