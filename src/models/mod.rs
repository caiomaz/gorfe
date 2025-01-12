use serde::{Deserialize, Serialize};

// Struct to represent the interest calculation request
#[derive(Deserialize, Serialize)]
pub struct InterestRequest {
    pub initial_capital: f64,
    pub interest_rate: f64,
    pub time: u32,
}

// Struct to represent the interest calculation response
#[derive(Serialize)]
pub struct InterestResponse {
    pub amount: f64,
    pub accumulated_interest: f64,
    pub input_data: InterestRequest,
    pub error_message: Option<String>,
}
