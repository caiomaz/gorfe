use axum::{extract::Json, response::IntoResponse, routing::{get, post}, Router};
use crate::{models::{InterestRequest, InterestResponse}, services};
use sentry::ClientOptions;
use std::env;

pub async fn calculate(Json(payload): Json<InterestRequest>) -> impl IntoResponse {
    tracing::info!("Recebido payload: {:?}", payload);
    if payload.initial_capital < 0.0 || payload.interest_rate < 0.0 || payload.time == 0 {
        return Json(InterestResponse {
            amount: 0.0,
            accumulated_interest: 0.0,
            input_data: payload,
            error_message: Some("Invalid inputs: negative values or zero time".to_string()),
        });
    }

    let (amount, accumulated_interest) = services::calculate_compound_interest(
        payload.initial_capital,
        payload.interest_rate,
        payload.time,
    );
    tracing::info!("Cálculo final: amount = {}, interest = {}", amount, accumulated_interest);

    Json(InterestResponse {
        amount,
        accumulated_interest,
        input_data: payload,
        error_message: None,
    })
}

pub async fn root() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn trigger_error() -> impl IntoResponse {
    let dsn = env::var("SENTRY_DSN").expect("SENTRY_DSN must be set");
    let _guard = sentry::init((dsn, ClientOptions::default()));
    sentry::capture_message("This is a test error", sentry::Level::Error);
    "Error sent to Sentry"
}

pub fn create_router() -> Router {
    Router::new() 
        .route("/", get(root))
        .route("/calculate", post(calculate))
        .route("/trigger_error", get(trigger_error))
}