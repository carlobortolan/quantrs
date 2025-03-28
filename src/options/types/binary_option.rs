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
//! - **Binary Option Type**: Specifies whether the option is Asset-or-Nothing or Cash-or-Nothing.
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{Option, BinaryOption, Instrument, OptionType};
//!
//! let instrument = Instrument::new().with_spot(100.0);
//! let option = BinaryOption::cash_or_nothing(instrument, 100.0, 1.0, OptionType::Call);
//!
//! println!("Option type: {:?}", option.option_type());
//! println!("Strike price: {}", option.strike());
//! println!("Option style: {:?}", option.style());
//! println!("Binary option type: {:?}", option.binary_option_type());
//! ```

use std::any::Any;

use super::{BinaryType, OptionStyle, OptionType};
use crate::options::{Instrument, Option};

/// A struct representing a Binary option.
#[derive(Clone, Debug)]
pub struct BinaryOption {
    /// The underlying instrument.
    pub instrument: Instrument,
    /// Strike price of the option (aka exercise price).
    pub strike: f64,
    /// The time horizon (in years).
    pub time_to_maturity: f64,
    /// Type of the option (Call or Put).
    pub option_type: OptionType,
    /// Style of the option (Binary with specific type).
    pub option_style: OptionStyle,
}

impl BinaryOption {
    /// Create a new `BinaryOption`.
    pub fn new(
        instrument: Instrument,
        strike: f64,
        time_to_maturity: f64,
        option_type: OptionType,
        binary_option_type: BinaryType,
    ) -> Self {
        Self {
            instrument,
            strike,
            time_to_maturity,
            option_type,
            option_style: OptionStyle::Binary(binary_option_type),
        }
    }

    /// Create a new `CashOrNothing` binary option.
    pub fn cash_or_nothing(
        instrument: Instrument,
        strike: f64,
        time_to_maturity: f64,
        option_type: OptionType,
    ) -> Self {
        Self::new(
            instrument,
            strike,
            time_to_maturity,
            option_type,
            BinaryType::CashOrNothing,
        )
    }

    /// Create a new `AssetOrNothing` binary option.
    pub fn asset_or_nothing(
        instrument: Instrument,
        strike: f64,
        time_to_maturity: f64,
        option_type: OptionType,
    ) -> Self {
        Self::new(
            instrument,
            strike,
            time_to_maturity,
            option_type,
            BinaryType::AssetOrNothing,
        )
    }

    /// Get the binary option type.
    pub fn binary_option_type(&self) -> &BinaryType {
        if let OptionStyle::Binary(ref binary_option_type) = self.option_style {
            binary_option_type
        } else {
            panic!("Not a binary option")
        }
    }
}

impl Option for BinaryOption {
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
        &self.option_style
    }

    fn flip(&self) -> Self {
        let flipped_option_type = match self.option_type {
            OptionType::Call => OptionType::Put,
            OptionType::Put => OptionType::Call,
        };
        BinaryOption::new(
            self.instrument.clone(),
            self.strike,
            self.time_to_maturity,
            flipped_option_type,
            *self.binary_option_type(),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    #[rustfmt::skip]
    fn payoff(&self, spot: std::option::Option<f64>) -> f64 {
        let spot = spot.unwrap_or(self.instrument.spot());
        match self.binary_option_type() {
            BinaryType::CashOrNothing => match self.option_type {
                OptionType::Call => if spot > self.strike { 1.0 } else { 0.0 },
                OptionType::Put => if spot < self.strike { 1.0 } else { 0.0 },
            },
            BinaryType::AssetOrNothing => match self.option_type {
                OptionType::Call => if spot > self.strike { spot } else { 0.0 },
                OptionType::Put => if spot < self.strike { spot } else { 0.0 },
            },
        }
    }
}
