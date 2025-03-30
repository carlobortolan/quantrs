//! Module for Barrier option type.

use std::any::Any;

use super::{BarrierType, OptionStyle, OptionType};
use crate::options::{Instrument, Option};

/// A struct representing an Bermudan option.
#[derive(Clone, Debug)]
pub struct BarrierOption {
    /// The underlying instrument.
    pub instrument: Instrument,
    /// Strike price of the option (aka exercise price).
    pub strike: f64,
    /// The barrier price
    pub barrier: f64,
    /// The time horizon (in years).
    pub time_to_maturity: f64,
    /// Type of the option (Call or Put).
    pub option_type: OptionType,
    /// Style of the option (Barrier with specific type).
    pub barrier_type: BarrierType,
}

impl BarrierOption {
    /// Create a new `BarrierOption`.
    pub fn new(
        instrument: Instrument,
        strike: f64,
        barrier: f64,
        time_to_maturity: f64,
        option_type: OptionType,
        barrier_type: BarrierType,
    ) -> Self {
        Self {
            instrument,
            strike,
            barrier,
            time_to_maturity,
            option_type,
            barrier_type,
        }
    }

    pub fn down_and_in(
        instrument: Instrument,
        strike: f64,
        barrier: f64,
        time_to_maturity: f64,
        option_type: OptionType,
    ) -> Self {
        Self::new(
            instrument,
            strike,
            barrier,
            time_to_maturity,
            option_type,
            BarrierType::DownAndIn,
        )
    }

    pub fn down_and_out(
        instrument: Instrument,
        strike: f64,
        barrier: f64,
        time_to_maturity: f64,
        option_type: OptionType,
    ) -> Self {
        Self::new(
            instrument,
            strike,
            barrier,
            time_to_maturity,
            option_type,
            BarrierType::DownAndOut,
        )
    }

    pub fn up_and_in(
        instrument: Instrument,
        strike: f64,
        barrier: f64,
        time_to_maturity: f64,
        option_type: OptionType,
    ) -> Self {
        Self::new(
            instrument,
            strike,
            barrier,
            time_to_maturity,
            option_type,
            BarrierType::UpAndIn,
        )
    }

    pub fn up_and_out(
        instrument: Instrument,
        strike: f64,
        barrier: f64,
        time_to_maturity: f64,
        option_type: OptionType,
    ) -> Self {
        Self::new(
            instrument,
            strike,
            barrier,
            time_to_maturity,
            option_type,
            BarrierType::UpAndOut,
        )
    }

    pub fn is_knocked_out(&self, spot: f64) -> bool {
        match self.barrier_type {
            BarrierType::DownAndOut => spot <= self.barrier,
            BarrierType::UpAndOut => spot >= self.barrier,
            _ => false, // In-options are never knocked out
        }
    }

    pub fn is_activated(&self, path: &[f64]) -> bool {
        match self.barrier_type {
            BarrierType::DownAndIn => path.iter().any(|&s| s <= self.barrier),
            BarrierType::UpAndIn => path.iter().any(|&s| s >= self.barrier),
            _ => true, // Out-options don't require activation
        }
    }
}

impl Option for BarrierOption {
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

    fn style(&self) -> OptionStyle {
        OptionStyle::Barrier(self.barrier_type)
    }

    #[rustfmt::skip]
    fn payoff(&self, terminal: std::option::Option<f64>) -> f64 {
        let terminal = terminal.unwrap_or_else(|| self.instrument.terminal_spot()); 
        let above = self.instrument.spot.iter().any(|&a| a >= self.barrier);
        let below = self.instrument.spot.iter().any(|&a| a <= self.barrier);
        let payoff = match self.option_type {
            OptionType::Call => (terminal - self.strike).max(0.0),
            OptionType::Put => (self.strike - terminal).max(0.0),
        };
    
        match self.barrier_type {
            BarrierType::DownAndIn  => if below { payoff } else { 0.0 },
            BarrierType::DownAndOut => if below { 0.0 } else { payoff },
            BarrierType::UpAndIn    => if above { payoff } else { 0.0 },
            BarrierType::UpAndOut   => if above { 0.0 } else { payoff },
        }
    }

    fn flip(&self) -> Self {
        let flipped_option_type = match self.option_type {
            OptionType::Call => OptionType::Put,
            OptionType::Put => OptionType::Call,
        };
        BarrierOption::new(
            self.instrument.clone(),
            self.strike,
            self.barrier,
            self.time_to_maturity,
            flipped_option_type,
            self.barrier_type,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
