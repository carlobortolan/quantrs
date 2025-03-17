//! Module for Monte Carlo option pricing model.

use crate::options::{Option, OptionPricing, OptionStyle, SimMethod};
use rand::rngs::ThreadRng;

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
}

impl MonteCarloModel {
    /// Create a new `MonteCarloModel`.
    pub fn new(
        time_to_maturity: f64,
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
        steps: usize,
    ) -> Self {
        Self {
            time_to_maturity,
            risk_free_rate,
            volatility,
            simulations,
            steps: steps.max(1),
        }
    }

    /// Create a new `MonteCarloModel` with the default values.
    pub fn default(time_to_maturity: f64, risk_free_rate: f64, volatility: f64) -> Self {
        Self::new(time_to_maturity, risk_free_rate, volatility, 10_000, 20)
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
        let mut total_price = 0.0;
        let mut min_avg_price = f64::MAX;
        let mut max_avg_price = f64::MIN;

        for _ in 0..self.simulations {
            // Simulate random asset prices and calculate the average price
            let avg_price = option.instrument().simulate_arithmetic_average(
                &mut rng,
                SimMethod::Log,
                self.volatility,
                self.time_to_maturity,
                self.risk_free_rate,
                self.steps,
            );

            // Update min and max average prices
            if avg_price < min_avg_price {
                min_avg_price = avg_price;
            }
            if avg_price > max_avg_price {
                max_avg_price = avg_price;
            }

            // Calculate each payoff
            total_price += option.payoff(Some(avg_price));
        }

        // Calculate the average payoff and discount it to present value
        (total_price / self.simulations as f64)
            * (-self.risk_free_rate * self.time_to_maturity).exp()
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
