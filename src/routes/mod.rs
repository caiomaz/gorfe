use axum::{extract::Json, response::IntoResponse, routing::{get, post}, Router};
use crate::{models::{InterestRequest, InterestResponse}, services};

// Handler function to calculate compound interest
pub async fn calculate(Json(payload): Json<InterestRequest>) -> impl IntoResponse {
    // Validate input data
    if payload.initial_capital < 0.0 || payload.interest_rate < 0.0 || payload.time == 0 {
        return Json(InterestResponse {
            amount: 0.0,
            accumulated_interest: 0.0,
            input_data: payload,
            error_message: Some("Invalid inputs: negative values or zero time".to_string()),
        });
    }

    // Calculate compound interest
    let (amount, accumulated_interest) = services::calculate_compound_interest(
        payload.initial_capital,
        payload.interest_rate,
        payload.time,
    );

    // Return the response
    Json(InterestResponse {
        amount,
        accumulated_interest,
        input_data: payload,
        error_message: None,
    })
}

// Handler function for the root path
pub async fn root() -> impl IntoResponse {
    "Hello, World!"
}

// Function to create the application router
pub fn create_router() -> Router {
    Router::new() 
        .route("/", get(root))
        .route("/calculate", post(calculate))
}
