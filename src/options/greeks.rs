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
//! use quantrs::options::{EuropeanOption, BlackScholesModel, OptionGreeks, OptionType, Instrument};
//!
//! let option = EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Call);
//! let model = quantrs::options::BlackScholesModel::new(1.0, 0.05, 0.2);
//!
//! let greeks = OptionGreeks::calculate(&model, option);
//! println!("Delta: {}", greeks.delta);
//! println!("Gamma: {}", greeks.gamma);
//! println!("Theta: {}", greeks.theta);
//! println!("Vega: {}", greeks.vega);
//! println!("Rho: {}", greeks.rho);
//! ```

use super::{Greeks, Option};

/// A struct representing the Greeks of an option.
#[derive(Debug)]
pub struct OptionGreeks {
    /// Delta measures the rate of change of the option price with respect to changes in the price of the underlying asset.
    pub delta: f64,
    /// Gamma measures the rate of change of the option delta with respect to changes in the price of the underlying asset.
    pub gamma: f64,
    /// Theta measures the rate of change of the option price with respect to changes in time to maturity.
    pub theta: f64,
    /// Vega measures the rate of change of the option price with respect to changes in the volatility of the underlying asset.
    pub vega: f64,
    /// Rho measures the rate of change of the option price with respect to changes in the risk-free interest rate.
    pub rho: f64,
    /// Lambda measures the rate of change of the option delta with respect to changes in the risk-free interest rate.
    pub lambda: f64,
    /// Vanna measures the rate of change of the option delta with respect to changes in the volatility of the underlying asset.
    pub vanna: f64,
    /// Charm measures the rate of change of the option delta with respect to changes in time to maturity.
    pub charm: f64,
    /// Vomma measures the rate of change of the option vega with respect to changes in the volatility of the underlying asset.
    pub vomma: f64,
    /// Veta measures the rate of change of the option vega with respect to changes in time to maturity.
    pub veta: f64,
    /// Speed measures the rate of change of the option gamma with respect to changes in the price of the underlying asset.
    pub speed: f64,
    /// Zomma measures the rate of change of the option gamma with respect to changes in the volatility of the underlying asset.
    pub zomma: f64,
    /// Color measures the rate of change of the option gamma with respect to changes in time to maturity.
    pub color: f64,
    /// Ultima measures the rate of change of the option vomma with respect to changes in the volatility of the underlying asset.
    pub ultima: f64,
}

impl OptionGreeks {
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
    pub fn calculate<T: Greeks, S: Option>(model: &T, option: S) -> Self {
        OptionGreeks {
            delta: model.delta(option.clone()),
            gamma: model.gamma(option.clone()),
            theta: model.theta(option.clone()),
            vega: model.vega(option.clone()),
            rho: model.rho(option.clone()),
            lambda: model.lambda(option.clone()),
            vanna: model.vanna(option.clone()),
            charm: model.charm(option.clone()),
            vomma: model.vomma(option.clone()),
            veta: model.veta(option.clone()),
            speed: model.speed(option.clone()),
            zomma: model.zomma(option.clone()),
            color: model.color(option.clone()),
            ultima: model.ultima(option.clone()),
        }
    }
}
