use crate::options::{types::Permutation, Instrument, Option, OptionStyle, OptionType};

#[derive(Clone, Debug)]
pub struct LookbackOption {
    pub instrument: Instrument,
    pub time_to_maturity: f64,
    pub option_type: OptionType,
    pub option_style: OptionStyle,
    pub lookback_type: Permutation,
}

impl LookbackOption {
    /// Create a new `LookbackOption`.
    pub fn new(
        instrument: Instrument,
        time_to_maturity: f64,
        option_type: OptionType,
        lookback_type: Permutation,
    ) -> Self {
        Self {
            instrument,
            time_to_maturity,
            option_type,
            option_style: OptionStyle::Lookback(lookback_type),
            lookback_type,
        }
    }

    /// Create a new `Fixed` lookback option.
    pub fn fixed(instrument: Instrument, ttm: f64, option_type: OptionType) -> Self {
        Self::new(instrument, ttm, option_type, Permutation::Fixed)
    }

    /// Create a new `Floating` lookback option.
    pub fn floating(instrument: Instrument, ttm: f64, option_type: OptionType) -> Self {
        Self::new(instrument, ttm, option_type, Permutation::Floating)
    }
}

impl Option for LookbackOption {
    fn instrument(&self) -> &Instrument {
        &self.instrument
    }

    fn set_instrument(&mut self, instrument: Instrument) {
        self.instrument = instrument;
    }

    fn time_to_maturity(&self) -> f64 {
        self.time_to_maturity
    }

    fn strike(&self) -> f64 {
        self.instrument.spot
    }

    fn option_type(&self) -> OptionType {
        self.option_type
    }

    fn style(&self) -> &OptionStyle {
        &self.option_style
    }

    fn payoff(&self, spot: std::option::Option<f64>) -> f64 {
        let spot_price = spot.unwrap_or(self.instrument.spot);
        match self.lookback_type {
            Permutation::Fixed => match self.option_type {
                OptionType::Call => (spot_price - self.strike()).max(0.0),
                OptionType::Put => (self.strike() - spot_price).max(0.0),
            },
            Permutation::Floating => match self.option_type {
                OptionType::Call => (spot_price - self.instrument.min_spot).max(0.0),
                OptionType::Put => (self.instrument.max_spot - spot_price).max(0.0),
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
            self.time_to_maturity,
            flipped_option_type,
            self.lookback_type,
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
