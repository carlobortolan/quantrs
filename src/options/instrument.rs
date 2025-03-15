//! Module for handling the underlying asset of an option and its dividend properties.
//!
//! An `Instrument` represents an underlying asset with dividend properties. It is used in option pricing models to calculate the price of an option.
//!
//! ## References
//! - [Wikipedia - Dividend yield](https://en.wikipedia.org/wiki/Dividend_yield)
//! - [Investopedia - Dividend Yield](https://www.investopedia.com/terms/d/dividendyield.asp)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::Instrument;
//!
//! let instrument = Instrument {
//!   spot: 100.0,
//!   continuous_dividend_yield: 0.2,
//!   discrete_dividend_yield: 0.0,
//!   dividend_times: vec![],
//! };
//! ```

/// A struct representing an instrument with dividend properties.
#[derive(Debug, Default, Clone)]
pub struct Instrument {
    /// Current price of the underlying asset.
    pub spot: f64,
    /// Continuous dividend yield where the dividend amount is proportional to the level of the underlying asset (e.g., 0.02 for 2%).
    pub continuous_dividend_yield: f64,
    /// Discrete proportional dividend yield (e.g., 0.02 for 2%).
    pub discrete_dividend_yield: f64,
    /// Times at which discrete dividends are paid.
    pub dividend_times: Vec<f64>,
}

impl Instrument {
    /// Create a new `Instrument`.
    pub fn new(spot: f64) -> Self {
        Self {
            spot,
            continuous_dividend_yield: 0.0,
            discrete_dividend_yield: 0.0,
            dividend_times: Vec::new(),
        }
    }

    /// Get the current price of the underlying asset.
    pub fn spot(&self) -> f64 {
        self.spot
    }
}
