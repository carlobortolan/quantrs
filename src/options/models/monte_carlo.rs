//! Module for Monte Carlo option pricing model.

use crate::options::{Instrument, Option, OptionPricing, OptionStyle};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal};

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

    fn simulate_asset_price(&self, instrument: &Instrument, rng: &mut ThreadRng) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let z = normal.sample(rng);
        instrument.spot
            * ((self.risk_free_rate
                - instrument.continuous_dividend_yield
                - 0.5 * self.volatility.powi(2))
                * self.time_to_maturity
                + self.volatility * z * self.time_to_maturity.sqrt())
            .exp()
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
            OptionStyle::Asian => self.price_asian(option),
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
            let simulated_price = self.simulate_asset_price(option.instrument(), &mut rng);
            total_price += option.payoff(Some(simulated_price));
        }
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

    fn price_asian<T: Option>(&self, option: &T) -> f64 {
        unimplemented!()
    }

    fn price_lookback<T: Option>(&self, option: &T) -> f64 {
        unimplemented!()
    }

    fn price_binary<T: Option>(&self, option: &T) -> f64 {
        unimplemented!()
    }
}
