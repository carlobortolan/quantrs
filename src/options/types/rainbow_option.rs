//! Module for Rainbow option type.
//!
//! A Rainbow option is a type of option whose payoff depends on the performance of multiple underlying assets.
//! The payoff of a Rainbow option can be based on the best or worst performing asset, the average performance of the assets, or other combinations.
//!
//! ## Rainbow Types
//! - `BestOf`: Pays the maximum return of the underlying assets.
//! - `WorstOf`: Pays the minimum return of the underlying assets.
//! - `CallOnMax`: Pays the difference between the maximum return and the strike price.
//! - `CallOnMin`: Pays the difference between the minimum return and the strike price.
//! - `PutOnMax`: Pays the difference between the strike price and the maximum return.
//! - `PutOnMin`: Pays the difference between the strike price and the minimum return.
//! - `CallOnAvg`: Pays the difference between the average return and the strike price.
//! - `PutOnAvg`: Pays the difference between the strike price and the average return.
//! - `AllITM`: Pays the sum of the returns if all assets are in-the-money.
//! - `AllOTM`: Pays the sum of the returns if all assets are out-of-the-money.
//!
//! ## References
//!
//! - [Wikipedia - Rainbow option](https://en.wikipedia.org/wiki/Rainbow_option)
//! - [FiNcyclopedia](https://fincyclopedia.net/derivatives/r/rainbow-option)
//! - [Investopedia - Rainbow option](https://www.investopedia.com/terms/r/rainbowoption.asp)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{Instrument, Option, RainbowOption, OptionType, RainbowType};
//!
//! let asset1 = Instrument::new().with_spot(100.0);
//! let asset2 = Instrument::new().with_spot(110.0);
//! let asset3 = Instrument::new().with_spot(90.0);
//!
//! let instrument = Instrument::new()
//!    .with_assets(vec![(asset1.clone()), (asset2.clone()), (asset3.clone())]);
//!    
//! let best_of = RainbowOption::best_of(instrument.clone(), 105.0, 1.0);
//! let worst_of = RainbowOption::worst_of(instrument.clone(), 105.0, 1.0);
//! let call_on_avg = RainbowOption::call_on_avg(instrument.clone(), 100.0, 1.0);
//! let put_on_avg = RainbowOption::put_on_avg(instrument.clone(), 110.0, 1.0);
//! let all_itm = RainbowOption::all_itm(instrument.clone(), 105.0, 1.0);
//! let all_otm = RainbowOption::all_otm(instrument.clone(), 105.0, 1.0);
//! let call_on_max = RainbowOption::call_on_max(instrument.clone(), 105.0, 1.0);
//! let call_on_min = RainbowOption::call_on_min(instrument.clone(), 80.0, 1.0);
//! let put_on_max = RainbowOption::put_on_max(instrument.clone(), 120.0, 1.0);
//! let put_on_min = RainbowOption::put_on_min(instrument.clone(), 105.0, 1.0);
//!
//! println!("Best-Of Payoff: {}", best_of.payoff(None)); // should be 115.0
//! println!("Worst-Of Payoff: {}", worst_of.payoff(None)); // should be 86.0
//! println!("Call-On-Avg Payoff: {}", call_on_avg.payoff(None)); // should be 1.6
//! println!("Put-On-Avg Payoff: {}", put_on_avg.payoff(None)); // should be 8.3
//! println!("All ITM Payoff: {}", all_itm.payoff(None)); // should be 0.0
//! println!("All OTM Payoff: {}", all_otm.payoff(None)); // should be 0.0
//! println!("Call-On-Max Payoff: {}", call_on_max.payoff(None)); // should be 10.0
//! println!("Call-On-Min Payoff: {}", call_on_min.payoff(None)); // should be 6.0
//! println!("Put-On-Max Payoff: {}", put_on_max.payoff(None)); // should be 5.0
//! println!("Put-On-Min Payoff: {}", put_on_min.payoff(None)); // should be 19.0
//! ```

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
    /// The time horizon (in years).
    pub time_to_maturity: f64,
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
        time_to_maturity: f64,
        option_type: OptionType,
        rainbow_option_type: RainbowType,
    ) -> Self {
        Self {
            instrument,
            strike,
            time_to_maturity,
            option_type,
            option_style: OptionStyle::Rainbow(rainbow_option_type),
        }
    }

    /// Create a new `BestOf` Rainbow option.
    pub fn best_of(instrument: Instrument, strike: f64, time_to_maturity: f64) -> Self {
        Self::new(
            instrument,
            strike,
            time_to_maturity,
            OptionType::Call,
            BestOf,
        )
    }

    /// Create a new `WorstOf` Rainbow option.
    pub fn worst_of(instrument: Instrument, strike: f64, ttm: f64) -> Self {
        Self::new(instrument, strike, ttm, OptionType::Call, WorstOf)
    }

    /// Create a new `CallOnMax` Rainbow option.
    pub fn call_on_max(instrument: Instrument, strike: f64, ttm: f64) -> Self {
        Self::new(instrument, strike, ttm, OptionType::Call, CallOnMax)
    }

    /// Create a new `CallOnMin` Rainbow option.
    pub fn call_on_min(instrument: Instrument, strike: f64, ttm: f64) -> Self {
        Self::new(instrument, strike, ttm, OptionType::Call, CallOnMin)
    }

    /// Create a new `PutOnMax` Rainbow option.
    pub fn put_on_max(instrument: Instrument, strike: f64, ttm: f64) -> Self {
        Self::new(instrument, strike, ttm, OptionType::Put, PutOnMax)
    }

    /// Create a new `PutOnMin` Rainbow option.
    pub fn put_on_min(instrument: Instrument, strike: f64, ttm: f64) -> Self {
        Self::new(instrument, strike, ttm, OptionType::Put, PutOnMin)
    }

    /// Create a new `CallOnAvg` Rainbow option.
    pub fn call_on_avg(instrument: Instrument, strike: f64, ttm: f64) -> Self {
        Self::new(instrument, strike, ttm, OptionType::Call, CallOnAvg)
    }

    /// Create a new `PutOnAvg` Rainbow option.
    pub fn put_on_avg(instrument: Instrument, strike: f64, ttm: f64) -> Self {
        Self::new(instrument, strike, ttm, OptionType::Put, PutOnAvg)
    }

    /// Create a new `AllITM` Rainbow option.
    pub fn all_itm(instrument: Instrument, strike: f64, ttm: f64) -> Self {
        Self::new(instrument, strike, ttm, OptionType::Call, AllITM)
    }

    /// Create a new `AllOTM` Rainbow option.
    pub fn all_otm(instrument: Instrument, strike: f64, ttm: f64) -> Self {
        Self::new(instrument, strike, ttm, OptionType::Put, AllOTM)
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
    fn instrument(&self) -> &Instrument {
        match self.rainbow_option_type() {
            BestOf | CallOnMax | PutOnMax => self.instrument.best_performer(),
            WorstOf | CallOnMin | PutOnMin => self.instrument.worst_performer(),
            _ => &self.instrument,
        }
    }

    fn instrument_mut(&mut self) -> &mut Instrument {
        match self.rainbow_option_type() {
            BestOf | CallOnMax | PutOnMax => self.instrument.best_performer_mut(),
            WorstOf | CallOnMin | PutOnMin => self.instrument.worst_performer_mut(),
            _ => &mut self.instrument,
        }
    }

    fn set_instrument(&mut self, instrument: Instrument) {
        self.instrument = instrument;
    }

    fn strike(&self) -> f64 {
        self.strike
    }

    fn time_to_maturity(&self) -> f64 {
        self.time_to_maturity
    }

    fn set_time_to_maturity(&mut self, time_to_maturity: f64) {
        self.time_to_maturity = time_to_maturity;
    }

    fn option_type(&self) -> OptionType {
        self.option_type
    }

    fn style(&self) -> &OptionStyle {
        &self.option_style
    }

    fn flip(&self) -> Self {
        let flipped_option_type = match self.option_type {
            OptionType::Call => OptionType::Put,
            OptionType::Put => OptionType::Call,
        };

        let flipped_option_style = match self.rainbow_option_type() {
            BestOf => BestOf,
            WorstOf => WorstOf,
            CallOnMax => PutOnMax,
            CallOnMin => PutOnMin,
            PutOnMax => CallOnMax,
            PutOnMin => CallOnMin,
            CallOnAvg => PutOnAvg,
            PutOnAvg => CallOnAvg,
            AllITM => AllOTM,
            AllOTM => AllITM,
        };

        RainbowOption::new(
            self.instrument.clone(),
            self.strike,
            self.time_to_maturity,
            flipped_option_type,
            flipped_option_style,
        )
    }

    fn payoff(&self, spot: std::option::Option<f64>) -> f64 {
        let spot_price: f64 = spot.unwrap_or_else(|| self.instrument().spot());

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
                if self.instrument().worst_performer().spot() > self.strike {
                    spot_price
                } else {
                    0.0
                }
            }
            AllOTM => {
                if self.instrument().best_performer().spot() < self.strike {
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
