use crate::options::{types::AsianType, Instrument, Option, OptionStyle, OptionType};

#[derive(Clone, Debug)]
pub struct AsianOption {
    pub instrument: Instrument,
    pub strike: f64,
    pub option_type: OptionType,
    pub option_style: OptionStyle,
    pub asian_type: AsianType,
}

impl AsianOption {
    /// Create a new `AsianOption`.
    pub fn new(
        instrument: Instrument,
        strike: f64,
        option_type: OptionType,
        asian_type: AsianType,
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
        Self::new(instrument, strike, option_type, AsianType::Fixed)
    }

    /// Create a new `Floating` Asian option.
    pub fn floating(instrument: Instrument, option_type: OptionType) -> Self {
        Self::new(instrument, 0.0, option_type, AsianType::Floating) // strike is not used for floating
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
            AsianType::Fixed => match self.option_type {
                OptionType::Call => (avg_price - self.strike).max(0.0),
                OptionType::Put => (self.strike - avg_price).max(0.0),
            },
            AsianType::Floating => match self.option_type {
                OptionType::Call => (self.instrument.spot - avg_price).max(0.0),
                OptionType::Put => (avg_price - self.instrument.spot).max(0.0),
            },
            _ => panic!("Unsupported Asian type"),
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
