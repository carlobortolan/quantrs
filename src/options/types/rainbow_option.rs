//! Module for Rainbow option type.

use super::{OptionStyle, OptionType, RainbowType, RainbowType::*};
use crate::options::{Instrument, Option};
use core::panic;
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
        Self::new(instrument, strike, OptionType::Call, BestOf)
    }

    /// Create a new `WorstOf` Rainbow option.
    pub fn worst_of(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Call, WorstOf)
    }

    /// Create a new `CallOnMax` Rainbow option.
    pub fn call_on_max(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Call, CallOnMax)
    }

    /// Create a new `CallOnMin` Rainbow option.
    pub fn call_on_min(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Call, CallOnMin)
    }

    /// Create a new `PutOnMax` Rainbow option.
    pub fn put_on_max(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Put, PutOnMax)
    }

    /// Create a new `PutOnMin` Rainbow option.
    pub fn put_on_min(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Put, PutOnMin)
    }

    /// Create a new `CallOnAvg` Rainbow option.
    pub fn call_on_avg(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Call, CallOnAvg)
    }

    /// Create a new `PutOnAvg` Rainbow option.
    pub fn put_on_avg(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Put, PutOnAvg)
    }

    /// Create a new `AllITM` Rainbow option.
    pub fn all_itm(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Call, AllITM)
    }

    /// Create a new `AllOTM` Rainbow option.
    pub fn all_otm(instrument: Instrument, strike: f64) -> Self {
        Self::new(instrument, strike, OptionType::Put, AllOTM)
    }

    /// Get the Rainbow option type.
    pub fn rainbow_option_type(&self) -> &RainbowType {
        if let OptionStyle::Rainbow(ref rainbow_option_type) = self.option_style {
            rainbow_option_type
        } else {
            panic!("Not a rainbow option")
        }
    }
}

impl Option for RainbowOption {
    fn style(&self) -> &OptionStyle {
        &self.option_style
    }

    fn instrument(&self) -> &Instrument {
        match self.rainbow_option_type() {
            BestOf | CallOnMax | PutOnMax => self.instrument.best_performer(),
            WorstOf | CallOnMin | PutOnMin => self.instrument.worst_performer(),
            _ => &self.instrument,
        }
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
        let spot_price: f64 = spot.unwrap_or_else(|| self.instrument().spot);

        match self.rainbow_option_type() {
            BestOf => spot_price.max(self.strike),
            WorstOf => spot_price.min(self.strike),
            CallOnMax => (spot_price - self.strike).max(0.0),
            CallOnMin => (spot_price - self.strike).max(0.0),
            PutOnMax => (self.strike - spot_price).max(0.0),
            PutOnMin => (self.strike - spot_price).max(0.0),
            CallOnAvg => (spot_price - self.strike).max(0.0),
            PutOnAvg => (self.strike - spot_price).max(0.0),
            AllITM => {
                if self.instrument().worst_performer().spot > self.strike {
                    spot_price
                } else {
                    0.0
                }
            }
            AllOTM => {
                if self.instrument().best_performer().spot < self.strike {
                    spot_price
                } else {
                    0.0
                }
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
