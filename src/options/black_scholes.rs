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
//! - [Investopedia - Black-Scholes Model](https://www.investopedia.com/terms/b/blackscholes.asp)
//! - [Options, Futures, and Other Derivatives (9th Edition)](https://www.pearson.com/store/p/options-futures-and-other-derivatives/P1000000000000013194)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{BlackScholesOption, OptionType, OptionPricing};
//!
//! let bs_option = BlackScholesOption {
//!    spot: 100.0,
//!    strike: 100.0,
//!    time_to_maturity: 1.0,
//!    risk_free_rate: 0.05,
//!    volatility: 0.2,
//!    dividend_yield: 0.02,
//!    ..Default::default()
//! };
//!
//! let price = bs_option.price(OptionType::Call);
//! println!("Option price: {}", price);
//! ```
use super::{Greeks, Option, OptionPricing, OptionStyle, OptionType};
use statrs::distribution::{Continuous, ContinuousCDF, Normal};

/// A struct representing a Black-Scholes option.
#[derive(Debug, Default)]
pub struct BlackScholesOption {
    /// Style of the option (American, European, etc.).
    pub style: OptionStyle,
    /// Current price of the underlying asset.
    pub spot: f64,
    /// Strike price of the option.
    pub strike: f64,
    /// Time horizon (in years).
    pub time_to_maturity: f64,
    /// Risk-free interest rate (e.g., 0.05 for 5%).
    pub risk_free_rate: f64,
    /// Annualized standard deviation of an asset's continuous returns (e.g., 0.2 for 20%).
    pub volatility: f64,
    /// Continuous dividend yield where the dividend amount is proportional to the level of the underlying asset (e.g., 0.02 for 2%).
    pub dividend_yield: f64,
}

impl BlackScholesOption {
    /// Calculate the price of an European call option using the Black-Scholes formula.
    ///
    /// # Returns
    ///
    /// The price of the call option.
    fn call_price(&self) -> f64 {
        let sqrt_t = self.time_to_maturity.sqrt();

        let d1: f64 = ((self.spot / self.strike).ln()
            + (self.risk_free_rate - self.dividend_yield + 0.5 * self.volatility.powi(2))
                * self.time_to_maturity)
            / (self.volatility * sqrt_t);

        let d2 = d1 - self.volatility * sqrt_t;

        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(d1);
        let nd2 = normal.cdf(d2);
        self.spot * (-self.dividend_yield * self.time_to_maturity).exp() * nd1
            - self.strike * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
    }

    /// Calculate the price of an European put option using the Black-Scholes formula.
    ///
    /// # Returns
    ///
    /// The price of the put option.
    fn put_price(&self) -> f64 {
        let sqrt_t = self.time_to_maturity.sqrt();

        let d1: f64 = ((self.spot / self.strike).ln()
            + (self.risk_free_rate - self.dividend_yield + 0.5 * self.volatility.powi(2))
                * self.time_to_maturity)
            / (self.volatility * sqrt_t);

        let d2 = d1 - self.volatility * sqrt_t;

        let normal = Normal::new(0.0, 1.0).unwrap();
        let nd1 = normal.cdf(-d1);
        let nd2 = normal.cdf(-d2);
        self.strike * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
            - self.spot * (-self.dividend_yield * self.time_to_maturity).exp() * nd1
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
    fn binary_price(&self, option_type: OptionType) -> f64 {
        let sqrt_t = self.time_to_maturity.sqrt();

        let d2: f64 = ((self.spot / self.strike).ln()
            + (self.risk_free_rate - self.dividend_yield - 0.5 * self.volatility.powi(2))
                * self.time_to_maturity)
            / (self.volatility * sqrt_t);

        let normal = Normal::new(0.0, 1.0).unwrap();
        match option_type {
            OptionType::Call => {
                (-self.risk_free_rate * self.time_to_maturity).exp() * normal.cdf(d2)
            }
            OptionType::Put => {
                (-self.risk_free_rate * self.time_to_maturity).exp() * normal.cdf(-d2)
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
    pub fn price_with_volatility(&self, option_type: OptionType, volatility: f64) -> f64 {
        let sqrt_t = self.time_to_maturity.sqrt();

        let d1: f64 = ((self.spot / self.strike).ln()
            + (self.risk_free_rate - self.dividend_yield + 0.5 * volatility.powi(2))
                * self.time_to_maturity)
            / (volatility * sqrt_t);

        let d2 = d1 - volatility * sqrt_t;

        let normal = Normal::new(0.0, 1.0).unwrap();
        match option_type {
            OptionType::Call => {
                let nd1 = normal.cdf(d1);
                let nd2 = normal.cdf(d2);
                self.spot * (-self.dividend_yield * self.time_to_maturity).exp() * nd1
                    - self.strike * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
            }
            OptionType::Put => {
                let nd1 = normal.cdf(-d1);
                let nd2 = normal.cdf(-d2);
                self.strike * (-self.risk_free_rate * self.time_to_maturity).exp() * nd2
                    - self.spot * (-self.dividend_yield * self.time_to_maturity).exp() * nd1
            }
        }
    }
}

impl OptionPricing for BlackScholesOption {
    fn price(&self, option_type: OptionType) -> f64 {
        match (option_type, self.style) {
            (OptionType::Call, OptionStyle::European) => self.call_price(),
            (OptionType::Put, OptionStyle::European) => self.put_price(),
            (_, OptionStyle::Binary) => self.binary_price(option_type),
            _ => panic!("Unsupported option type or style"),
        }
    }

    fn implied_volatility(&self, market_price: f64, option_type: OptionType) -> f64 {
        let mut sigma = 0.2; // Initial guess
        let tolerance = 1e-5;
        let max_iterations = 100;
        for _ in 0..max_iterations {
            let price = self.price_with_volatility(option_type, sigma);
            let vega = self.vega(option_type);
            let diff = market_price - price;
            if diff.abs() < tolerance {
                return sigma;
            }
            sigma += diff / vega;
        }
        sigma
    }

    fn strike(&self) -> f64 {
        self.strike
    }
}

impl Greeks for BlackScholesOption {
    fn delta(&self, option_type: OptionType) -> f64 {
        let sqrt_t = self.time_to_maturity.sqrt();

        let d1: f64 = ((self.spot / self.strike).ln()
            + (self.risk_free_rate - self.dividend_yield + 0.5 * self.volatility.powi(2))
                * self.time_to_maturity)
            / (self.volatility * sqrt_t);

        let normal = Normal::new(0.0, 1.0).unwrap();
        match option_type {
            OptionType::Call => {
                (-self.dividend_yield * self.time_to_maturity).exp() * normal.cdf(d1)
            }
            OptionType::Put => {
                (-self.dividend_yield * self.time_to_maturity).exp() * (normal.cdf(d1) - 1.0)
            }
        }
    }

    fn gamma(&self, option_type: OptionType) -> f64 {
        let sqrt_t = self.time_to_maturity.sqrt();

        let d1: f64 = ((self.spot / self.strike).ln()
            + (self.risk_free_rate - self.dividend_yield + 0.5 * self.volatility.powi(2))
                * self.time_to_maturity)
            / (self.volatility * sqrt_t);

        let normal = Normal::new(0.0, 1.0).unwrap();
        normal.pdf(d1) / (self.spot * self.volatility * sqrt_t)
    }

    fn theta(&self, option_type: OptionType) -> f64 {
        -0.01 // TODO: Placeholder value
    }

    fn vega(&self, option_type: OptionType) -> f64 {
        let sqrt_t = self.time_to_maturity.sqrt();

        let d1: f64 = ((self.spot / self.strike).ln()
            + (self.risk_free_rate - self.dividend_yield + 0.5 * self.volatility.powi(2))
                * self.time_to_maturity)
            / (self.volatility * sqrt_t);

        let normal = Normal::new(0.0, 1.0).unwrap();
        self.spot * (-self.dividend_yield * self.time_to_maturity).exp() * normal.pdf(d1) * sqrt_t
    }

    fn rho(&self, option_type: OptionType) -> f64 {
        let sqrt_t = self.time_to_maturity.sqrt();

        let d2: f64 = ((self.spot / self.strike).ln()
            + (self.risk_free_rate - self.dividend_yield - 0.5 * self.volatility.powi(2))
                * self.time_to_maturity)
            / (self.volatility * sqrt_t);

        let normal = Normal::new(0.0, 1.0).unwrap();
        match option_type {
            OptionType::Call => {
                self.strike
                    * self.time_to_maturity
                    * (-self.risk_free_rate * self.time_to_maturity).exp()
                    * normal.cdf(d2)
            }
            OptionType::Put => {
                -self.strike
                    * self.time_to_maturity
                    * (-self.risk_free_rate * self.time_to_maturity).exp()
                    * normal.cdf(-d2)
            }
        }
    }
}

impl Option for BlackScholesOption {
    fn style(&self) -> &OptionStyle {
        &self.style
    }
}
