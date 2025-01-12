// Function to calculate compound interest
pub fn calculate_compound_interest(initial_capital: f64, interest_rate: f64, time: u32) -> (f64, f64) {
    let amount: f64 = initial_capital * (1.0 + interest_rate / 100.0).powi(time as i32);
    let accumulated_interest: f64 = amount - initial_capital;
    (amount, accumulated_interest)
}
