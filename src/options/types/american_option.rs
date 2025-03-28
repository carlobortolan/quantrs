//! Module for American option type.
//!
//! An American option is a type of options contract that can be exercised at any time before its expiration date.
//! This flexibility makes American options more valuable than their European counterparts, which can only be exercised at expiration.
//!
//! ## Characteristics
//!
//! - **Underlying Instrument**: The asset on which the option is based.
//! - **Strike Price**: The price at which the option can be exercised.
//! - **Option Type**: Specifies whether the option is a call (right to buy) or a put (right to sell).
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{Option, AmericanOption, Instrument, OptionType};
//!
//! let instrument = Instrument::new().with_spot(100.0);
//! let option = AmericanOption::new(instrument, 100.0, 1.0, OptionType::Call);
//!
//! println!("Option type: {:?}", option.option_type());
//! println!("Strike price: {}", option.strike());
//! println!("Option style: {:?}", option.style());
//! ```

use std::any::Any;

use super::{OptionStyle, OptionType};
use crate::options::{Instrument, Option};

/// A struct representing an American option.
#[derive(Clone, Debug)]
pub struct AmericanOption {
    /// The underlying instrument.
    pub instrument: Instrument,
    /// Strike price of the option (aka exercise price).
    pub strike: f64,
    /// Type of the option (Call or Put).
    /// The time horizon (in years).
    pub time_to_maturity: f64,
    pub option_type: OptionType,
}

impl AmericanOption {
    /// Create a new `AmericanOption`.
    pub fn new(
        instrument: Instrument,
        strike: f64,
        time_to_maturity: f64,
        option_type: OptionType,
    ) -> Self {
        Self {
            instrument,
            strike,
            time_to_maturity,
            option_type,
        }
    }
}

impl Option for AmericanOption {
    fn instrument(&self) -> &Instrument {
        &self.instrument
    }

    fn instrument_mut(&mut self) -> &mut Instrument {
        &mut self.instrument
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
        &OptionStyle::American
    }

    fn flip(&self) -> Self {
        let flipped_option_type = match self.option_type {
            OptionType::Call => OptionType::Put,
            OptionType::Put => OptionType::Call,
        };
        AmericanOption::new(
            self.instrument.clone(),
            self.strike,
            self.time_to_maturity,
            flipped_option_type,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
