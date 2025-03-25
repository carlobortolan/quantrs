//! Module for Black76 option pricing model.
//! Assumes constant risk-free interest rate r and the futures price F(t) of a particular underlying is log-normal with constant volatility σ.

use crate::options::{
    types::BinaryType::{AssetOrNothing, CashOrNothing},
    Instrument, Option, OptionGreeks, OptionPricing, OptionStrategy, OptionStyle, OptionType,
};
use statrs::distribution::{Continuous, ContinuousCDF, Normal};

/// Black76 option pricing model.
#[derive(Debug, Default)]
pub struct Black76Model {
    /// Risk-free interest rate (e.g., 0.05 for 5%).
    pub risk_free_rate: f64,
    /// Volatility of the underlying asset (e.g., 0.2 for 20%).
    pub volatility: f64,
}

impl Black76Model {
    /// Create a new `Black76Model`.
    ///
    /// # Arguments
    ///
    /// * `risk_free_rate` - Risk-free interest rate (e.g., 0.05 for 5%).
    /// * `volatility` - Volatility of the underlying asset (e.g., 0.2 for 20%).
    ///
    /// # Returns
    ///
    /// A new `Black76Model`.
    pub fn new(risk_free_rate: f64, volatility: f64) -> Self {
        Self {
            risk_free_rate,
            volatility,
        }
    }

    /// Calculate d1 and d2 for the Black-76 formula.
    ///
    /// # Arguments
    ///
    /// * `instrument` - The instrument to calculate d1 and d2 for.
    /// * `strike` - The strike price of the option.
    /// * `ttm` - Time to maturity of the option.
    ///
    /// # Returns
    ///
    /// A tuple containing d1 and d2.
    fn calculate_d1_d2(&self, instrument: &Instrument, strike: f64, ttm: f64) -> (f64, f64) {
        let sqrt_t = ttm.sqrt();
        let n_dividends = instrument
            .dividend_times
            .iter()
            .filter(|&&t| t <= ttm)
            .count() as f64;
        let adjusted_f =
            instrument.spot * (1.0 - instrument.discrete_dividend_yield).powf(n_dividends);

        let d1 = ((adjusted_f / strike).ln() + (0.5 * self.volatility.powi(2)) * ttm)
            / (self.volatility * sqrt_t);

        let d2 = d1 - self.volatility * sqrt_t;

        (d1, d2)
    }

    /// Calculate the price of a European call option using the Black-76 formula.
    ///
    /// # Arguments
    ///
    /// * `instrument` - The instrument to calculate the option price for.
    /// * `strike` - The strike price of the option.
    /// * `ttm` - Time to maturity of the option.
    /// * `normal` - A normal distribution.
    ///
    /// # Returns
    ///
    /// The price of the European call option.
    fn price_euro_call(
        &self,
        instrument: &Instrument,
        strike: f64,
        ttm: f64,
        normal: &Normal,
    ) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(instrument, strike, ttm);

        (-self.risk_free_rate * ttm).exp()
            * (instrument.spot * normal.cdf(d1) - strike * normal.cdf(d2))
    }

    /// Calculate the price of a European put option using the Black-76 formula.
    ///
    /// # Arguments
    ///
    /// * `instrument` - The instrument to calculate the option price for.
    /// * `strike` - The strike price of the option.
    /// * `ttm` - Time to maturity of the option.
    /// * `normal` - A normal distribution.
    ///
    /// # Returns
    ///
    /// The price of the European put option.
    fn price_euro_put(
        &self,
        instrument: &Instrument,
        strike: f64,
        ttm: f64,
        normal: &Normal,
    ) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(instrument, strike, ttm);

        (-self.risk_free_rate * ttm).exp()
            * (strike * normal.cdf(-d2) - instrument.spot * normal.cdf(-d1))
    }
}

impl OptionPricing for Black76Model {
    #[rustfmt::skip]
    fn price<T: Option>(&self, option: &T) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        match (option.option_type(), option.style()) {
            (OptionType::Call, OptionStyle::European) => self.price_euro_call(option.instrument(), option.strike(),option.time_to_maturity(), &normal),
            (OptionType::Put, OptionStyle::European) => self.price_euro_put(option.instrument(), option.strike(), option.time_to_maturity(),&normal),
            _ => panic!("Black76Model does not support this option type or style"),
        }
    }

    fn implied_volatility<T: Option>(&self, _option: &T, _market_price: f64) -> f64 {
        panic!("Black76Model does not support implied volatility calculation yet");
    }
}

impl OptionGreeks for Black76Model {
    fn delta<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support delta calculation yet");
    }

    fn gamma<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support gamma calculation yet");
    }

    fn theta<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support theta calculation yet");
    }

    fn vega<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support vega calculation yet");
    }

    fn rho<T: Option>(&self, option: &T) -> f64 {
        panic!("Black76Model does not support rho calculation yet");
    }
}

impl OptionStrategy for Black76Model {}
