//! Module for finite difference option pricing model.

use crate::options::{Option, OptionPricing, OptionStrategy};

/// Finite difference option pricing model.
#[derive(Debug, Default)]
pub struct FiniteDiffModel {
    /// Risk-free interest rate (e.g., 0.05 for 5%).
    pub risk_free_rate: f64,
    /// Volatility of the underlying asset (e.g., 0.2 for 20%).
    pub volatility: f64,
    /// Number of steps in the binomial tree.
    pub steps: usize,
}

impl FiniteDiffModel {
    /// Create a new `FiniteDiffModel`.
    pub fn new(risk_free_rate: f64, volatility: f64, steps: usize) -> Self {
        Self {
            risk_free_rate,
            volatility,
            steps,
        }
    }
}

impl OptionPricing for FiniteDiffModel {
    fn price<T: Option>(&self, option: &T) -> f64 {
        panic!("FiniteDiffModel does not support price calculation yet");
    }

    fn implied_volatility<T: Option>(&self, _option: &T, _market_price: f64) -> f64 {
        panic!("FiniteDiffModel does not support implied volatility calculation yet");
    }
}

impl OptionStrategy for FiniteDiffModel {}
