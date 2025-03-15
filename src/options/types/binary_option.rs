//! Module for Binary option type.
//!
//! A Binary option is a type of options contract where the payoff is either a fixed amount or nothing at all.
//! This type of option is also known as an all-or-nothing option or digital option.
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
//! use quantrs::options::{Option, BinaryOption, Instrument, OptionType};
//!
//! let instrument = Instrument::new(100.0);
//! let option = BinaryOption::new(instrument, 100.0, OptionType::Call);
//!
//! println!("Option type: {:?}", option.option_type());
//! println!("Strike price: {}", option.strike());
//! println!("Option style: {:?}", option.style());
//! ```

use super::{OptionStyle, OptionType};
use crate::options::{Instrument, Option};

/// A struct representing a Binary option.
#[derive(Clone, Debug)]
pub struct BinaryOption {
    /// The underlying instrument.
    pub instrument: Instrument,
    /// Strike price of the option (aka exercise price).
    pub strike: f64,
    /// Type of the option (Call or Put).
    pub option_type: OptionType,
}

impl BinaryOption {
    /// Create a new `BinaryOption`.
    pub fn new(instrument: Instrument, strike: f64, option_type: OptionType) -> Self {
        Self {
            instrument,
            strike,
            option_type,
        }
    }
}

impl Option for BinaryOption {
    fn style(&self) -> &OptionStyle {
        &OptionStyle::Binary
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
        BinaryOption::new(self.instrument.clone(), self.strike, flipped_option_type)
    }
}
