//! Module for European option type.
//!
//! A European option is a type of options contract that can only be exercised at its expiration date.
//! This contrasts with American options, which can be exercised at any time before expiration.
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
//! use quantrs::options::{Option, EuropeanOption, Instrument, OptionType};
//!
//! let instrument = Instrument::new(100.0);
//! let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
//!
//! println!("Option type: {:?}", option.option_type());
//! println!("Strike price: {}", option.strike());
//! println!("Option style: {:?}", option.style());
//! ```

use super::{OptionStyle, OptionType};
use crate::options::{Instrument, Option};

/// A struct representing a European option.
#[derive(Clone, Debug)]
pub struct EuropeanOption {
    /// The underlying instrument.
    pub instrument: Instrument,
    /// Strike price of the option (aka exercise price).
    pub strike: f64,
    /// Type of the option (Call or Put).
    pub option_type: OptionType,
}

impl EuropeanOption {
    /// Create a new `EuropeanOption`.
    pub fn new(instrument: Instrument, strike: f64, option_type: OptionType) -> Self {
        Self {
            instrument,
            strike,
            option_type,
        }
    }
}

impl Option for EuropeanOption {
    fn style(&self) -> &OptionStyle {
        &OptionStyle::European
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
        EuropeanOption::new(self.instrument.clone(), self.strike, flipped_option_type)
    }
}
