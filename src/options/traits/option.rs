use crate::options::types::*;
use crate::options::Instrument;
use std::any::Any;

use super::OptionPricing;

/// Supertrait that combines OptionPricing and Greeks.
pub trait Option: Clone + Send + Sync {
    /// Get the underlying instrument of the option.
    ///
    /// # Returns
    ///
    /// The underlying instrument of the option.
    fn instrument(&self) -> &Instrument;

    /// Get the underlying instrument of the option (mutable).
    ///
    /// # Returns
    ///
    /// The underlying instrument of the option.
    fn instrument_mut(&mut self) -> &mut Instrument;

    /// Set the underlying instrument of the option.
    ///
    /// # Arguments
    ///
    /// * `instrument` - The underlying instrument.
    fn set_instrument(&mut self, instrument: Instrument);

    /// Get the strike price of the option.
    ///
    /// # Returns
    ///
    /// The strike price of the option.
    fn strike(&self) -> f64;

    /// Time horizon (in years).
    ///
    /// # Returns
    ///
    /// The time horizon (in years).
    fn time_to_maturity(&self) -> f64;

    /// Get the expiration dates of the option.
    ///     
    /// # Returns
    ///
    /// The expiration dates of the option. (Only for Bermudan options)
    fn expiration_dates(&self) -> std::option::Option<&Vec<f64>> {
        None
    }

    /// Set the time horizon (in years).
    ///
    /// # Arguments
    ///
    /// * `time_to_maturity` - The time horizon (in years).
    fn set_time_to_maturity(&mut self, time_to_maturity: f64);

    /// Get the type of the option.
    ///
    /// # Returns
    ///
    /// The type of the option.
    fn option_type(&self) -> OptionType;

    /// Get the style of the option.
    ///
    /// # Returns
    ///
    /// The style of the option.
    fn style(&self) -> &OptionStyle;

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
        let spot_price = spot.unwrap_or_else(|| self.instrument().spot());
        match self.option_type() {
            OptionType::Call => (spot_price - self.strike()).max(0.0),
            OptionType::Put => (self.strike() - spot_price).max(0.0),
        }
    }

    /// Calculate the price of the option.
    ///
    /// # Arguments
    ///
    /// * `model` - The pricing model.
    ///
    /// # Returns
    ///
    /// The price of the option.
    fn price<T: OptionPricing>(&self, model: T) -> f64 {
        model.price(self)
    }

    /// Calculate the time value of the option.
    ///
    /// # Arguments
    ///
    /// * `spot` - The price of the underlying asset.
    /// * `model` - The pricing model.
    ///
    /// # Returns
    ///
    /// The time value of the option.
    fn time_value<T: OptionPricing>(&self, model: T) -> f64 {
        model.price(self) - self.payoff(None)
    }

    /// Return the option as a call.
    ///
    /// # Returns
    ///
    /// The option as a call.
    fn as_call(&self) -> Self {
        if self.is_call() {
            self.clone()
        } else {
            self.flip()
        }
    }

    /// Return the option as a put.
    ///
    /// # Returns
    ///
    /// The option as a put.
    fn as_put(&self) -> Self {
        if self.is_put() {
            self.clone()
        } else {
            self.flip()
        }
    }

    /// Check if the option is a call.
    ///
    /// # Returns
    ///
    /// True if the option is a call, false otherwise.
    fn is_call(&self) -> bool {
        matches!(self.option_type(), OptionType::Call)
    }

    /// Check if the option is a put.
    ///
    /// # Returns
    ///
    /// True if the option is a put, false otherwise.
    fn is_put(&self) -> bool {
        matches!(self.option_type(), OptionType::Put)
    }

    /// Check if the option is ATM
    ///
    /// # Returns
    ///
    /// True if the option is ATM, false otherwise.
    fn atm(&self) -> bool {
        match self.option_type() {
            OptionType::Call => self.instrument().spot() == self.strike(),
            OptionType::Put => self.instrument().spot() == self.strike(),
        }
    }

    /// Check if the option is ITM
    ///
    /// # Returns
    ///
    /// True if the option is ITM, false otherwise.
    fn itm(&self) -> bool {
        match self.option_type() {
            OptionType::Call => self.instrument().spot() > self.strike(),
            OptionType::Put => self.instrument().spot() < self.strike(),
        }
    }

    /// Check if the option is OTM
    ///
    /// # Returns
    ///
    /// True if the option is OTM, false otherwise.
    fn otm(&self) -> bool {
        match self.option_type() {
            OptionType::Call => self.instrument().spot() < self.strike(),
            OptionType::Put => self.instrument().spot() > self.strike(),
        }
    }

    /// Get the option as a trait object.
    ///
    /// # Returns
    ///
    /// The option as a trait object.
    fn as_any(&self) -> &dyn Any;
}
