//! Module for various option pricing models.
//!
//! ## Supported models
//!
//! - [Black-Scholes Option Pricing Model](black_scholes/struct.BlackScholesModel.html)
//! - [Binomial Option Pricing Model](binomial_tree/struct.BinomialTreeModel.html)
//! - [Monte Carlo Option Pricing Model](monte_carlo/struct.MonteCarloModel.html)
//!
//! ## Greek calculations
//!
//! This module also provides implementations of the Greeks for each option pricing model.
//! See the [Greeks](options/trait.Greeks.html) trait for more information.

pub mod greeks;
pub mod instrument;
pub mod models;
pub mod types;

pub use greeks::OptionGreeks;
pub use instrument::Instrument;
pub use models::{BinomialTreeModel, BlackScholesModel, MonteCarloModel};
pub use types::{AmericanOption, EuropeanOption, OptionStyle, OptionType};

/// Supertrait that combines OptionPricing and Greeks.
pub trait Option: Clone {
    /// Get the style of the option.
    ///
    /// # Returns
    ///
    /// The style of the option.
    fn style(&self) -> &OptionStyle;

    /// Get the underlying instrument of the option.
    ///
    /// # Returns
    ///
    /// The underlying instrument of the option.
    fn instrument(&self) -> &Instrument;

    /// Get the strike price of the option.
    ///
    /// # Returns
    ///
    /// The strike price of the option.
    fn strike(&self) -> f64;

    /// Get the type of the option.
    ///
    /// # Returns
    ///     
    /// The type of the option.
    fn option_type(&self) -> OptionType;

    /// Flip the option type (Call to Put or Put to Call).
    ///
    /// # Returns
    ///
    /// The flipped option.
    fn flip(&self) -> Self;

    /// Calculate the payoff of the option at maturity.
    ///
    /// # Arguments
    ///
    /// * `underlying_price` - The price of the underlying asset at maturity.
    /// * `option_type` - The type of option (Call or Put).
    ///
    /// # Returns
    ///
    /// The payoff of the option.
    fn payoff(&self, spot: f64) -> f64 {
        match self.option_type() {
            OptionType::Call => (spot - self.strike()).max(0.0),
            OptionType::Put => (self.strike() - spot).max(0.0),
        }
    }
}

/// Trait for option pricing models.
pub trait OptionPricing {
    /// Calculate the option price.
    ///
    /// # Arguments
    ///
    /// * `option_type` - The type of option (Call or Put).
    ///
    /// # Returns
    ///
    /// The price of the option.
    fn price<T: Option>(&self, option: T) -> f64;

    /// Calculate the implied volatility for a given market price.
    ///
    /// # Arguments
    ///
    /// * `market_price` - The market price of the option.
    /// * `option_type` - The type of option (Call or Put).
    ///
    /// # Returns
    ///
    /// The implied volatility.
    fn implied_volatility<T: Option>(&self, option: T, market_price: f64) -> f64;
}

/// Trait for calculating the Greeks of an option.
pub trait Greeks {
    // First order Greeks
    /// Delta measures the rate of change of the option price with respect to changes in the price of the underlying asset.
    fn delta<T: Option>(&self, option: T) -> f64;
    /// Gamma measures the rate of change of the option delta with respect to changes in the price of the underlying asset.
    fn gamma<T: Option>(&self, option: T) -> f64;
    /// Theta measures the rate of change of the option price with respect to changes in time to maturity.
    fn theta<T: Option>(&self, option: T) -> f64;
    /// Vega measures the rate of change of the option price with respect to changes in the volatility of the underlying asset.
    fn vega<T: Option>(&self, option: T) -> f64;
    /// Rho measures the rate of change of the option price with respect to changes in the risk-free interest rate.
    fn rho<T: Option>(&self, option: T) -> f64;

    // Higher order Greeks
    /// Lambda measures the rate of change of the option delta with respect to changes in the risk-free interest rate.
    fn lambda<T: Option>(&self, option: T) -> f64 {
        0.0 // Placeholder value
    }
    /// Vanna measures the rate of change of the option delta with respect to changes in the volatility of the underlying asset.
    fn vanna<T: Option>(&self, option: T) -> f64 {
        0.0 // Placeholder value
    }
    /// Charm measures the rate of change of the option delta with respect to changes in time to maturity.
    fn charm<T: Option>(&self, option: T) -> f64 {
        0.0 // Placeholder value
    }
    /// Vomma measures the rate of change of the option vega with respect to changes in the volatility of the underlying asset.
    fn vomma<T: Option>(&self, option: T) -> f64 {
        0.0 // Placeholder value
    }
    /// Veta measures the rate of change of the option vega with respect to changes in time to maturity.
    fn veta<T: Option>(&self, option: T) -> f64 {
        0.0 // Placeholder value
    }
    /// Speed measures the rate of change of the option gamma with respect to changes in the price of the underlying asset.
    fn speed<T: Option>(&self, option: T) -> f64 {
        0.0 // Placeholder value
    }
    /// Zomma measures the rate of change of the option gamma with respect to changes in the volatility of the underlying asset.
    fn zomma<T: Option>(&self, option: T) -> f64 {
        0.0 // Placeholder value
    }
    /// Color measures the rate of change of the option gamma with respect to changes in time to maturity.
    fn color<T: Option>(&self, option: T) -> f64 {
        0.0 // Placeholder value
    }
    /// Ultima measures the rate of change of the option vomma with respect to changes in the volatility of the underlying asset.
    fn ultima<T: Option>(&self, option: T) -> f64 {
        0.0 // Placeholder value
    }
}
