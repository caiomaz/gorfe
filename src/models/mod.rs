use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InterestRequest {
    pub initial_capital: f64,
    pub interest_rate: f64,
    pub time: u32,
}

#[derive(Serialize)]
pub struct InterestResponse {
    pub amount: f64,
    pub accumulated_interest: f64,
    pub input_data: InterestRequest,
    pub error_message: Option<String>,
}
