//! Module for calculating the Greeks of an option.
//!
//! The Greeks are calculated using the formulas provided by the `Greeks` trait.
//!
//! ## References
//! - [Wikipedia - Option Greeks](https://en.wikipedia.org/wiki/Greeks_(finance))
//! - [Options, Futures, and Other Derivatives (9th Edition)](https://www.pearson.com/store/p/options-futures-and-other-derivatives/P1000000000000013194)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{EuropeanOption, BlackScholesModel, Greeks, OptionType, Instrument};
//!
//! let option = EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, 1.0, OptionType::Call);
//! let model = quantrs::options::BlackScholesModel::new(0.05, 0.2);
//!
//! let greeks = Greeks::calculate(&model, &option);
//! println!("Delta: {}", greeks.delta);
//! println!("Gamma: {}", greeks.gamma);
//! println!("Theta: {}", greeks.theta);
//! println!("Vega: {}", greeks.vega);
//! println!("Rho: {}", greeks.rho);
//! ```

use super::{Option, OptionGreeks};
use std::panic::{catch_unwind, AssertUnwindSafe};

/// A struct representing the Greeks of an option.
#[derive(Debug)]
pub struct Greeks {
    // First-order Greeks
    /// Delta measures the rate of change of the option price with respect to changes in the price of the underlying asset.
    pub delta: f64,
    /// Vega measures the rate of change of the option price with respect to changes in the volatility of the underlying asset.
    pub vega: f64,
    /// Theta measures the rate of change of the option price with respect to changes in time to maturity.
    pub theta: f64,
    /// Rho measures the rate of change of the option price with respect to changes in the risk-free interest rate.
    pub rho: f64,
    /// Lambda measures the rate of change of the option delta with respect to changes in the risk-free interest rate.
    pub lambda: f64,
    /// Epsilon measures the rate of change of the option delta with respect to changes in the dividend yield.
    pub epsilon: f64,

    // Second-order Greeks
    /// Gamma measures the rate of change of the option delta with respect to changes in the price of the underlying asset.
    pub gamma: f64,
    /// Vanna measures the rate of change of the option delta with respect to changes in the volatility of the underlying asset.
    pub vanna: f64,
    /// Charm measures the rate of change of the option delta with respect to changes in time to maturity.
    pub charm: f64,
    /// Vomma measures the rate of change of the option vega with respect to changes in the volatility of the underlying asset.
    pub vomma: f64,
    /// Veta measures the rate of change of the option vega with respect to changes in time to maturity.
    pub veta: f64,
    /// Speed measures the rate of change of the option gamma with respect to changes in the price of the underlying asset.
    pub vera: f64,

    // Third-order Greeks
    /// Speed measures the rate of change of the option gamma with respect to changes in the price of the underlying asset.
    pub speed: f64,
    /// Zomma measures the rate of change of the option gamma with respect to changes in the volatility of the underlying asset.
    pub zomma: f64,
    /// Color measures the rate of change of the option gamma with respect to changes in time to maturity.
    pub color: f64,
    /// Ultima measures the rate of change of the option vomma with respect to changes in the volatility of the underlying asset.
    pub ultima: f64,
    /// Parmicharma measures the rate of change of charm over the passage of time.
    pub parmicharma: f64,
}

impl Greeks {
    /// Calculate the Greeks for a given option.
    ///
    /// Arguments
    ///
    /// * `option` - The option for which to calculate the Greeks.
    /// * `option_type` - The type of option (Call or Put).
    ///
    /// Returns
    ///
    /// The calculated Greeks.
    #[rustfmt::skip]
    pub fn calculate<T: OptionGreeks, S: Option>(model: &T, option: &S) -> Self {
        Greeks {
            delta: catch_unwind(AssertUnwindSafe(|| model.delta(option))).unwrap_or_default(),
            vega: catch_unwind(AssertUnwindSafe(|| model.vega(option))).unwrap_or_default(),
            theta: catch_unwind(AssertUnwindSafe(|| model.theta(option))).unwrap_or_default(),
            rho: catch_unwind(AssertUnwindSafe(|| model.rho(option))).unwrap_or_default(),
            lambda: catch_unwind(AssertUnwindSafe(|| model.lambda(option))).unwrap_or_default(),
            epsilon: catch_unwind(AssertUnwindSafe(|| model.epsilon(option))).unwrap_or_default(),

            gamma: catch_unwind(AssertUnwindSafe(|| model.gamma(option))).unwrap_or_default(),
            vanna: catch_unwind(AssertUnwindSafe(|| model.vanna(option))).unwrap_or_default(),
            charm: catch_unwind(AssertUnwindSafe(|| model.charm(option))).unwrap_or_default(),
            vomma: catch_unwind(AssertUnwindSafe(|| model.vomma(option))).unwrap_or_default(),
            veta: catch_unwind(AssertUnwindSafe(|| model.veta(option))).unwrap_or_default(),
            vera: catch_unwind(AssertUnwindSafe(|| model.vera(option))).unwrap_or_default(),

            speed: catch_unwind(AssertUnwindSafe(|| model.speed(option))).unwrap_or_default(),
            zomma: catch_unwind(AssertUnwindSafe(|| model.zomma(option))).unwrap_or_default(),
            color: catch_unwind(AssertUnwindSafe(|| model.color(option))).unwrap_or_default(),
            ultima: catch_unwind(AssertUnwindSafe(|| model.ultima(option))).unwrap_or_default(),
            parmicharma: catch_unwind(AssertUnwindSafe(|| model.parmicharma(option))).unwrap_or_default(),
        }
    }
}
