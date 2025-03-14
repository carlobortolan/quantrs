//! Module for Monte Carlo option pricing model.
//!
//! The Monte Carlo option pricing model is a mathematical model used to calculate the theoretical price of options using simulation.
//! The model is based on the assumption that the price of the underlying asset follows a log-normal distribution.
//!
//! The Monte Carlo model is widely used by options traders to determine the fair price of an option based on various factors,
//! including the current price of the underlying asset, the strike price of the option, the time to expiration, the risk-free interest rate,
//! and the volatility of the underlying asset.
//!
//! The Monte Carlo model is particularly useful for pricing options with complex payoff structures or when the underlying asset does not follow a simple log-normal distribution.
//!
//! ## References
//!
//! - [Wikipedia - Monte Carlo methods in finance](https://en.wikipedia.org/wiki/Monte_Carlo_methods_in_finance)
//! - [Investopedia - Monte Carlo Simulation](https://www.investopedia.com/terms/m/montecarlosimulation.asp)
//! - [Options, Futures, and Other Derivatives (9th Edition)](https://www.pearson.com/store/p/options-futures-and-other-derivatives/P1000000000000013194)
//!
//! ## Formula
//!
//! The price of an option using the Monte Carlo model is calculated as follows:
//!
//! ```text
//! price = e^(-rT) * (Σ(payoff) / N)
//! ```
//!
//! where:
//! - `price` is the price of the option.
//! - `r` is the risk-free interest rate.
//! - `T` is the time to maturity.
//! - `Σ(payoff)` is the sum of the payoffs from each simulation.
//! - `N` is the number of simulations.
//!
//! The payoff is calculated as:
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
//! ## Example
//!
//! ```
//! use quantrs::options::{MonteCarloOption, OptionType, OptionPricing};
//!
//! let mc_option = MonteCarloOption {
//!    spot: 100.0,
//!    strike: 100.0,
//!    time_to_maturity: 1.0,
//!    risk_free_rate: 0.05,
//!    volatility: 0.2,
//!    simulations: 10000,
//!    ..Default::default()
//! };
//!
//! let price = mc_option.price(OptionType::Call);
//! println!("Option price: {}", price);
//! ```
use super::{Greeks, Option, OptionPricing, OptionStyle, OptionType};
use rand::rng;
use rand_distr::{Distribution, Normal};

/// A struct representing a Monte Carlo option.
#[derive(Debug, Default, Clone)]
pub struct MonteCarloOption {
    pub style: OptionStyle,
    pub spot: f64,
    pub strike: f64,
    pub time_to_maturity: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub simulations: usize,
}

impl OptionPricing for MonteCarloOption {
    fn price(&self, option_type: OptionType) -> f64 {
        let mut rng = rng();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut payoff_sum = 0.0;

        for _ in 0..self.simulations {
            let z = normal.sample(&mut rng);
            let st = self.spot
                * ((self.risk_free_rate - 0.5 * self.volatility.powi(2)) * self.time_to_maturity)
                    .exp()
                * (self.volatility * self.time_to_maturity.sqrt() * z).exp();
            let payoff = self.payoff(st, option_type);
            payoff_sum += payoff;
        }

        let discount_factor = (-self.risk_free_rate * self.time_to_maturity).exp();
        discount_factor * payoff_sum / self.simulations as f64
    }

    fn implied_volatility(&self, market_price: f64, option_type: OptionType) -> f64 {
        let mut sigma = 0.2; // Initial guess
        let tolerance = 1e-5;
        let max_iterations = 100;
        for _ in 0..max_iterations {
            let price = self.price(option_type);
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

impl Greeks for MonteCarloOption {
    fn delta(&self, option_type: OptionType) -> f64 {
        // Implement the delta calculation using finite differences
        let epsilon = 1e-4;
        let original_spot = self.spot;
        let mut option_up = self.clone();
        option_up.spot = original_spot + epsilon;
        let price_up = option_up.price(option_type);

        let mut option_down = self.clone();
        option_down.spot = original_spot - epsilon;
        let price_down = option_down.price(option_type);

        (price_up - price_down) / (2.0 * epsilon)
    }

    fn gamma(&self, option_type: OptionType) -> f64 {
        // Implement the gamma calculation using finite differences
        let epsilon = 1e-4;
        let original_spot = self.spot;
        let mut option_up = self.clone();
        option_up.spot = original_spot + epsilon;
        let price_up = option_up.price(option_type);

        let mut option_down = self.clone();
        option_down.spot = original_spot - epsilon;
        let price_down = option_down.price(option_type);

        let price = self.price(option_type);
        (price_up - 2.0 * price + price_down) / (epsilon * epsilon)
    }

    fn theta(&self, option_type: OptionType) -> f64 {
        // Implement the theta calculation using finite differences
        let epsilon = 1e-4;
        let original_time_to_maturity = self.time_to_maturity;
        let mut option_up = self.clone();
        option_up.time_to_maturity = original_time_to_maturity + epsilon;
        let price_up = option_up.price(option_type);

        let mut option_down = self.clone();
        option_down.time_to_maturity = original_time_to_maturity - epsilon;
        let price_down = option_down.price(option_type);

        (price_down - price_up) / (2.0 * epsilon)
    }

    fn vega(&self, option_type: OptionType) -> f64 {
        // Implement the vega calculation using finite differences
        let epsilon = 1e-4;
        let original_volatility = self.volatility;
        let mut option_up = self.clone();
        option_up.volatility = original_volatility + epsilon;
        let price_up = option_up.price(option_type);

        let mut option_down = self.clone();
        option_down.volatility = original_volatility - epsilon;
        let price_down = option_down.price(option_type);

        (price_up - price_down) / (2.0 * epsilon)
    }

    fn rho(&self, option_type: OptionType) -> f64 {
        // Implement the rho calculation using finite differences
        let epsilon = 1e-4;
        let original_risk_free_rate = self.risk_free_rate;
        let mut option_up = self.clone();
        option_up.risk_free_rate = original_risk_free_rate + epsilon;
        let price_up = option_up.price(option_type);

        let mut option_down = self.clone();
        option_down.risk_free_rate = original_risk_free_rate - epsilon;
        let price_down = option_down.price(option_type);

        (price_up - price_down) / (2.0 * epsilon)
    }
}

impl Option for MonteCarloOption {
    fn style(&self) -> &OptionStyle {
        &self.style
    }
}
