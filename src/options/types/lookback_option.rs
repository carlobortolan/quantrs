//! Module for Lookback option type.

use crate::options::{types::Permutation, Instrument, Option, OptionStyle, OptionType};

/// A struct representing a Lookback option.
#[derive(Clone, Debug)]
pub struct LookbackOption {
    /// The underlying instrument.
    pub instrument: Instrument,
    /// Strike price of the option (aka exercise price).
    pub strike: f64,
    /// The time horizon (in years).
    pub time_to_maturity: f64,
    /// Type of the option (Call or Put).
    pub option_type: OptionType,
    /// Style of the option (Lookback with specific type).
    pub permutation: Permutation,
}

impl LookbackOption {
    /// Create a new `LookbackOption`.
    pub fn new(
        instrument: Instrument,
        strike: f64,
        time_to_maturity: f64,
        option_type: OptionType,
        permutation: Permutation,
    ) -> Self {
        Self {
            instrument,
            time_to_maturity,
            strike,
            option_type,
            permutation,
        }
    }

    /// Create a new `Fixed` lookback option.
    pub fn fixed(
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
            Permutation::Fixed,
        )
    }

    /// Create a new `Floating` lookback option.
    pub fn floating(
        instrument: Instrument,
        time_to_maturity: f64,
        option_type: OptionType,
    ) -> Self {
        Self::new(
            instrument,
            0.0,
            time_to_maturity,
            option_type,
            Permutation::Floating,
        )
    }
}

impl Option for LookbackOption {
    fn instrument(&self) -> &Instrument {
        &self.instrument
    }

    fn instrument_mut(&mut self) -> &mut Instrument {
        &mut self.instrument
    }

    fn set_instrument(&mut self, instrument: Instrument) {
        self.instrument = instrument;
    }

    fn time_to_maturity(&self) -> f64 {
        self.time_to_maturity
    }

    fn set_time_to_maturity(&mut self, time_to_maturity: f64) {
        self.time_to_maturity = time_to_maturity;
    }

    fn strike(&self) -> f64 {
        self.strike
    }

    fn option_type(&self) -> OptionType {
        self.option_type
    }

    fn style(&self) -> OptionStyle {
        OptionStyle::Lookback(self.permutation)
    }

    fn payoff(&self, avg_price: std::option::Option<f64>) -> f64 {
        let avg_price = avg_price.unwrap_or(self.instrument.spot());
        let max_spot = self.instrument.max_spot();

        match self.permutation {
            Permutation::Fixed => match self.option_type {
                OptionType::Call => (self.instrument.max_spot() - self.strike).max(0.0),
                OptionType::Put => (self.strike - self.instrument.min_spot()).max(0.0),
            },
            Permutation::Floating => match self.option_type {
                OptionType::Call => {
                    (self.instrument.terminal_spot() - self.instrument.min_spot()).max(0.0)
                }
                OptionType::Put => {
                    (self.instrument.max_spot() - self.instrument.terminal_spot()).max(0.0)
                }
            },
        }
    }

    fn flip(&self) -> Self {
        let flipped_option_type = match self.option_type {
            OptionType::Call => OptionType::Put,
            OptionType::Put => OptionType::Call,
        };
        LookbackOption::new(
            self.instrument.clone(),
            self.strike,
            self.time_to_maturity,
            flipped_option_type,
            self.permutation,
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
