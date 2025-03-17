//! Module for calculating the price and Greeks of various types of options.
//!
//! ## Supported models
//!
//! - [Black-Scholes Option Pricing Model](models/black_scholes/struct.BlackScholesModel.html)
//! - [Binomial Option Pricing Model](models/binomial_tree/struct.BinomialTreeModel.html)
//! - [Monte Carlo Option Pricing Model](models/monte_carlo/struct.MonteCarloModel.html)
//!
//! ## Greek calculations
//!
//! This module also provides implementations of the Greeks for each option pricing model.
//! See the [Greeks](trait.Greeks.html) trait for more information.

pub mod greeks;
pub mod instrument;
pub mod models;
pub mod types;

use std::any::Any;

pub use greeks::*;
pub use instrument::*;
pub use models::*;
pub use types::*;

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
    /// * `spot` - The price of the underlying asset at maturity (optional).
    ///
    /// # Returns
    ///
    /// The payoff of the option.
    fn payoff(&self, spot: std::option::Option<f64>) -> f64 {
        let spot_price = spot.unwrap_or_else(|| self.instrument().spot);
        match self.option_type() {
            OptionType::Call => (spot_price - self.strike()).max(0.0),
            OptionType::Put => (self.strike() - spot_price).max(0.0),
        }
    }

    /// Get the option as a trait object.
    ///
    /// # Returns
    ///
    /// The option as a trait object.
    fn as_any(&self) -> &dyn Any;
}

/// Trait for option pricing models.
pub trait OptionPricing {
    /// Calculate the option price.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to price.
    ///
    /// # Returns
    ///
    /// The price of the option.
    fn price<T: Option>(&self, option: &T) -> f64;

    /// Calculate the implied volatility for a given market price.
    ///
    /// # Arguments
    ///
    /// * `option` - The option for which to calculate the implied volatility.
    /// * `market_price` - The market price of the option.
    ///
    /// # Returns
    ///
    /// The implied volatility.
    fn implied_volatility<T: Option>(&self, option: &T, market_price: f64) -> f64;
}

/// Trait for calculating the Greeks of an option.
pub trait Greeks {
    // First-order Greeks
    /// Delta measures the rate of change of the option price with respect to changes in the price of the underlying asset.
    fn delta<T: Option>(&self, option: T) -> f64 {
        panic!("Delta not implemented for this model");
    }
    /// Vega measures the rate of change of the option price with respect to changes in the volatility of the underlying asset.
    fn vega<T: Option>(&self, option: T) -> f64 {
        panic!("Vega not implemented for this model");
    }
    /// Theta measures the rate of change of the option price with respect to changes in time to maturity.
    fn theta<T: Option>(&self, option: T) -> f64 {
        panic!("Theta not implemented for this model");
    }
    /// Rho measures the rate of change of the option price with respect to changes in the risk-free interest rate.
    fn rho<T: Option>(&self, option: T) -> f64 {
        panic!("Rho not implemented for this model");
    }
    /// Lambda measures the rate of change of the option delta with respect to changes in the risk-free interest rate.
    fn lambda<T: Option>(&self, option: T) -> f64 {
        panic!("Lambda not implemented for this model");
    }
    /// Epsilon measures the rate of change of the option delta with respect to changes in the dividend yield.
    fn epsilon<T: Option>(&self, option: T) -> f64 {
        panic!("Epsilon not implemented for this model");
    }

    // Second-order Greeks
    /// Gamma measures the rate of change of the option delta with respect to changes in the price of the underlying asset.
    fn gamma<T: Option>(&self, option: T) -> f64 {
        panic!("Gamma not implemented for this model");
    }

    /// Vanna measures the rate of change of the option delta with respect to changes in the volatility of the underlying asset.
    fn vanna<T: Option>(&self, option: T) -> f64 {
        panic!("Vanna not implemented for this model");
    }
    /// Charm measures the rate of change of the option delta with respect to changes in time to maturity.
    fn charm<T: Option>(&self, option: T) -> f64 {
        panic!("Charm not implemented for this model");
    }
    /// Vomma measures the rate of change of the option vega with respect to changes in the volatility of the underlying asset.
    fn vomma<T: Option>(&self, option: T) -> f64 {
        panic!("Vomma not implemented for this model");
    }
    /// Veta measures the rate of change of the option vega with respect to changes in time to maturity.
    fn veta<T: Option>(&self, option: T) -> f64 {
        panic!("Veta not implemented for this model");
    }
    /// Vera measures the rate of change of the option gamma with respect to changes in the volatility of the underlying asset.
    fn vera<T: Option>(&self, option: T) -> f64 {
        panic!("Vera not implemented for this model");
    }

    // Third-order Greeks
    /// Speed measures the rate of change of the option gamma with respect to changes in the price of the underlying asset.
    fn speed<T: Option>(&self, option: T) -> f64 {
        panic!("Speed not implemented for this model");
    }
    /// Zomma measures the rate of change of the option gamma with respect to changes in the volatility of the underlying asset.
    fn zomma<T: Option>(&self, option: T) -> f64 {
        panic!("Zomma not implemented for this model");
    }
    /// Color measures the rate of change of the option gamma with respect to changes in time to maturity.
    fn color<T: Option>(&self, option: T) -> f64 {
        panic!("Color not implemented for this model");
    }
    /// Ultima measures the rate of change of the option vomma with respect to changes in the volatility of the underlying asset.
    fn ultima<T: Option>(&self, option: T) -> f64 {
        panic!("Ultima not implemented for this model");
    }

    /// Parmicharma measures the rate of change of charm over the passage of time.
    fn parmicharma<T: Option>(&self, option: T) -> f64 {
        panic!("Parmicharma not implemented for this model");
    }
}
