use axum::{Router, Server};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum_prometheus::PrometheusMetricLayer;
use std::net::SocketAddr;
mod routes;
mod models;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Iniciando servidor...");

    let prometheus_layer = PrometheusMetricLayer::new();

    let app: Router = routes::create_router().layer(prometheus_layer);

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
