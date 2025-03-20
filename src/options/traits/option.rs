use crate::options::types::*;
use crate::options::Instrument;
use std::any::Any;

/// Supertrait that combines OptionPricing and Greeks.
pub trait Option: Clone + Send + Sync {
    /// Time horizon (in years).
    ///
    /// # Returns
    ///
    /// The time horizon (in years).
    fn time_to_maturity(&self) -> f64;

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
