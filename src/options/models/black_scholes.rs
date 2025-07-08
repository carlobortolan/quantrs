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
//! ## References
//!
//! - [Wikipedia - Black-Scholes model](https://en.wikipedia.org/wiki/Black%E2%80%93Scholes_model)
//! - [Black-Scholes Calculator](https://www.math.drexel.edu/~pg/fin/VanillaCalculator.html)
//! - [Cash or Nothing Options' Greeks](https://quantpie.co.uk/bsm_bin_c_formula/bs_bin_c_summary.php)
//! - [Asset or Nothing Options' Greeks](https://quantpie.co.uk/bsm_bin_a_formula/bs_bin_a_summary.php)
//! - Musiela, M., Rutkowski, M. Martingale Methods in Financial Modelling, 2nd Ed Springer, 2007
//! - Joshi, M. The Concepts and Practice of Mathematical Finance, 2nd Ed Cambridge University Press, 2008
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{BlackScholesModel, OptionType, OptionPricing, Instrument, EuropeanOption};
//!
//! let instrument = Instrument::new().with_spot(100.0);
//! let option = EuropeanOption::new(instrument, 100.0, 1.0, OptionType::Call);
//! let model = BlackScholesModel::new(0.05, 0.2);
//!
//! let price = model.price(&option);
//! println!("Option price: {price}");
//! ```

use crate::options::{
    types::BinaryType::{AssetOrNothing, CashOrNothing},
    Instrument, Option, OptionGreeks, OptionPricing, OptionStrategy, OptionStyle, OptionType,
    Permutation, RainbowType,
};
use rand_distr::num_traits::Pow;
use statrs::distribution::{Continuous, ContinuousCDF, Normal};

/// A struct representing a Black-Scholes model.
#[derive(Debug, Default)]
pub struct BlackScholesModel {
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
    /// * `risk_free_rate` - Risk-free interest rate (e.g., 0.05 for 5%).
    /// * `volatility` - Annualized standard deviation of an asset's continuous returns (e.g., 0.2 for 20%).
    ///     
    /// # Returns
    ///
    /// A new `BlackScholesModel`.
    pub fn new(risk_free_rate: f64, volatility: f64) -> Self {
        Self {
            risk_free_rate,
            volatility,
        }
    }

    /// Calculate d1 and d2 for the Black-Scholes formula.
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
            + (self.risk_free_rate - instrument.continuous_dividend_yield
                + 0.5 * self.volatility.powi(2))
                * ttm)
            / (self.volatility * sqrt_t);

        let d2 = d1 - self.volatility * sqrt_t;

        (d1, d2)
    }

    /// Calculate the price of an European call option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `instrument` - The instrument to price the call option for.
    /// * `strike` - The strike price of the option.
    /// * `ttm` - Time to maturity of the option.
    /// * `normal` - The standard normal distribution.
    ///
    /// # Returns
    ///
    /// The price of the call option.
    pub fn price_euro_call(
        &self,
        instrument: &Instrument,
        strike: f64,
        ttm: f64,
        normal: &Normal,
    ) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(instrument, strike, ttm);
        instrument.calculate_adjusted_spot(ttm)
            * (-instrument.continuous_dividend_yield * ttm).exp()
            * normal.cdf(d1)
            - strike * (-self.risk_free_rate * ttm).exp() * normal.cdf(d2)
    }

    /// Calculate the price of an European put option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `instrument` - The instrument to price the put option for.
    /// * `strike` - The strike price of the option.
    /// * `ttm` - Time to maturity of the option.
    /// * `normal` - The standard normal distribution.
    ///
    /// # Returns
    ///
    /// The price of the put option.
    pub fn price_euro_put(
        &self,
        instrument: &Instrument,
        strike: f64,
        ttm: f64,
        normal: &Normal,
    ) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(instrument, strike, ttm);
        strike * (-self.risk_free_rate * ttm).exp() * normal.cdf(-d2)
            - instrument.calculate_adjusted_spot(ttm)
                * (-instrument.continuous_dividend_yield * ttm).exp()
                * normal.cdf(-d1)
    }

    /// Calculate the price of a binary cash-or-nothing European option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The binary option to price.
    /// * `normal` - The standard normal distribution.
    ///
    /// # Returns
    ///
    /// The price of the binary option.
    pub fn price_cash_or_nothing<T: Option>(&self, option: &T, normal: &Normal) -> f64 {
        let (_, d2) = self.calculate_d1_d2(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
        );

        match option.option_type() {
            OptionType::Call => {
                (-self.risk_free_rate * option.time_to_maturity()).exp() * normal.cdf(d2)
            }
            OptionType::Put => {
                (-self.risk_free_rate * option.time_to_maturity()).exp() * normal.cdf(-d2)
            }
        }
    }

    /// Calculate the price of a binary asset-or-nothing European option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The binary option to price.
    /// * `normal` - The standard normal distribution.
    ///
    /// # Returns
    ///
    /// The price of the binary option.
    pub fn price_asset_or_nothing<T: Option>(&self, option: &T, normal: &Normal) -> f64 {
        let (d1, d2) = self.calculate_d1_d2(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
        );

        match option.option_type() {
            OptionType::Call => {
                option.instrument().spot()
                    * (-option.instrument().continuous_dividend_yield * option.time_to_maturity())
                        .exp()
                    * normal.cdf(d1)
            }
            OptionType::Put => {
                option.instrument().spot()
                    * (-option.instrument().continuous_dividend_yield * option.time_to_maturity())
                        .exp()
                    * normal.cdf(-d1)
            }
        }
    }

    /// Calculate the price of a rainbow call using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to price.
    /// * `normal` - The standard normal distribution.
    ///
    /// # Returns
    ///
    /// The price of the option.
    pub fn price_rainbow_call<T: Option>(&self, option: &T, normal: &Normal) -> f64 {
        if matches!(option.style(), OptionStyle::Rainbow(RainbowType::AllITM))
            && option.payoff(None) <= 0.0
        {
            return 0.0;
        }
        let price = self.price_euro_call(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
            normal,
        );

        if matches!(option.style(), OptionStyle::Rainbow(RainbowType::BestOf))
            || matches!(option.style(), OptionStyle::Rainbow(RainbowType::WorstOf))
        {
            panic!("BestOf/WorstOf options not supported by Black-Scholes model");
        }
        price
    }

    /// Calculate the price of a rainbow put using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to price.
    /// * `normal` - The standard normal distribution.
    ///
    /// # Returns
    ///
    /// The price of the option.
    pub fn price_rainbow_put<T: Option>(&self, option: &T, normal: &Normal) -> f64 {
        if matches!(option.style(), OptionStyle::Rainbow(RainbowType::AllOTM))
            && option.payoff(None) <= 0.0
        {
            return 0.0;
        }
        self.price_euro_put(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
            normal,
        )
    }

    /// Calculate the price of a lookback option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to price.
    /// * `normal` - The standard normal distribution.
    ///
    /// # Returns
    ///
    /// The price of the option.
    pub fn price_lookback<T: Option>(&self, option: &T, normal: &Normal) -> f64 {
        let max = option.instrument().max_spot();
        let min = option.instrument().min_spot();
        let sqrt_t = option.time_to_maturity().sqrt();
        let s = option.instrument().spot();
        let r = self.risk_free_rate;
        let t = option.time_to_maturity();
        let vola = self.volatility;

        assert!(s > 0.0 && max > 0.0 && min > 0.0, "Spot prices must be > 0");

        println!("max: {max}, min: {min}");

        let a1 = |s: f64, h: f64| ((s / h).ln() + (r + 0.5 * vola.powi(2)) * t) / (vola * sqrt_t);
        let a2 = |s: f64, h: f64| a1(s, h) - vola * sqrt_t;
        let a3 = |s: f64, h: f64| a1(s, h) - 2.0 * r * sqrt_t / vola;

        let phi = |x: f64| normal.cdf(x);

        match option.option_type() {
            OptionType::Call => {
                s * phi(a1(s, min))
                    - min * (-r * t).exp() * phi(a2(s, min))
                    - (0.5 * s * vola.powi(2)) / (r)
                        * (phi(-a1(s, min))
                            - (-r * t).exp()
                                * (min / s).pow((2f64 * r) / (vola.powi(2)))
                                * phi(-a3(s, min)))
            }
            OptionType::Put => {
                -s * phi(-a1(s, max))
                    + max * (-r * t).exp() * phi(-a2(s, max))
                    + (0.5 * s * vola.powi(2)) / (r)
                        * (phi(a1(s, max))
                            - (-r * t).exp()
                                * (max / s).pow((2f64 * r) / (vola.powi(2)))
                                * phi(a3(s, max)))
            }
        }
    }

    /// Calculate the option price using the Black-Scholes formula with a given volatility.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to price.
    /// * `volatility` - The volatility of the underlying asset.
    /// * `normal` - The standard normal distribution.
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
        let sqrt_t = option.time_to_maturity().sqrt();
        let n_dividends = option
            .instrument()
            .dividend_times
            .iter()
            .filter(|&&t| t <= option.time_to_maturity())
            .count() as f64;
        let adjusted_spot = option.instrument().spot()
            * (1.0 - option.instrument().discrete_dividend_yield).powf(n_dividends);

        let d1 = ((adjusted_spot / option.strike()).ln()
            + (self.risk_free_rate - option.instrument().continuous_dividend_yield
                + 0.5 * volatility.powi(2))
                * option.time_to_maturity())
            / (volatility * sqrt_t);

        let d2 = d1 - volatility * sqrt_t;

        match option.option_type() {
            OptionType::Call => {
                let nd1 = normal.cdf(d1);
                let nd2 = normal.cdf(d2);
                adjusted_spot
                    * (-option.instrument().continuous_dividend_yield * option.time_to_maturity())
                        .exp()
                    * nd1
                    - option.strike()
                        * (-self.risk_free_rate * option.time_to_maturity()).exp()
                        * nd2
            }
            OptionType::Put => {
                let nd1 = normal.cdf(-d1);
                let nd2 = normal.cdf(-d2);
                option.strike() * (-self.risk_free_rate * option.time_to_maturity()).exp() * nd2
                    - adjusted_spot
                        * (-option.instrument().continuous_dividend_yield
                            * option.time_to_maturity())
                        .exp()
                        * nd1
            }
        }
    }
}

impl OptionPricing for BlackScholesModel {
    #[rustfmt::skip]
    fn price<T: Option>(&self, option: &T) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        match (option.option_type(), option.style()) {
            (OptionType::Call, OptionStyle::European) => self.price_euro_call(option.instrument(), option.strike(),option.time_to_maturity(), &normal),
            (OptionType::Put, OptionStyle::European) => self.price_euro_put(option.instrument(), option.strike(), option.time_to_maturity(),&normal),
            (_, OptionStyle::Binary(CashOrNothing)) => self.price_cash_or_nothing(option, &normal),
            (_, OptionStyle::Binary(AssetOrNothing)) => self.price_asset_or_nothing(option, &normal),
            (OptionType::Call, OptionStyle::Rainbow(_)) => self.price_rainbow_call(option, &normal),
            (OptionType::Put, OptionStyle::Rainbow(_)) => self.price_rainbow_put(option, &normal),
            (_, OptionStyle::Lookback(Permutation::Floating)) => self.price_lookback(option, &normal),
            _ => panic!("BlackScholesModel does not support this option type or style"),
        }
    }

    /// Calculate the implied volatility of an option using the Newton-Raphson method.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to calculate the implied volatility for.
    /// * `market_price` - The market price of the option.
    ///
    /// # Returns
    ///
    /// The implied volatility of the option.
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

impl OptionGreeks for BlackScholesModel {
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
                    (-option.instrument().continuous_dividend_yield * option.time_to_maturity())
                        .exp()
                        * normal.cdf(d1)
                }
                OptionType::Put => {
                    (-option.instrument().continuous_dividend_yield * option.time_to_maturity())
                        .exp()
                        * (normal.cdf(d1) - 1.0)
                }
            },
            OptionStyle::Binary(CashOrNothing) => {
                let delta = (-self.risk_free_rate * option.time_to_maturity()).exp()
                    * normal.pdf(d2)
                    / (self.volatility
                        * option.instrument().spot()
                        * option.time_to_maturity().sqrt());

                match option.option_type() {
                    OptionType::Call => delta,
                    OptionType::Put => -delta,
                }
            }
            OptionStyle::Binary(AssetOrNothing) => match option.option_type() {
                OptionType::Call => {
                    (-option.instrument().continuous_dividend_yield * option.time_to_maturity())
                        .exp()
                        * normal.pdf(d1)
                        / (self.volatility * option.time_to_maturity().sqrt())
                        + (-option.instrument().continuous_dividend_yield
                            * option.time_to_maturity())
                        .exp()
                            * normal.cdf(d1)
                }
                OptionType::Put => {
                    -(-option.instrument().continuous_dividend_yield * option.time_to_maturity())
                        .exp()
                        * normal.pdf(d1)
                        / (self.volatility * option.time_to_maturity().sqrt())
                        + (-option.instrument().continuous_dividend_yield
                            * option.time_to_maturity())
                        .exp()
                            * normal.cdf(-d1)
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
                normal.pdf(d1)
                    / (adjusted_spot * self.volatility * option.time_to_maturity().sqrt())
            }
            OptionStyle::Binary(CashOrNothing) => {
                let gamma =
                    -(-self.risk_free_rate * option.time_to_maturity()).exp() * normal.pdf(d2) * d1
                        / (self.volatility.powi(2)
                            * option.instrument().spot().powi(2)
                            * option.time_to_maturity());

                match option.option_type() {
                    OptionType::Call => gamma,
                    OptionType::Put => -gamma,
                }
            }
            OptionStyle::Binary(AssetOrNothing) => {
                let gamma = -(-option.instrument().continuous_dividend_yield
                    * option.time_to_maturity())
                .exp()
                    * normal.pdf(d1)
                    * d2
                    / (option.instrument().spot()
                        * self.volatility.powi(2)
                        * option.time_to_maturity());

                match option.option_type() {
                    OptionType::Call => gamma,
                    OptionType::Put => -gamma,
                }
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
                    adjusted_spot * normal.pdf(d1) * self.volatility
                        / (2.0 * option.time_to_maturity().sqrt())
                        + self.risk_free_rate
                            * option.strike()
                            * (-self.risk_free_rate * option.time_to_maturity()).exp()
                            * normal.cdf(d2)
                        - option.instrument().continuous_dividend_yield
                            * adjusted_spot
                            * (-option.instrument().continuous_dividend_yield
                                * option.time_to_maturity())
                            .exp()
                            * normal.cdf(d1)
                }
                OptionType::Put => {
                    adjusted_spot * normal.pdf(d1) * self.volatility
                        / (2.0 * option.time_to_maturity().sqrt())
                        - self.risk_free_rate
                            * option.strike()
                            * (-self.risk_free_rate * option.time_to_maturity()).exp()
                            * normal.cdf(-d2)
                        + option.instrument().continuous_dividend_yield
                            * adjusted_spot
                            * (-option.instrument().continuous_dividend_yield
                                * option.time_to_maturity())
                            .exp()
                            * normal.cdf(-d1)
                }
            },
            OptionStyle::Binary(CashOrNothing) => match option.option_type() {
                OptionType::Call => {
                    (-self.risk_free_rate * option.time_to_maturity()).exp()
                        * (normal.pdf(d2)
                            / (2.0
                                * option.time_to_maturity()
                                * self.volatility
                                * option.time_to_maturity().sqrt())
                            * ((option.instrument().spot() / option.strike()).ln()
                                - (self.risk_free_rate
                                    - option.instrument().continuous_dividend_yield
                                    - self.volatility.powi(2) * 0.5)
                                    * option.time_to_maturity())
                            + self.risk_free_rate * normal.cdf(d2))
                }
                OptionType::Put => {
                    -(-self.risk_free_rate * option.time_to_maturity()).exp()
                        * (normal.pdf(d2)
                            / (2.0
                                * option.time_to_maturity()
                                * self.volatility
                                * option.time_to_maturity().sqrt())
                            * ((option.instrument().spot() / option.strike()).ln()
                                - (self.risk_free_rate
                                    - option.instrument().continuous_dividend_yield
                                    - self.volatility.powi(2) * 0.5)
                                    * option.time_to_maturity())
                            - self.risk_free_rate * normal.cdf(-d2))
                }
            },
            OptionStyle::Binary(AssetOrNothing) => match option.option_type() {
                OptionType::Call => {
                    option.instrument().spot()
                        * (-option.instrument().continuous_dividend_yield
                            * option.time_to_maturity())
                        .exp()
                        * (normal.pdf(d1) * 1.0
                            / (2.0
                                * option.time_to_maturity()
                                * self.volatility
                                * option.time_to_maturity().sqrt())
                            * ((option.instrument().spot() / option.strike()).ln()
                                - (self.risk_free_rate
                                    - option.instrument().continuous_dividend_yield
                                    + 0.5 * self.volatility.powi(2))
                                    * option.time_to_maturity())
                            + option.instrument().continuous_dividend_yield * normal.cdf(d1))
                }
                OptionType::Put => {
                    option.instrument().spot()
                        * (-option.instrument().continuous_dividend_yield
                            * option.time_to_maturity())
                        .exp()
                        * (-normal.pdf(d1) * 1.0
                            / (2.0
                                * option.time_to_maturity()
                                * self.volatility
                                * option.time_to_maturity().sqrt())
                            * ((option.instrument().spot() / option.strike()).ln()
                                - (self.risk_free_rate
                                    - option.instrument().continuous_dividend_yield
                                    + 0.5 * self.volatility.powi(2))
                                    * option.time_to_maturity())
                            + option.instrument().continuous_dividend_yield * -normal.cdf(d1))
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
                    * (-option.instrument().continuous_dividend_yield * option.time_to_maturity())
                        .exp()
                    * normal.pdf(d1)
                    * option.time_to_maturity().sqrt()
            }
            OptionStyle::Binary(CashOrNothing) => {
                let vega =
                    -(-self.risk_free_rate * option.time_to_maturity()).exp() * d1 * normal.pdf(d2)
                        / self.volatility;

                match option.option_type() {
                    OptionType::Call => vega,
                    OptionType::Put => -vega,
                }
            }
            OptionStyle::Binary(AssetOrNothing) => {
                let vega = -option.instrument().spot()
                    * (-option.instrument().continuous_dividend_yield * option.time_to_maturity())
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
        let (d1, d2) = self.calculate_d1_d2(
            option.instrument(),
            option.strike(),
            option.time_to_maturity(),
        );
        let normal = Normal::new(0.0, 1.0).unwrap();

        match option.style() {
            OptionStyle::European => match option.option_type() {
                OptionType::Call => {
                    option.strike()
                        * option.time_to_maturity()
                        * (-self.risk_free_rate * option.time_to_maturity()).exp()
                        * normal.cdf(d2)
                }
                OptionType::Put => {
                    -option.strike()
                        * option.time_to_maturity()
                        * (-self.risk_free_rate * option.time_to_maturity()).exp()
                        * normal.cdf(-d2)
                }
            },
            OptionStyle::Binary(CashOrNothing) => match option.option_type() {
                OptionType::Call => {
                    (-self.risk_free_rate * option.time_to_maturity()).exp()
                        * (option.time_to_maturity().sqrt() * normal.pdf(d2) / self.volatility
                            - option.time_to_maturity() * normal.cdf(d2))
                }
                OptionType::Put => {
                    -(-self.risk_free_rate * option.time_to_maturity()).exp()
                        * (option.time_to_maturity().sqrt() * normal.pdf(d2) / self.volatility
                            + option.time_to_maturity() * normal.cdf(-d2))
                }
            },
            OptionStyle::Binary(AssetOrNothing) => {
                let rho = option.instrument().spot()
                    * (-option.instrument().continuous_dividend_yield * option.time_to_maturity())
                        .exp()
                    * option.time_to_maturity().sqrt()
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

impl OptionStrategy for BlackScholesModel {}
