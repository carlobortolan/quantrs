//! Module for Monte Carlo option pricing model.
//!
//! The Monte Carlo option pricing model is a numerical method used to calculate the theoretical price of options by simulating the random paths of the underlying asset's price.
//! This method is particularly useful for pricing complex derivatives and options with multiple sources of uncertainty or path-dependent features.
//!
//! The Monte Carlo model makes several assumptions, including:
//! - The underlying asset follows a stochastic process, typically modeled as a geometric Brownian motion.
//! - The risk-free interest rate is constant.
//! - The volatility of the underlying asset is constant.
//!
//! The Monte Carlo model is widely used by options traders and financial engineers to determine the fair price of an option based on various factors,
//! including the current price of the underlying asset, the strike price of the option, the time to expiration, the risk-free interest rate,
//! and the volatility of the underlying asset.
//!
//! ## Formula
//!
//! The price of an option using the Monte Carlo model is calculated by simulating the random paths of the underlying asset's price and averaging the discounted payoffs.
//!
//! The simulated price of the underlying asset at maturity is calculated as follows:
//!
//! ```text
//! ST = S * exp((r - 0.5 * σ^2) * T + σ * sqrt(T) * Z)
//! ```
//!
//! where:
//! - `ST` is the simulated price of the underlying asset at maturity.
//! - `S` is the current price of the underlying asset.
//! - `r` is the risk-free interest rate.
//! - `T` is the time to maturity.
//! - `σ` is the volatility of the underlying asset.
//! - `Z` is a random variable from the standard normal distribution.
//!
//! The payoff of the option is calculated as:
//!
//! ```text
//! payoff = max(ST - K, 0) for a call option
//! payoff = max(K - ST, 0) for a put option
//! ```
//!
//! where:
//! - `ST` is the simulated price of the underlying asset at maturity.
//! - `K` is the strike price of the option.
//! - `max` is the maximum function.
//!
//! The option price is then calculated as the discounted average of the simulated payoffs:
//!
//! ```text
//! price = e^(-rT) * (1 / N) * Σ payoff_i
//! ```
//!
//! where:
//! - `N` is the number of simulations.
//! - `payoff_i` is the payoff of the option in the i-th simulation.
//!
//! ## References
//!
//! - [Wikipedia - Monte Carlo methods in finance](https://en.wikipedia.org/wiki/Monte_Carlo_methods_in_finance)
//! - [Investopedia - Monte Carlo Simulation](https://www.investopedia.com/terms/m/montecarlosimulation.asp)
//! - [Options, Futures, and Other Derivatives (9th Edition)](https://www.pearson.com/store/p/options-futures-and-other-derivatives/P1000000000000013194)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{MonteCarloModel, OptionType, OptionPricing, Instrument, OptionStyle, EuropeanOption};
//!
//! let instrument = Instrument::new(100.0);
//! let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
//! let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);
//!
//! let price = model.price(option);
//! println!("Option price: {}", price);
//! ```

use crate::options::{Greeks, Option, OptionPricing, OptionType};
use rand_distr::{Distribution, Normal};

/// A struct representing a Monte Carlo option.
#[derive(Debug, Default, Clone)]
pub struct MonteCarloModel {
    /// Time horizon (in years).
    pub time_to_maturity: f64,
    /// Risk-free interest rate (e.g., 0.05 for 5%).
    pub risk_free_rate: f64,
    /// Volatility of the underlying asset (e.g., 0.2 for 20%).
    pub volatility: f64,
    /// Number of simulations to run.
    pub simulations: usize,
}

impl MonteCarloModel {
    /// Create a new `MonteCarloModel`.
    pub fn new(
        time_to_maturity: f64,
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
    ) -> Self {
        Self {
            time_to_maturity,
            risk_free_rate,
            volatility,
            simulations,
        }
    }
}

impl OptionPricing for MonteCarloModel {
    fn price<T: Option>(&self, option: T) -> f64 {
        let mut rng = rand::rng();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let dt_sqrt = self.time_to_maturity.sqrt();
        let drift = (self.risk_free_rate - 0.5 * self.volatility.powi(2)) * self.time_to_maturity;
        let discount_factor = (-self.risk_free_rate * self.time_to_maturity).exp();

        let sum_payoff: f64 = (0..self.simulations)
            .map(|_| {
                let z = normal.sample(&mut rng);
                let st = option.instrument().spot * (drift + self.volatility * dt_sqrt * z).exp();
                match option.option_type() {
                    OptionType::Call => (st - option.strike()).max(0.0),
                    OptionType::Put => (option.strike() - st).max(0.0),
                }
            })
            .sum();

        discount_factor * (sum_payoff / self.simulations as f64)
    }

    fn implied_volatility<T: Option>(&self, option: T, market_price: f64) -> f64 {
        // Return 0.0 for unrealistic market prices
        if market_price <= 0.0 || market_price > option.instrument().spot {
            return 0.0;
        }

        let mut sigma = 0.2; // Initial guess
        let tolerance = 1e-5;
        let max_iterations = 100;
        let mut prev_sigma = sigma;

        for _ in 0..max_iterations {
            let price = self.price(option.clone());
            let vega = self.vega(option.clone());
            let diff = market_price - price;

            if diff.abs() < tolerance {
                return sigma;
            }

            let update = diff / vega;
            sigma += update.clamp(-0.1, 0.1);

            if (sigma - prev_sigma).abs() < tolerance {
                return sigma;
            }

            prev_sigma = sigma;
        }

        sigma
    }
}

impl Greeks for MonteCarloModel {
    fn delta<T: Option + Clone>(&self, option: T) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut model = self.clone();
            model.time_to_maturity += epsilon;
            model.price(option.clone())
        };
        let price_down = {
            let mut model = self.clone();
            model.time_to_maturity -= epsilon;
            model.price(option.clone())
        };
        (price_up - price_down) / (2.0 * epsilon)
    }

    fn gamma<T: Option + Clone>(&self, option: T) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut model = self.clone();
            model.time_to_maturity += epsilon;
            model.price(option.clone())
        };
        let price_down = {
            let mut model = self.clone();
            model.time_to_maturity -= epsilon;
            model.price(option.clone())
        };
        let price = self.price(option.clone());
        (price_up - 2.0 * price + price_down) / (epsilon * epsilon)
    }

    fn theta<T: Option + Clone>(&self, option: T) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut model = self.clone();
            model.time_to_maturity += epsilon;
            model.price(option.clone())
        };
        let price_down = {
            let mut model = self.clone();
            model.time_to_maturity -= epsilon;
            model.price(option.clone())
        };
        (price_down - price_up) / (2.0 * epsilon)
    }

    fn vega<T: Option + Clone>(&self, option: T) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut opt = self.clone();
            opt.volatility += epsilon;
            self.price(option.clone())
        };
        let price_down = {
            let mut opt = self.clone();
            opt.volatility -= epsilon;
            self.price(option.clone())
        };
        (price_up - price_down) / (2.0 * epsilon)
    }

    fn rho<T: Option + Clone>(&self, option: T) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut opt = self.clone();
            opt.risk_free_rate += epsilon;
            self.price(option.clone())
        };
        let price_down = {
            let mut opt = self.clone();
            opt.risk_free_rate -= epsilon;
            self.price(option.clone())
        };
        (price_up - price_down) / (2.0 * epsilon)
    }
}
