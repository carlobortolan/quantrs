use crate::options::{Greeks, Option, OptionPricing, OptionType};
use rand_distr::{Distribution, Normal};

/// A struct representing a Monte Carlo option.
#[derive(Debug, Default, Clone)]
pub struct MonteCarloModel {
    pub time_to_maturity: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
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
