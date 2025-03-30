//! Module for Bermudan option type.
//!
//! A Bermuda option can be exercised early, but only on a set of specific dates before its expiration.
//! These exercise dates are often set in one-month increments.
//! Premiums for Bermuda options are typically lower than those of American options, which can be exercised any time before expiry.

use std::any::Any;

use super::{OptionStyle, OptionType};
use crate::{
    log_warn,
    options::{Instrument, Option},
};

/// A struct representing an Bermudan option.
#[derive(Clone, Debug)]
pub struct BermudanOption {
    /// The underlying instrument.
    pub instrument: Instrument,
    /// Strike price of the option (aka exercise price).
    pub strike: f64,
    /// The time horizon (in years).
    pub time_to_maturity: f64,
    /// The expiration dates of the option (in years).
    pub expiration_dates: Vec<f64>,
    /// Type of the option (Call or Put).
    pub option_type: OptionType,
}

impl BermudanOption {
    /// Create a new `BermudanOption`.
    pub fn new(
        instrument: Instrument,
        strike: f64,
        expiration_dates: Vec<f64>,
        option_type: OptionType,
    ) -> Self {
        Self {
            instrument,
            strike,
            time_to_maturity: if let Some(&last_date) = expiration_dates.last() {
                last_date
            } else {
                log_warn!("Expiration dates are empty, setting time to maturity to 0.0");
                0.0
            },
            expiration_dates,
            option_type,
        }
    }
}

impl Option for BermudanOption {
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

    fn expiration_dates(&self) -> std::option::Option<&Vec<f64>> {
        Some(&self.expiration_dates)
    }

    fn set_time_to_maturity(&mut self, time_to_maturity: f64) {
        self.time_to_maturity = time_to_maturity;
    }

    fn option_type(&self) -> OptionType {
        self.option_type
    }

    fn style(&self) -> &OptionStyle {
        &OptionStyle::Bermudan
    }

    fn flip(&self) -> Self {
        let flipped_option_type = match self.option_type {
            OptionType::Call => OptionType::Put,
            OptionType::Put => OptionType::Call,
        };
        BermudanOption::new(
            self.instrument.clone(),
            self.strike,
            self.expiration_dates.clone(),
            flipped_option_type,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
