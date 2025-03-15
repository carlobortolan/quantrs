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
