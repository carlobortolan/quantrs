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
//! - [Cash or Nothing Options' Greeks](https://quantpie.co.uk/bsm_bin_c_formula/bs_bin_c_summary.php)
//! - [Asset or Nothing Options' Greeks](https://quantpie.co.uk/bsm_bin_a_formula/bs_bin_a_summary.php)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{BlackScholesModel, OptionType, OptionPricing, Instrument, EuropeanOption};
//!
//! let instrument = Instrument::new().with_spot(100.0);
//! let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
//! let model = BlackScholesModel::new(1.0, 0.05, 0.2);
//!
//! let price = model.price(&option);
//! println!("Option price: {}", price);
//! ```

use crate::options::{
    types::BinaryType::{AssetOrNothing, CashOrNothing},
    Option, OptionGreeks, OptionPricing, OptionStyle, OptionType,
};
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
    ///
    /// # Arguments
    ///
    /// * `time_to_maturity` - Time horizon (in years).
    /// * `risk_free_rate` - Risk-free interest rate (e.g., 0.05 for 5%).
    /// * `volatility` - Annualized standard deviation of an asset's continuous returns (e.g., 0.2 for 20%).
    ///     
    /// # Returns
    ///
    /// A new `BlackScholesModel`.
    pub fn new(time_to_maturity: f64, risk_free_rate: f64, volatility: f64) -> Self {
        Self {
            time_to_maturity,
            risk_free_rate,
            volatility,
        }
    }

    /// Calculate d1 and d2 for the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to calculate d1 and d2 for.
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

    /// Calculate the adjusted spot price.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to calculate the adjusted spot price for.
    ///
    /// # Returns
    ///
    /// The adjusted spot price.
    fn calculate_adjusted_spot<T: Option>(&self, option: &T) -> f64 {
        let n_dividends = option
            .instrument()
            .dividend_times
            .iter()
            .filter(|&&t| t <= self.time_to_maturity)
            .count() as f64;
        option.instrument().spot
            * (1.0 - option.instrument().discrete_dividend_yield).powf(n_dividends)
    }

    /// Calculate the price of an European call option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The call option to price.
    ///
    /// # Returns
    ///
    /// The price of the call option.
    fn price_euro_call<T: Option>(&self, option: &T, normal: &Normal) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(option);
        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);
        let adjusted_spot = self.calculate_adjusted_spot(option);

        adjusted_spot
            * (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
            * nd1
            - option.strike() * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
    }

    /// Calculate the price of an European put option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The binary option to price.
    ///
    /// # Returns
    ///
    /// The price of the put option.
    fn price_euro_put<T: Option>(&self, option: &T, normal: &Normal) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(option);
        let nd1 = normal.cdf(-d1);
        let nd2 = normal.cdf(-d2);
        let adjusted_spot = self.calculate_adjusted_spot(option);

        option.strike() * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
            - adjusted_spot
                * (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                * nd1
    }

    /// Calculate the price of a binary cash-or-nothing European option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The binary option to price.
    ///
    /// # Returns
    ///
    /// The price of the binary option.
    pub fn price_cash_or_nothing<T: Option>(&self, option: &T, normal: &Normal) -> f64 {
        let (_, d2) = self.calculate_d1_d2(option);

        match option.option_type() {
            OptionType::Call => {
                (-self.risk_free_rate * self.time_to_maturity).exp() * normal.cdf(d2)
            }
            OptionType::Put => {
                (-self.risk_free_rate * self.time_to_maturity).exp() * normal.cdf(-d2)
            }
        }
    }

    /// Calculate the price of a binary asset-or-nothing European option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The binary option to price.
    ///
    /// # Returns
    ///
    /// The price of the binary option.
    pub fn price_asset_or_nothing<T: Option>(&self, option: &T, normal: &Normal) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(option);

        match option.option_type() {
            OptionType::Call => {
                option.instrument().spot
                    * (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                    * normal.cdf(d1)
            }
            OptionType::Put => {
                option.instrument().spot
                    * (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                    * normal.cdf(-d1)
            }
        }
    }

    /// Calculate the option price using the Black-Scholes formula with a given volatility.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to price.
    /// * `volatility` - The volatility of the underlying asset.
    ///
    /// # Returns
    ///
    /// The price of the option.
    fn price_with_volatility<T: Option>(
        &self,
        option: &T,
        volatility: f64,
        normal: &Normal,
    ) -> f64 {
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
    fn price<T: Option>(&self, option: &T) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        match (option.option_type(), option.style()) {
            (OptionType::Call, OptionStyle::European) => self.price_euro_call(option, &normal),
            (OptionType::Put, OptionStyle::European) => self.price_euro_put(option, &normal),
            (_, OptionStyle::Binary(CashOrNothing)) => self.price_cash_or_nothing(option, &normal),
            (_, OptionStyle::Binary(AssetOrNothing)) => {
                self.price_asset_or_nothing(option, &normal)
            }
            //(OptionType::Call, OptionStyle::Rainbow(_)) => {
            //    option.instrument().assets.iter().map(|asset| {
            //        let option = option.clone().with_instrument(asset.clone());
            //        self.price(option)
            //    }).sum()
            //}
            _ => panic!("Unsupported option type or style"),
        }
    }

    /// Calculate the implied volatility of an option using the Newton-Raphson method.
    fn implied_volatility<T: Option>(&self, option: &T, market_price: f64) -> f64 {
        let mut sigma = 0.2; // Initial guess
        let tolerance = 1e-5;
        let max_iterations = 100;
        let normal = Normal::new(0.0, 1.0).unwrap();
        for _ in 0..max_iterations {
            let price = self.price_with_volatility(option, sigma, &normal);
            let vega = self.vega(option);
            let diff = market_price - price;
            if diff.abs() < tolerance {
                return sigma;
            }
            sigma += diff / vega;
        }
        sigma
    }

    // Calculate the implied volatility of an option using the Brent method.
    //fn implied_volatility<T: Option>(&self, option: &T, market_price: f64) -> f64 {
    //    let normal = Normal::new(0.0, 1.0).unwrap();
    //    let f = |sigma: f64| self.price_with_volatility(option, sigma, &normal) - market_price;
    //
    //    let tol = 1e-5;
    //    let lower_bound = 1e-5;
    //    let upper_bound = 5.0;
    //
    //    // Ensure that the function values at the bounds have different signs
    //    if f(lower_bound) * f(upper_bound) > 0.0 {
    //        panic!("f(min) and f(max) must have different signs");
    //    }
    //
    //    let problem = TestFunc::new(f);
    //    let solver = BrentRoot::new(lower_bound, upper_bound, tol);
    //
    //    let res = Executor::new(problem, solver)
    //        .configure(|state| state.max_iters(100))
    //        .run()
    //        .unwrap();
    //
    //    res.state().best_param.unwrap()
    //}
}

// struct TestFunc<F> {
//     f: F,
// }
//
// impl<F> TestFunc<F> {
//     fn new(f: F) -> Self {
//         TestFunc { f }
//     }
// }
//
// impl<F> CostFunction for TestFunc<F>
// where
//     F: Fn(f64) -> f64,
// {
//     // one dimensional problem, no vector needed
//     type Param = f64;
//     type Output = f64;
//
//     fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
//         Ok((self.f)(*p))
//     }
// }

impl OptionGreeks for BlackScholesModel {
    fn delta<T: Option>(&self, option: &T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(option);
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
            OptionStyle::Binary(CashOrNothing) => {
                let delta = (-self.risk_free_rate * self.time_to_maturity).exp() * normal.pdf(d2)
                    / (self.volatility * option.instrument().spot * self.time_to_maturity.sqrt());

                match option.option_type() {
                    OptionType::Call => delta,
                    OptionType::Put => -delta,
                }
            }
            OptionStyle::Binary(AssetOrNothing) => match option.option_type() {
                OptionType::Call => {
                    (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                        * normal.pdf(d1)
                        / (self.volatility * self.time_to_maturity.sqrt())
                        + (-option.instrument().continuous_dividend_yield * self.time_to_maturity)
                            .exp()
                            * normal.cdf(d1)
                }
                OptionType::Put => {
                    -(-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                        * normal.pdf(d1)
                        / (self.volatility * self.time_to_maturity.sqrt())
                        + (-option.instrument().continuous_dividend_yield * self.time_to_maturity)
                            .exp()
                            * normal.cdf(-d1)
                }
            },
            _ => panic!("Unsupported option style for delta calculation"),
        }
    }

    fn gamma<T: Option>(&self, option: &T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(option);
        let adjusted_spot = self.calculate_adjusted_spot(option);
        let normal = Normal::new(0.0, 1.0).unwrap();

        match option.style() {
            OptionStyle::European => {
                normal.pdf(d1) / (adjusted_spot * self.volatility * self.time_to_maturity.sqrt())
            }
            OptionStyle::Binary(CashOrNothing) => {
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
            OptionStyle::Binary(AssetOrNothing) => {
                let gamma = -(-option.instrument().continuous_dividend_yield
                    * self.time_to_maturity)
                    .exp()
                    * normal.pdf(d1)
                    * d2
                    / (option.instrument().spot * self.volatility.powi(2) * self.time_to_maturity);

                match option.option_type() {
                    OptionType::Call => gamma,
                    OptionType::Put => -gamma,
                }
            }
            _ => panic!("Unsupported option style for gamma calculation"),
        }
    }

    fn theta<T: Option>(&self, option: &T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(option);
        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);
        let pdf_d1 = normal.pdf(d1);
        let adjusted_spot = self.calculate_adjusted_spot(option);

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
            OptionStyle::Binary(CashOrNothing) => match option.option_type() {
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
            OptionStyle::Binary(AssetOrNothing) => match option.option_type() {
                OptionType::Call => {
                    option.instrument().spot
                        * (-option.instrument().continuous_dividend_yield * self.time_to_maturity)
                            .exp()
                        * (normal.pdf(d1) * 1.0
                            / (2.0
                                * self.time_to_maturity
                                * self.volatility
                                * self.time_to_maturity.sqrt())
                            * ((option.instrument().spot / option.strike()).ln()
                                - (self.risk_free_rate
                                    - option.instrument().continuous_dividend_yield
                                    + 0.5 * self.volatility.powi(2))
                                    * self.time_to_maturity)
                            + option.instrument().continuous_dividend_yield * nd1)
                }
                OptionType::Put => {
                    option.instrument().spot
                        * (-option.instrument().continuous_dividend_yield * self.time_to_maturity)
                            .exp()
                        * (-normal.pdf(d1) * 1.0
                            / (2.0
                                * self.time_to_maturity
                                * self.volatility
                                * self.time_to_maturity.sqrt())
                            * ((option.instrument().spot / option.strike()).ln()
                                - (self.risk_free_rate
                                    - option.instrument().continuous_dividend_yield
                                    + 0.5 * self.volatility.powi(2))
                                    * self.time_to_maturity)
                            + option.instrument().continuous_dividend_yield * -nd1)
                }
            },
            _ => panic!("Unsupported option style for theta calculation"),
        }
    }

    fn vega<T: Option>(&self, option: &T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(option);
        let normal = Normal::new(0.0, 1.0).unwrap();
        let adjusted_spot = self.calculate_adjusted_spot(option);

        match option.style() {
            OptionStyle::European => {
                adjusted_spot
                    * (-option.instrument().continuous_dividend_yield * self.time_to_maturity).exp()
                    * normal.pdf(d1)
                    * self.time_to_maturity.sqrt()
            }
            OptionStyle::Binary(CashOrNothing) => {
                let vega =
                    -(-self.risk_free_rate * self.time_to_maturity).exp() * d1 * normal.pdf(d2)
                        / self.volatility;

                match option.option_type() {
                    OptionType::Call => vega,
                    OptionType::Put => -vega,
                }
            }
            OptionStyle::Binary(AssetOrNothing) => {
                let vega = -option.instrument().spot
                    * (-option.instrument().continuous_dividend_yield * self.time_to_maturity)
                        .exp()
                    * d2
                    * normal.pdf(d1)
                    / (self.volatility);

                match option.option_type() {
                    OptionType::Call => vega,
                    OptionType::Put => -vega,
                }
            }
            _ => panic!("Unsupported option style for vega calculation"),
        }
    }

    fn rho<T: Option>(&self, option: &T) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(option);
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
            OptionStyle::Binary(CashOrNothing) => match option.option_type() {
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
            OptionStyle::Binary(AssetOrNothing) => {
                let rho = option.instrument().spot
                    * (-option.instrument().continuous_dividend_yield * self.time_to_maturity)
                        .exp()
                    * self.time_to_maturity.sqrt()
                    * normal.pdf(d1)
                    / (self.volatility);

                match option.option_type() {
                    OptionType::Call => rho,
                    OptionType::Put => -rho,
                }
            }
            _ => panic!("Unsupported option style for rho calculation"),
        }
    }
}
