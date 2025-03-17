//! Module for Monte Carlo option pricing model.
//!
//! This module provides a Monte Carlo simulation model for pricing various types of options.
//! The Monte Carlo method is a statistical technique that uses random sampling to estimate the
//! expected value of an option's payoff.
//!
//! ## Characteristics
//!
//! - **Time to Maturity**: The time horizon (in years) for the option.
//! - **Risk-Free Rate**: The risk-free interest rate (e.g., 0.05 for 5%).
//! - **Volatility**: The volatility of the underlying asset (e.g., 0.2 for 20%).
//! - **Simulations**: The number of simulations to run.
//! - **Steps**: The number of steps in each simulation.
//! - **Averaging Method**: The method used to average the simulated prices (geometric or arithmetic).
//!
//! ## Example
//!
//! ```rust
//! use quantrs::options::{MonteCarloModel, Option, Instrument, OptionType, AvgMethod};
//!
//! let instrument = Instrument::new().with_spot(100.0);
//! let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
//! let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000, 252, AvgMethod::Arithmetic);
//!
//! let price = model.price(&option);
//! println!("Monte Carlo Call Price: {}", price);
//! ```

use crate::options::{Option, OptionPricing, OptionStyle, SimMethod};
use rand::rngs::ThreadRng;

/// Enum for averaging methods.
#[derive(Debug, Default, Clone, Copy)]
pub enum AvgMethod {
    Geometric,
    #[default]
    Arithmetic,
}

/// A struct representing a Monte Carlo Simulation model for option pricing.
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
    /// Number of steps in the simulation.
    pub steps: usize,
    /// average method
    pub method: AvgMethod,
}

impl MonteCarloModel {
    /// Create a new `MonteCarloModel`.
    pub fn new(
        time_to_maturity: f64,
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
        steps: usize,
        method: AvgMethod,
    ) -> Self {
        Self {
            time_to_maturity,
            risk_free_rate,
            volatility,
            simulations,
            steps: steps.max(1),
            method,
        }
    }

    /// Create a new `MonteCarloModel` with the geometric averaging method.
    pub fn geometric(
        time_to_maturity: f64,
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
        steps: usize,
    ) -> Self {
        Self::new(
            time_to_maturity,
            risk_free_rate,
            volatility,
            simulations,
            steps,
            AvgMethod::Geometric,
        )
    }

    /// Create a new `MonteCarloModel` with the arithmetic averaging method.
    pub fn arithmetic(
        time_to_maturity: f64,
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
        steps: usize,
    ) -> Self {
        Self::new(
            time_to_maturity,
            risk_free_rate,
            volatility,
            simulations,
            steps,
            AvgMethod::Arithmetic,
        )
    }
}

impl OptionPricing for MonteCarloModel {
    fn price<T: Option>(&self, option: &T) -> f64 {
        match option.style() {
            OptionStyle::European => self.price_european(option),
            OptionStyle::Basket => self.price_basket(option),
            OptionStyle::Rainbow(_) => self.price_rainbow(option),
            OptionStyle::Barrier(_) => self.price_barrier(option),
            OptionStyle::DoubleBarrier(_, _) => self.price_double_barrier(option),
            OptionStyle::Asian(_) => self.price_asian(option),
            OptionStyle::Lookback(_) => self.price_lookback(option),
            OptionStyle::Binary(_) => self.price_binary(option),
            _ => panic!("Unsupported option style"),
        }
    }

    fn implied_volatility<T: Option>(&self, _option: &T, _market_price: f64) -> f64 {
        unimplemented!()
    }
}

impl MonteCarloModel {
    fn price_european<T: Option>(&self, option: &T) -> f64 {
        let mut rng = rand::rng();
        let mut total_price = 0.0;
        for _ in 0..self.simulations {
            let simulated_price = option.instrument().simulate_geometric_brownian_motion(
                &mut rng,
                self.volatility,
                self.time_to_maturity,
                self.risk_free_rate,
                self.steps,
            );
            total_price += option.payoff(Some(simulated_price));
        }
        (total_price / self.simulations as f64)
            * (-self.risk_free_rate * self.time_to_maturity).exp()
    }

    fn price_asian<T: Option>(&self, option: &T) -> f64 {
        let mut rng: ThreadRng = rand::rng();
        let mut sum = 0.0;
        let mut min_spot = f64::MAX;
        let mut max_spot = f64::MIN;

        for _ in 0..self.simulations {
            // Simulate random asset prices and calculate the average price (discounted)
            let avg_price = option.instrument().simulate_geometric_average(
                &mut rng,
                SimMethod::Log,
                self.volatility,
                self.time_to_maturity,
                self.risk_free_rate,
                self.steps,
            );

            // Update min and max spot prices
            min_spot = min_spot.min(avg_price);
            max_spot = max_spot.max(avg_price);

            // Calculate each payoff
            sum += (-(self.risk_free_rate - option.instrument().continuous_dividend_yield)
                * self.time_to_maturity)
                .exp()
                * option.payoff(Some(avg_price));
        }

        println!("Min Spot: {}, Max Spot: {}", min_spot, max_spot);

        // Calculate the average payoff and discount it to present value
        sum / self.simulations as f64
    }

    fn price_basket<T: Option>(&self, option: &T) -> f64 {
        unimplemented!()
    }

    fn price_rainbow<T: Option>(&self, option: &T) -> f64 {
        unimplemented!()
    }

    fn price_barrier<T: Option>(&self, option: &T) -> f64 {
        unimplemented!()
    }

    fn price_double_barrier<T: Option>(&self, option: &T) -> f64 {
        unimplemented!()
    }

    fn price_lookback<T: Option>(&self, option: &T) -> f64 {
        unimplemented!()
    }

    fn price_binary<T: Option>(&self, option: &T) -> f64 {
        unimplemented!()
    }
}
