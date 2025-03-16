//! Module for Black-Scholes option pricing model.
//!
//! The Black-Scholes option pricing model is a mathematical model used to calculate the theoretical price of European-style options.
//! The model was developed by Fischer Black, Myron Scholes, and Robert Merton in the early 1970s.
//!
//! The Black-Scholes model makes several assumptions, including:
//! - The option is European-style (can only be exercised at expiration).
//! - The underlying asset follows a log-normal distribution.
//! - There are no transaction costs or taxes.
//! - The risk-free interest rate is constant.
//! - The volatility of the underlying asset is constant.
//! - The returns on the underlying asset are normally distributed.
//!
//! The Black-Scholes model is widely used by options traders to determine the fair price of an option based on various factors,
//! including the current price of the underlying asset, the strike price of the option, the time to expiration, the risk-free interest rate,
//! and the volatility of the underlying asset.
//!
//! ## Formula
//!
//! The price of an option using the Black-Scholes model is calculated as follows:
//!
//! ```text
//! C = S * N(d1) - X * e^(-rT) * N(d2) for a call option
//! P = X * e^(-rT) * N(-d2) - S * N(-d1) for a put option
//! ```
//!
//! where:
//! - `C` is the price of the call option.
//! - `P` is the price of the put option.
//! - `S` is the current price of the underlying asset.
//! - `X` is the strike price of the option.
//! - `r` is the risk-free interest rate.
//! - `T` is the time to maturity.
//! - `N` is the cumulative distribution function of the standard normal distribution.
//! - `d1` and `d2` are calculated as follows:
//!     ```text
//!     d1 = (ln(S / X) + (r + 0.5 * σ^2) * T) / (σ * sqrt(T))
//!     d2 = d1 - σ * sqrt(T)
//!     ```
//! - `σ` is the volatility of the underlying asset.
//!
//! The payoff of the option is calculated as:
//!
//! ```text
//! payoff = max(ST - K, 0) for a call option
//! payoff = max(K - ST, 0) for a put option
//! ```
//!
//! where:
//! - `ST` is the price of the underlying asset at maturity.
//! - `K` is the strike price of the option.
//! - `max` is the maximum function.
//!
//! ## References
//!
//! - [Wikipedia - Black-Scholes model](https://en.wikipedia.org/wiki/Black%E2%80%93Scholes_model)
//! - [Black-Scholes Calculator](https://www.math.drexel.edu/~pg/fin/VanillaCalculator.html)
//! - [Black-Scholes Binary Options](https://quantpie.co.uk/bsm_bin_c_formula/bs_bin_c_summary.php)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{BlackScholesModel, OptionType, OptionPricing, Instrument, OptionStyle, EuropeanOption};
//!
//! let instrument = Instrument::new().with_spot(100.0);
//! let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
//! let model = BlackScholesModel::new(1.0, 0.05, 0.2);
//!
//! let price = model.price(option);
//! println!("Option price: {}", price);
//! ```

use crate::options::{Greeks, Option, OptionPricing, OptionStyle, OptionType};
use statrs::distribution::{Continuous, ContinuousCDF, Normal};

/// A struct representing a Black-Scholes model.
#[derive(Debug, Default)]
pub struct BlackScholesModel {
    /// Time horizon (in years).
    pub time_to_maturity: f64,
    /// Risk-free interest rate (e.g., 0.05 for 5%).
    pub risk_free_rate: f64,
    /// Annualized standard deviation of an asset's continuous returns (e.g., 0.2 for 20%).
    pub volatility: f64,
}

impl BlackScholesModel {
    /// Create a new `BlackScholesModel`.
    pub fn new(time_to_maturity: f64, risk_free_rate: f64, volatility: f64) -> Self {
        Self {
            time_to_maturity,
            risk_free_rate,
            volatility,
        }
    }

    /// Calculate d1 and d2 for the Black-Scholes formula.
    ///
    /// # Returns
    ///
    /// A tuple containing d1 and d2.
    fn calculate_d1_d2<T: Option>(&self, option: &T) -> (f64, f64) {
        let sqrt_t = self.time_to_maturity.sqrt();
        let n_dividends = option
            .instrument()
            .dividend_times
            .iter()
            .filter(|&&t| t <= self.time_to_maturity)
            .count() as f64;
        let adjusted_spot = option.instrument().spot
            * (1.0 - option.instrument().discrete_dividend_yield).powf(n_dividends);

        let d1 = ((adjusted_spot / option.strike()).ln()
            + (self.risk_free_rate - option.instrument().continuous_dividend_yield
                + 0.5 * self.volatility.powi(2))
                * self.time_to_maturity)
            / (self.volatility * sqrt_t);

        let d2 = d1 - self.volatility * sqrt_t;

        (d1, d2)
    }

    /// Calculate the price of an European call option using the Black-Scholes formula.
    ///
    /// # Returns
    ///
    /// The price of the call option.
    fn call_price<T: Option>(&self, option: T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(&option);

        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);
        let adjusted_spot = option.instrument().spot
            * (1.0 - option.instrument().discrete_dividend_yield).powf(
                option
                    .instrument()
                    .dividend_times
                    .iter()
                    .filter(|&&t| t <= self.time_to_maturity)
                    .count() as f64,
            );

        adjusted_spot
            * (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
            * nd1
            - option.strike() * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
    }

    /// Calculate the price of an European put option using the Black-Scholes formula.
    ///
    /// # Returns
    ///
    /// The price of the put option.
    fn put_price<T: Option>(&self, option: T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(&option);

        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(-d1);
        let nd2 = normal.cdf(-d2);
        let adjusted_spot = option.instrument().spot
            * (1.0 - option.instrument().discrete_dividend_yield).powf(
                option
                    .instrument()
                    .dividend_times
                    .iter()
                    .filter(|&&t| t <= self.time_to_maturity)
                    .count() as f64,
            );

        option.strike() * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
            - adjusted_spot
                * (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                * nd1
    }

    /// Calculate the price of a binary cash-or-nothing European option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option_type` - The type of option (Call or Put).
    ///
    /// # Returns
    ///
    /// The price of the binary option.
    fn binary_price<T: Option>(&self, option: T) -> f64 {
        let (_, d2) = self.calculate_d1_d2(&option);

        let normal = Normal::new(0.0, 1.0).unwrap();
        match option.option_type() {
            OptionType::Call => {
                (-self.risk_free_rate * self.time_to_maturity).exp() * normal.cdf(d2)
            }
            OptionType::Put => {
                -(-self.risk_free_rate * self.time_to_maturity).exp() * normal.cdf(d2)
            }
        }
    }

    /// Calculate the option price using the Black-Scholes formula with a given volatility.
    ///
    /// # Arguments
    ///
    /// * `option_type` - The type of option (Call or Put).
    /// * `volatility` - The volatility of the underlying asset.
    ///
    /// # Returns
    ///
    /// The price of the option.
    fn price_with_volatility<T: Option>(&self, option: T, volatility: f64) -> f64 {
        let sqrt_t = self.time_to_maturity.sqrt();
        let n_dividends = option
            .instrument()
            .dividend_times
            .iter()
            .filter(|&&t| t <= self.time_to_maturity)
            .count() as f64;
        let adjusted_spot = option.instrument().spot
            * (1.0 - option.instrument().discrete_dividend_yield).powf(n_dividends);

        let d1 = ((adjusted_spot / option.strike()).ln()
            + (self.risk_free_rate - option.instrument().continuous_dividend_yield
                + 0.5 * volatility.powi(2))
                * self.time_to_maturity)
            / (volatility * sqrt_t);

        let d2 = d1 - volatility * sqrt_t;

        let normal = Normal::new(0.0, 1.0).unwrap();
        match option.option_type() {
            OptionType::Call => {
                let nd1 = normal.cdf(d1);
                let nd2 = normal.cdf(d2);
                adjusted_spot
                    * (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                    * nd1
                    - option.strike() * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
            }
            OptionType::Put => {
                let nd1 = normal.cdf(-d1);
                let nd2 = normal.cdf(-d2);
                option.strike() * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
                    - adjusted_spot
                        * (-option.instrument().continuous_dividend_yield * self.time_to_maturity)
                            .exp()
                        * nd1
            }
        }
    }
}

impl OptionPricing for BlackScholesModel {
    fn price<T: Option>(&self, option: T) -> f64 {
        match (option.option_type(), option.style()) {
            (OptionType::Call, OptionStyle::European) => self.call_price(option),
            (OptionType::Put, OptionStyle::European) => self.put_price(option),
            (_, OptionStyle::Binary) => self.binary_price(option),
            _ => panic!("Unsupported option type or style"),
        }
    }

    fn implied_volatility<T: Option>(&self, option: T, market_price: f64) -> f64 {
        let mut sigma = 0.2; // Initial guess
        let tolerance = 1e-5;
        let max_iterations = 100;
        for _ in 0..max_iterations {
            let price = self.price_with_volatility(option.clone(), sigma);
            let vega = self.vega(option.clone());
            let diff = market_price - price;
            if diff.abs() < tolerance {
                return sigma;
            }
            sigma += diff / vega;
        }
        sigma
    }
}

impl Greeks for BlackScholesModel {
    fn delta<T: Option>(&self, option: T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(&option);

        let normal = Normal::new(0.0, 1.0).unwrap();
        match option.style() {
            OptionStyle::European => match option.option_type() {
                OptionType::Call => {
                    (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                        * normal.cdf(d1)
                }
                OptionType::Put => {
                    (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                        * (normal.cdf(d1) - 1.0)
                }
            },
            OptionStyle::Binary => {
                let delta = (-self.risk_free_rate * self.time_to_maturity).exp() * normal.pdf(d2)
                    / (self.volatility * option.instrument().spot * self.time_to_maturity.sqrt());

                match option.option_type() {
                    OptionType::Call => delta,
                    OptionType::Put => -delta,
                }
            }
            _ => panic!("Unsupported option style for delta calculation"),
        }
    }

    fn gamma<T: Option>(&self, option: T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(&option);

        let normal = Normal::new(0.0, 1.0).unwrap();
        let adjusted_spot = option.instrument().spot
            * (1.0 - option.instrument().discrete_dividend_yield).powf(
                option
                    .instrument()
                    .dividend_times
                    .iter()
                    .filter(|&&t| t <= self.time_to_maturity)
                    .count() as f64,
            );

        match option.style() {
            OptionStyle::European => {
                normal.pdf(d1) / (adjusted_spot * self.volatility * self.time_to_maturity.sqrt())
            }
            OptionStyle::Binary => {
                let gamma =
                    -(-self.risk_free_rate * self.time_to_maturity).exp() * normal.pdf(d2) * d1
                        / (self.volatility.powi(2)
                            * option.instrument().spot.powi(2)
                            * self.time_to_maturity);

                match option.option_type() {
                    OptionType::Call => gamma,
                    OptionType::Put => -gamma,
                }
            }
            _ => panic!("Unsupported option style for gamma calculation"),
        }
    }

    fn theta<T: Option>(&self, option: T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(&option);

        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);
        let pdf_d1 = normal.pdf(d1);
        let adjusted_spot = option.instrument().spot
            * (1.0 - option.instrument().discrete_dividend_yield).powf(
                option
                    .instrument()
                    .dividend_times
                    .iter()
                    .filter(|&&t| t <= self.time_to_maturity)
                    .count() as f64,
            );

        match option.style() {
            OptionStyle::European => match option.option_type() {
                OptionType::Call => {
                    adjusted_spot * pdf_d1 * self.volatility / (2.0 * self.time_to_maturity.sqrt())
                        + self.risk_free_rate
                            * option.strike()
                            * (-self.risk_free_rate * self.time_to_maturity).exp()
                            * nd2
                        - option.instrument().continuous_dividend_yield
                            * adjusted_spot
                            * (-option.instrument().continuous_dividend_yield
                                * self.time_to_maturity)
                                .exp()
                            * nd1
                }
                OptionType::Put => {
                    adjusted_spot * pdf_d1 * self.volatility / (2.0 * self.time_to_maturity.sqrt())
                        - self.risk_free_rate
                            * option.strike()
                            * (-self.risk_free_rate * self.time_to_maturity).exp()
                            * normal.cdf(-d2)
                        + option.instrument().continuous_dividend_yield
                            * adjusted_spot
                            * (-option.instrument().continuous_dividend_yield
                                * self.time_to_maturity)
                                .exp()
                            * normal.cdf(-d1)
                }
            },
            OptionStyle::Binary => match option.option_type() {
                OptionType::Call => {
                    (-self.risk_free_rate * self.time_to_maturity).exp()
                        * (normal.pdf(d2)
                            / (2.0
                                * self.time_to_maturity
                                * self.volatility
                                * self.time_to_maturity.sqrt())
                            * ((option.instrument().spot / option.strike()).ln()
                                - (self.risk_free_rate
                                    - option.instrument().continuous_dividend_yield
                                    - self.volatility.powi(2) * 0.5)
                                    * self.time_to_maturity)
                            + self.risk_free_rate * nd2)
                }
                OptionType::Put => {
                    -(-self.risk_free_rate * self.time_to_maturity).exp()
                        * (normal.pdf(d2)
                            / (2.0
                                * self.time_to_maturity
                                * self.volatility
                                * self.time_to_maturity.sqrt())
                            * ((option.instrument().spot / option.strike()).ln()
                                - (self.risk_free_rate
                                    - option.instrument().continuous_dividend_yield
                                    - self.volatility.powi(2) * 0.5)
                                    * self.time_to_maturity)
                            - self.risk_free_rate * normal.cdf(-d2))
                }
            },
            _ => panic!("Unsupported option style for theta calculation"),
        }
    }

    fn vega<T: Option>(&self, option: T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(&option);

        let normal = Normal::new(0.0, 1.0).unwrap();
        let adjusted_spot = option.instrument().spot
            * (1.0 - option.instrument().discrete_dividend_yield).powf(
                option
                    .instrument()
                    .dividend_times
                    .iter()
                    .filter(|&&t| t <= self.time_to_maturity)
                    .count() as f64,
            );

        match option.style() {
            OptionStyle::European => {
                adjusted_spot
                    * (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                    * normal.pdf(d1)
                    * self.time_to_maturity.sqrt()
            }
            OptionStyle::Binary => {
                let vega =
                    -(-self.risk_free_rate * self.time_to_maturity).exp() * d1 * normal.pdf(d2)
                        / self.volatility;

                match option.option_type() {
                    OptionType::Call => vega,
                    OptionType::Put => -vega,
                }
            }
            _ => panic!("Unsupported option style for vega calculation"),
        }
    }

    fn rho<T: Option>(&self, option: T) -> f64 {
        let (_, d2) = self.calculate_d1_d2(&option);

        let normal = Normal::new(0.0, 1.0).unwrap();
        match option.style() {
            OptionStyle::European => match option.option_type() {
                OptionType::Call => {
                    option.strike()
                        * self.time_to_maturity
                        * (-self.risk_free_rate * self.time_to_maturity).exp()
                        * normal.cdf(d2)
                }
                OptionType::Put => {
                    -option.strike()
                        * self.time_to_maturity
                        * (-self.risk_free_rate * self.time_to_maturity).exp()
                        * normal.cdf(-d2)
                }
            },
            OptionStyle::Binary => match option.option_type() {
                OptionType::Call => {
                    (-self.risk_free_rate * self.time_to_maturity).exp()
                        * (self.time_to_maturity.sqrt() * normal.pdf(d2) / self.volatility
                            - self.time_to_maturity * normal.cdf(d2))
                }
                OptionType::Put => {
                    -(-self.risk_free_rate * self.time_to_maturity).exp()
                        * (self.time_to_maturity.sqrt() * normal.pdf(d2) / self.volatility
                            + self.time_to_maturity * normal.cdf(-d2))
                }
            },
            _ => panic!("Unsupported option style for rho calculation"),
        }
    }
}
