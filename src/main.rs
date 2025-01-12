use axum::{Router, Server};
use sentry::integrations::anyhow::capture_anyhow;
use axum_extra::middleware::metrics::PrometheusMetricLayer;
use std::net::SocketAddr;
mod routes;
mod models;
mod services;

#[tokio::main]
async fn main() {
    let _guard = sentry::init(("https://f10ec06978e996aea2a2542c5320f9da@o4508631411064832.ingest.us.sentry.io/4508632300847104", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    let prometheus_layer = PrometheusMetricLayer::new();

    // Create the application router
    let app: Router = routes::create_router().layer(prometheus_layer);

    // Define the address and port for the server
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("Server running at http://{}", addr);

    // Bind the server to the address and serve the application
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
