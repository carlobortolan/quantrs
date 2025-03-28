//! Module for Black76 option pricing model.
//! Assumes constant risk-free interest rate r and the futures price F(t) of a particular underlying is log-normal with constant volatility σ.
//! https://en.wikipedia.org/wiki/Black_model
//! https://www.glynholton.com/notes/black_1976/

use crate::options::{
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

        let d1 = ((instrument.calculate_adjusted_spot(ttm) / strike).ln()
            + (0.5 * self.volatility.powi(2)) * ttm)
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
            * (instrument.spot() * normal.cdf(d1) - strike * normal.cdf(d2))
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
            * (strike * normal.cdf(-d2) - instrument.spot() * normal.cdf(-d1))
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
        let (d1, d2) = self.calculate_d1_d2(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
        );
        let normal = Normal::new(0.0, 1.0).unwrap();
        match option.style() {
            OptionStyle::European => match option.option_type() {
                OptionType::Call => {
                    (-self.risk_free_rate * option.time_to_maturity()).exp() * normal.cdf(d1)
                }
                OptionType::Put => {
                    (-self.risk_free_rate * option.time_to_maturity()).exp()
                        * (normal.cdf(d1) - 1.0)
                }
            },
            _ => panic!("Unsupported option style for delta calculation"),
        }
    }

    fn gamma<T: Option>(&self, option: &T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
        );
        let adjusted_spot = option
            .instrument()
            .calculate_adjusted_spot(option.time_to_maturity());
        let normal = Normal::new(0.0, 1.0).unwrap();

        match option.style() {
            OptionStyle::European => {
                (-self.risk_free_rate * option.time_to_maturity()).exp() * normal.pdf(d1)
                    / (adjusted_spot * self.volatility * option.time_to_maturity().sqrt())
            }
            _ => panic!("Unsupported option style for gamma calculation"),
        }
    }

    fn theta<T: Option>(&self, option: &T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
        );
        let adjusted_spot = option
            .instrument()
            .calculate_adjusted_spot(option.time_to_maturity());
        let normal = Normal::new(0.0, 1.0).unwrap();

        match option.style() {
            OptionStyle::European => match option.option_type() {
                OptionType::Call => {
                    -adjusted_spot
                        * (-self.risk_free_rate * option.time_to_maturity()).exp()
                        * normal.pdf(d1)
                        * self.volatility
                        / (2.0 * option.time_to_maturity().sqrt())
                        + self.risk_free_rate
                            * adjusted_spot
                            * (-self.risk_free_rate * option.time_to_maturity()).exp()
                            * normal.cdf(d1)
                        - self.risk_free_rate
                            * option.strike()
                            * (-self.risk_free_rate * option.time_to_maturity()).exp()
                            * normal.cdf(d2)
                }
                OptionType::Put => {
                    -adjusted_spot
                        * (-self.risk_free_rate * option.time_to_maturity()).exp()
                        * normal.pdf(d1)
                        * self.volatility
                        / (2.0 * option.time_to_maturity().sqrt())
                        - self.risk_free_rate
                            * adjusted_spot
                            * (-self.risk_free_rate * option.time_to_maturity()).exp()
                            * normal.cdf(-d1)
                        + self.risk_free_rate
                            * option.strike()
                            * (-self.risk_free_rate * option.time_to_maturity()).exp()
                            * normal.cdf(-d2)
                }
            },
            _ => panic!("Unsupported option style for theta calculation"),
        }
    }

    fn vega<T: Option>(&self, option: &T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
        );
        let adjusted_spot = option
            .instrument()
            .calculate_adjusted_spot(option.time_to_maturity());
        let normal = Normal::new(0.0, 1.0).unwrap();

        match option.style() {
            OptionStyle::European => {
                adjusted_spot
                    * (-self.risk_free_rate * option.time_to_maturity()).exp()
                    * normal.pdf(d1)
                    * option.time_to_maturity().sqrt()
            }
            _ => panic!("Unsupported option style for vega calculation"),
        }
    }

    fn rho<T: Option>(&self, option: &T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
        );
        let normal = Normal::new(0.0, 1.0).unwrap();

        match option.style() {
            OptionStyle::European => match option.option_type() {
                OptionType::Call => {
                    -option.time_to_maturity()
                        * self.price_euro_call(
                            option.instrument(),
                            option.strike(),
                            option.time_to_maturity(),
                            &normal,
                        )
                }
                OptionType::Put => {
                    -option.time_to_maturity()
                        * self.price_euro_put(
                            option.instrument(),
                            option.strike(),
                            option.time_to_maturity(),
                            &normal,
                        )
                }
            },
            _ => panic!("Unsupported option style for rho calculation"),
        }
    }
}

impl OptionStrategy for Black76Model {}
