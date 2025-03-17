//! Module for Rainbow option type.

use super::{OptionStyle, OptionType, RainbowType};
use crate::options::{Instrument, Option};
use std::any::Any;

/// A struct representing a Rainbow option.
#[derive(Clone, Debug)]
pub struct RainbowOption {
    /// The underlying instrument.
    pub instrument: Instrument,
    /// Strike price of the option (aka exercise price).
    pub strike: f64,
    /// Type of the option (Call or Put).
    pub option_type: OptionType,
    /// Style of the option (Rainbow with specific type).
    pub option_style: OptionStyle,
}

impl RainbowOption {
    /// Create a new `RainbowOption`.
    pub fn new(
        instrument: Instrument,
        strike: f64,
        option_type: OptionType,
        rainbow_option_type: RainbowType,
    ) -> Self {
        Self {
            instrument,
            strike,
            option_type,
            option_style: OptionStyle::Rainbow(rainbow_option_type),
        }
    }

    /// Create a new `BestOf` Rainbow option.
    pub fn best_of(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Call, RainbowType::BestOf)
    }

    /// Create a new `WorstOf` Rainbow option.
    pub fn worst_of(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Call, RainbowType::WorstOf)
    }

    /// Create a new `CallOnMax` Rainbow option.
    pub fn call_on_max(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Call, RainbowType::CallOnMax)
    }

    /// Create a new `CallOnMin` Rainbow option.
    pub fn call_on_min(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Call, RainbowType::CallOnMin)
    }

    /// Create a new `PutOnMax` Rainbow option.
    pub fn put_on_max(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Put, RainbowType::PutOnMax)
    }

    /// Create a new `PutOnMin` Rainbow option.
    pub fn put_on_min(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Put, RainbowType::PutOnMin)
    }

    /// Get the Rainbow option type.
    pub fn rainbow_option_type(&self) -> &RainbowType {
        if let OptionStyle::Rainbow(ref rainbow_option_type) = self.option_style {
            rainbow_option_type
        } else {
            panic!("Not a rainbow option")
        }
    }

    /// Calculate the payoff of the Rainbow option.
    #[rustfmt::skip]
    pub fn payoff(&self) -> f64 {
        let asset_prices: Vec<f64> = self
            .instrument
            .assets
            .iter()
            .map(|(asset, _)| asset.spot)
            .collect();
        match self.rainbow_option_type() {
            RainbowType::BestOf => asset_prices.iter().cloned().fold(self.strike, f64::max),
            RainbowType::WorstOf => asset_prices.iter().cloned().fold(self.strike, f64::min),
            RainbowType::CallOnMax => (asset_prices.iter().cloned().fold(f64::MIN, f64::max) - self.strike).max(0.0),
            RainbowType::CallOnMin => (asset_prices.iter().cloned().fold(f64::MAX, f64::min) - self.strike).max(0.0),
            RainbowType::PutOnMax => (self.strike - asset_prices.iter().cloned().fold(f64::MIN, f64::max)).max(0.0),
            RainbowType::PutOnMin => (self.strike - asset_prices.iter().cloned().fold(f64::MAX, f64::min)).max(0.0),
        }
    }
}

impl Option for RainbowOption {
    fn style(&self) -> &OptionStyle {
        &self.option_style
    }

    fn instrument(&self) -> &Instrument {
        &self.instrument
    }

    fn strike(&self) -> f64 {
        self.strike
    }

    fn option_type(&self) -> OptionType {
        self.option_type
    }

    fn flip(&self) -> Self {
        let flipped_option_type = match self.option_type {
            OptionType::Call => OptionType::Put,
            OptionType::Put => OptionType::Call,
        };
        RainbowOption::new(
            self.instrument.clone(),
            self.strike,
            flipped_option_type,
            *self.rainbow_option_type(),
        )
    }

    fn payoff(&self, spot: std::option::Option<f64>) -> f64 {
        let asset_prices: Vec<f64> = self
            .instrument
            .assets
            .iter()
            .map(|(asset, _)| asset.spot)
            .collect();
        match self.rainbow_option_type() {
            RainbowType::BestOf => asset_prices.iter().cloned().fold(self.strike, f64::max),
            RainbowType::WorstOf => asset_prices.iter().cloned().fold(self.strike, f64::min),
            RainbowType::CallOnMax => {
                (asset_prices.iter().cloned().fold(f64::MIN, f64::max) - self.strike).max(0.0)
            }
            RainbowType::CallOnMin => {
                (asset_prices.iter().cloned().fold(f64::MAX, f64::min) - self.strike).max(0.0)
            }
            RainbowType::PutOnMax => {
                (self.strike - asset_prices.iter().cloned().fold(f64::MIN, f64::max)).max(0.0)
            }
            RainbowType::PutOnMin => {
                (self.strike - asset_prices.iter().cloned().fold(f64::MAX, f64::min)).max(0.0)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
