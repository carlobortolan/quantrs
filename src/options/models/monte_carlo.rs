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
//! use quantrs::options::{MonteCarloModel, OptionPricing, Instrument, OptionType, EuropeanOption};
//!
//! let instrument = Instrument::new().with_spot(100.0);
//! let option = EuropeanOption::new(instrument, 100.0, 1.0, OptionType::Call);
//! let model = MonteCarloModel::geometric(0.05, 0.2, 10_000, 252);
//!
//! let price = model.price(&option);
//! println!("Monte Carlo Call Price: {price}");
//! ```

use crate::options::{Option, OptionPricing, OptionStrategy, OptionStyle, SimMethod};
use rand::rngs::ThreadRng;
use rayon::prelude::*;

/// Enum for averaging methods.
#[derive(Debug, Default, Clone, Copy)]
pub enum AvgMethod {
    Geometric,
    Arithmetic,
    #[default]
    Brownian,
}

/// A struct representing a Monte Carlo Simulation model for option pricing.
#[derive(Debug, Default, Clone)]
pub struct MonteCarloModel {
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
    ///
    /// # Arguments
    ///
    /// * `risk_free_rate` - The risk-free interest rate (e.g., 0.05 for 5%).
    /// * `volatility` - The volatility of the underlying asset (e.g., 0.2 for 20%).
    /// * `simulations` - The number of simulations to run.
    /// * `steps` - The number of steps in the simulation.
    /// * `method` - The method used to average the simulated prices (geometric or arithmetic).
    ///
    /// # Returns
    ///
    /// A new `MonteCarloModel`.
    pub fn new(
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
        steps: usize,
        method: AvgMethod,
    ) -> Self {
        Self {
            risk_free_rate,
            volatility,
            simulations,
            steps: steps.max(1),
            method,
        }
    }

    /// Create a new `MonteCarloModel` with the geometric averaging method.
    ///
    /// # Arguments
    ///
    /// * `risk_free_rate` - The risk-free interest rate (e.g., 0.05 for 5%).
    /// * `volatility` - The volatility of the underlying asset (e.g., 0.2 for 20%).
    /// * `simulations` - The number of simulations to run.
    /// * `steps` - The number of steps in the simulation.
    ///
    /// # Returns
    ///
    /// A new `MonteCarloModel`.
    pub fn geometric(
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
        steps: usize,
    ) -> Self {
        Self::new(
            risk_free_rate,
            volatility,
            simulations,
            steps,
            AvgMethod::Geometric,
        )
    }

    /// Create a new `MonteCarloModel` with the arithmetic averaging method.
    ///
    /// # Arguments
    ///
    /// * `risk_free_rate` - The risk-free interest rate (e.g., 0.05 for 5%).
    /// * `volatility` - The volatility of the underlying asset (e.g., 0.2 for 20%).
    /// * `simulations` - The number of simulations to run.
    /// * `steps` - The number of steps in the simulation.
    ///
    /// # Returns
    ///
    /// A new `MonteCarloModel`.
    pub fn arithmetic(
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
        steps: usize,
    ) -> Self {
        Self::new(
            risk_free_rate,
            volatility,
            simulations,
            steps,
            AvgMethod::Arithmetic,
        )
    }

    /// Create a new `MonteCarloModel` using the geometric Brownian motion for averaging.
    ///
    /// # Arguments
    ///
    /// * `risk_free_rate` - The risk-free interest rate (e.g., 0.05 for 5%).
    /// * `volatility` - The volatility of the underlying asset (e.g., 0.2 for 20%).
    /// * `simulations` - The number of simulations to run.
    /// * `steps` - The number of steps in the simulation.
    ///
    /// # Returns
    ///
    /// A new `MonteCarloModel`.
    pub fn brownian(
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
        steps: usize,
    ) -> Self {
        Self::new(
            risk_free_rate,
            volatility,
            simulations,
            steps,
            AvgMethod::Brownian,
        )
    }

    /// Simulate price paths and compute the expected discounted payoff.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to price.
    ///
    /// # Returns
    ///
    /// The expected discounted payoff of the option.
    fn simulate_price_paths<T: Option>(&self, option: &T) -> f64 {
        let discount_factor = (-self.risk_free_rate * option.time_to_maturity()).exp();

        // Use parallel iteration to simulate multiple price paths
        let total_payoff: f64 = (0..self.simulations)
            .into_par_iter() // Rayon parallel iterator
            .map(|_| {
                let mut rng = rand::rng();
                let simulated_price = match self.method {
                    AvgMethod::Geometric => option.instrument().simulate_geometric_average(
                        &mut rng,
                        SimMethod::Log,
                        self.volatility,
                        option.time_to_maturity(),
                        self.risk_free_rate,
                        self.steps,
                    ),
                    AvgMethod::Arithmetic => option.instrument().simulate_arithmetic_average(
                        &mut rng,
                        SimMethod::Log,
                        self.volatility,
                        option.time_to_maturity(),
                        self.risk_free_rate,
                        self.steps,
                    ),
                    AvgMethod::Brownian => option.instrument().simulate_geometric_brownian_motion(
                        &mut rng,
                        self.volatility,
                        option.time_to_maturity(),
                        self.risk_free_rate,
                        self.steps,
                    ),
                };

                option.payoff(Some(simulated_price))
            })
            .sum();

        (total_payoff / self.simulations as f64) * discount_factor
    }
}

impl OptionPricing for MonteCarloModel {
    fn price<T: Option>(&self, option: &T) -> f64 {
        match option.style() {
            OptionStyle::European => self.simulate_price_paths(option),
            OptionStyle::Basket => self.simulate_price_paths(option),
            OptionStyle::Rainbow(_) => self.simulate_price_paths(option),
            OptionStyle::Barrier(_) => self.simulate_price_paths(option),
            OptionStyle::DoubleBarrier(_, _) => self.simulate_price_paths(option),
            OptionStyle::Asian(_) => self.price_asian(option),
            OptionStyle::Lookback(_) => self.price_asian(option),
            OptionStyle::Binary(_) => self.simulate_price_paths(option),
            _ => panic!("Monte Carlo model does not support this option style"),
        }
    }

    fn implied_volatility<T: Option>(&self, _option: &T, _market_price: f64) -> f64 {
        unimplemented!()
    }
}

impl MonteCarloModel {
    /// Simulate price paths and compute the expected discounted payoff for Asian options.
    ///
    /// # Arguments
    ///
    /// * `option` - The Asian option to price.
    ///
    /// # Returns
    ///
    /// The expected discounted payoff of the option.
    fn price_asian<T: Option>(&self, option: &T) -> f64 {
        let mut rng: ThreadRng = rand::rng();
        let mut sum = 0.0;
        let mut option_clone = option.clone();

        for _ in 0..self.simulations {
            // Get a mutable reference to the instrument
            let avg_price = {
                let instrument = option_clone.instrument_mut();
                match self.method {
                    AvgMethod::Geometric => instrument.simulate_geometric_average_mut(
                        &mut rng,
                        SimMethod::Log,
                        self.volatility,
                        option.time_to_maturity(),
                        self.risk_free_rate,
                        self.steps,
                    ),
                    AvgMethod::Arithmetic => instrument.simulate_arithmetic_average_mut(
                        &mut rng,
                        SimMethod::Log,
                        self.volatility,
                        option.time_to_maturity(),
                        self.risk_free_rate,
                        self.steps,
                    ),
                    _ => panic!("Invalid averaging method"),
                }
            };

            // Now call payoff after the mutable borrow is done
            sum += (-(self.risk_free_rate - option_clone.instrument().continuous_dividend_yield)
                * option_clone.time_to_maturity())
            .exp()
                * option_clone.payoff(Some(avg_price));
        }

        // Return average payoff
        sum / self.simulations as f64
    }
}

impl OptionStrategy for MonteCarloModel {}
