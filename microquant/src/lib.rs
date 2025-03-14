//! # MicroQuant
//! A tiny Rust library for options pricing, portfolio optimization, and risk analysis.

pub mod data;
pub mod fixed_income;
pub mod options;
pub mod portfolio;
pub mod timeseries;

#[cfg(test)]
mod tests {
    use super::options::black_scholes::black_scholes_call_price;

    #[test]
    fn test_black_scholes_call_price() {
        let spot = 100.0;
        let strike = 100.0;
        let time_to_maturity = 1.0;
        let risk_free_rate = 0.05;
        let volatility = 0.2;
        let price =
            black_scholes_call_price(spot, strike, time_to_maturity, risk_free_rate, volatility);
        assert!((price - 10.4506).abs() < 0.0001);
    }
}
