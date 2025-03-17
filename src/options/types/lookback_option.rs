use crate::options::{types::LookbackType, Instrument, Option, OptionStyle, OptionType};

#[derive(Clone, Debug)]
pub struct LookbackOption {
    pub instrument: Instrument,
    pub option_type: OptionType,
    pub option_style: OptionStyle,
    pub lookback_type: LookbackType,
}

impl LookbackOption {
    /// Create a new `LookbackOption`.
    pub fn new(
        instrument: Instrument,
        option_type: OptionType,
        lookback_type: LookbackType,
    ) -> Self {
        Self {
            instrument,
            option_type,
            option_style: OptionStyle::Lookback(lookback_type),
            lookback_type,
        }
    }

    /// Create a new `Fixed` lookback option.
    pub fn fixed(instrument: Instrument, option_type: OptionType) -> Self {
        Self::new(instrument, option_type, LookbackType::Fixed)
    }

    /// Create a new `Floating` lookback option.
    pub fn floating(instrument: Instrument, option_type: OptionType) -> Self {
        Self::new(instrument, option_type, LookbackType::Floating)
    }
}

impl Option for LookbackOption {
    fn style(&self) -> &OptionStyle {
        &self.option_style
    }

    fn instrument(&self) -> &Instrument {
        &self.instrument
    }

    fn strike(&self) -> f64 {
        self.instrument.spot
    }

    fn option_type(&self) -> OptionType {
        self.option_type
    }

    fn payoff(&self, spot: std::option::Option<f64>) -> f64 {
        let spot_price = spot.unwrap_or(self.instrument.spot);
        match self.lookback_type {
            LookbackType::Fixed => match self.option_type {
                OptionType::Call => (spot_price - self.strike()).max(0.0),
                OptionType::Put => (self.strike() - spot_price).max(0.0),
            },
            LookbackType::Floating => match self.option_type {
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
            flipped_option_type,
            self.lookback_type,
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
