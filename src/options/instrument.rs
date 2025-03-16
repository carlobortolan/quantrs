//! Module for handling the underlying asset of an option and its dividend properties.
//!
//! An `Instrument` represents an underlying asset with dividend properties. It is used in option pricing models to calculate the price of an option.
//!
//! ## References
//! - [Wikipedia - Dividend yield](https://en.wikipedia.org/wiki/Dividend_yield)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::Instrument;
//!
//! let instrument = Instrument::new()
//!     .with_spot(100.0)
//!     .with_continuous_dividend_yield(0.2)
//!     .with_discrete_dividend_yield(0.0)
//!     .with_dividend_times(vec![]);
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
    pub fn new() -> Self {
        Self {
            spot: 0.0,
            continuous_dividend_yield: 0.0,
            discrete_dividend_yield: 0.0,
            dividend_times: Vec::new(),
        }
    }

    /// Set the spot price of the instrument.
    pub fn with_spot(mut self, spot: f64) -> Self {
        self.spot = spot;
        self
    }

    /// Set the continuous dividend yield of the instrument.
    pub fn with_continuous_dividend_yield(mut self, yield_: f64) -> Self {
        self.continuous_dividend_yield = yield_;
        self
    }

    /// Set the discrete dividend yield of the instrument.
    pub fn with_discrete_dividend_yield(mut self, yield_: f64) -> Self {
        self.discrete_dividend_yield = yield_;
        self
    }

    /// Set the dividend times of the instrument.
    pub fn with_dividend_times(mut self, times: Vec<f64>) -> Self {
        self.dividend_times = times;
        self
    }
}
