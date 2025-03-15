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

pub mod american_option;
pub mod european_option;

pub use american_option::AmericanOption;
pub use european_option::EuropeanOption;

/// Enum representing the type of option.
#[derive(Clone, Copy, Debug)]
pub enum OptionType {
    /// Call option (gives the holder the right to buy the underlying asset)
    Call,
    /// Put option (gives the holder the right to sell the underlying asset)
    Put,
}

/// Enum representing the style of the option.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptionStyle {
    /// American option (can be exercised at any time)
    American,
    /// European option (default, can be exercised only at expiration)
    European,
    /// Bermudan option (can be exercised at specific dates)
    Bermudan,
    /// Asian option (payoff depends on average price of underlying asset)
    Asian,
    /// Barrier option (payoff depends on whether underlying asset crosses a barrier)
    Barrier,
    /// Binary option (payout is fixed amount or nothing)
    Binary,
    /// Digital option (payout is fixed amount or nothing; also known as cash-or-nothing or asset-or-nothing option)
    Digital,
    /// Lookback option (payoff depends on extrema of underlying asset)
    Lookback,
}

impl Default for OptionStyle {
    /// Default option style is European.
    ///
    /// # Returns
    ///
    /// The default option style.
    fn default() -> Self {
        OptionStyle::European
    }
}
