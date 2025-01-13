use axum::{Router, Server};
use dotenv::dotenv;
use sentry::integrations::tracing::layer as sentry_layer;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let dsn = env::var("SENTRY_DSN").expect("SENTRY_DSN must be set");
    let _guard = sentry::init((dsn, sentry::ClientOptions::default()));

    tracing_subscriber::registry()
        .with(sentry_layer())
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Iniciando servidor...");

    let app: Router = routes::create_router();

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}