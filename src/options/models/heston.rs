//! Module for Heston option pricing model.

use crate::options::{Option, OptionPricing};

/// Heston option pricing model.
#[derive(Debug, Default)]
pub struct HestonModel {
    /// Time horizon (in years).
    pub time_to_maturity: f64,
    /// Risk-free interest rate (e.g., 0.05 for 5%).
    pub risk_free_rate: f64,
    /// Volatility of the underlying asset (e.g., 0.2 for 20%).
    pub volatility: f64,
    /// Number of steps in the binomial tree.
    pub steps: usize,
}

impl HestonModel {
    /// Create a new `HestonModel`.
    pub fn new(time_to_maturity: f64, risk_free_rate: f64, volatility: f64, steps: usize) -> Self {
        Self {
            time_to_maturity,
            risk_free_rate,
            volatility,
            steps,
        }
    }
}

impl OptionPricing for HestonModel {
    fn price<T: Option>(&self, option: &T) -> f64 {
        panic!("HestonModel does not support price calculation yet");
    }

    fn implied_volatility<T: Option>(&self, _option: &T, _market_price: f64) -> f64 {
        panic!("HestonModel does not support implied volatility calculation yet");
    }
}
