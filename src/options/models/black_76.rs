//! Module for Black76 option pricing model.

use crate::options::{Option, OptionGreeks, OptionPricing, OptionStrategy};

/// Black76 option pricing model.
#[derive(Debug, Default)]
pub struct Black76Model {
    /// Risk-free interest rate (e.g., 0.05 for 5%).
    pub risk_free_rate: f64,
    /// Volatility of the underlying asset (e.g., 0.2 for 20%).
    pub volatility: f64,
}

impl Black76Model {
    /// Create a new `Black76Model`.
    pub fn new(risk_free_rate: f64, volatility: f64) -> Self {
        Self {
            risk_free_rate,
            volatility,
        }
    }
}

impl OptionPricing for Black76Model {
    fn price<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support price calculation yet");
    }

    fn implied_volatility<T: Option>(&self, _option: &T, _market_price: f64) -> f64 {
        panic!("Black76Model does not support implied volatility calculation yet");
    }
}

impl OptionGreeks for Black76Model {
    fn delta<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support delta calculation yet");
    }

    fn gamma<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support gamma calculation yet");
    }

    fn theta<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support theta calculation yet");
    }

    fn vega<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support vega calculation yet");
    }

    fn rho<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support rho calculation yet");
    }
}

impl OptionStrategy for Black76Model {}
