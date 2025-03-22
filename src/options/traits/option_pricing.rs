//! Traits for option pricing models.

use crate::options::Option;

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
