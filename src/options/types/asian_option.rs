//! Module for Asian option type.
//!
//! An Asian option is a type of options contract where the payoff depends on the average price of the underlying asset over a certain period of time.
//! This averaging feature makes Asian options less volatile and generally cheaper than their European or American counterparts.
//!
//! ## Characteristics
//!
//! - **Underlying Instrument**: The asset on which the option is based.
//! - **Strike Price**: The price at which the option can be exercised (for fixed strike options).
//! - **Option Type**: Specifies whether the option is a call (right to buy) or a put (right to sell).
//! - **Asian Type**: Specifies whether the option is a fixed strike or floating strike.
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{Option, AsianOption, Instrument, OptionType, Permutation};
//!
//! let instrument = Instrument::new().with_spot(100.0);
//! let option = AsianOption::new(instrument, 100.0, OptionType::Call, Permutation::Fixed);
//!
//! println!("Option type: {:?}", option.option_type());
//! println!("Strike price: {}", option.strike());
//! println!("Option style: {:?}", option.style());
//! ```
use crate::options::{types::Permutation, Instrument, Option, OptionStyle, OptionType};

/// A struct representing an Asian option.
#[derive(Clone, Debug)]
pub struct AsianOption {
    pub instrument: Instrument,
    pub strike: f64,
    pub option_type: OptionType,
    pub option_style: OptionStyle,
    pub asian_type: Permutation,
}

impl AsianOption {
    /// Create a new `AsianOption`.
    pub fn new(
        instrument: Instrument,
        strike: f64,
        option_type: OptionType,
        asian_type: Permutation,
    ) -> Self {
        Self {
            instrument,
            strike,
            option_type,
            option_style: OptionStyle::Asian(asian_type),
            asian_type,
        }
    }

    /// Create a new `Fixed` Asian option.
    pub fn fixed(instrument: Instrument, strike: f64, option_type: OptionType) -> Self {
        Self::new(instrument, strike, option_type, Permutation::Fixed)
    }

    /// Create a new `Floating` Asian option.
    pub fn floating(instrument: Instrument, option_type: OptionType) -> Self {
        Self::new(instrument, 0.0, option_type, Permutation::Floating) // strike is not used for floating
    }
}

impl Option for AsianOption {
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

    fn payoff(&self, avg_price: std::option::Option<f64>) -> f64 {
        let avg_price = avg_price.unwrap_or(self.instrument.spot);
        match self.asian_type {
            Permutation::Fixed => match self.option_type {
                OptionType::Call => (avg_price - self.strike).max(0.0),
                OptionType::Put => (self.strike - avg_price).max(0.0),
            },
            Permutation::Floating => match self.option_type {
                // spot is the price at maturity
                OptionType::Call => (self.instrument.spot - avg_price).max(0.0),
                OptionType::Put => (avg_price - self.instrument.spot).max(0.0),
            },
        }
    }

    fn flip(&self) -> Self {
        let flipped_option_type = match self.option_type {
            OptionType::Call => OptionType::Put,
            OptionType::Put => OptionType::Call,
        };
        AsianOption::new(
            self.instrument.clone(),
            self.strike,
            flipped_option_type,
            self.asian_type,
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
