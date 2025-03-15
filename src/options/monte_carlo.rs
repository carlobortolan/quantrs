use super::{Greeks, Instrument, Option, OptionPricing, OptionStyle, OptionType};
use rand_distr::{Distribution, Normal};

/// A struct representing a Monte Carlo option.
#[derive(Debug, Default, Clone)]
pub struct MonteCarloOption {
    pub style: OptionStyle,
    pub instrument: Instrument,
    pub strike: f64,
    pub time_to_maturity: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub simulations: usize,
}

impl MonteCarloOption {
    /// Create a new `MonteCarloOption`.
    pub fn new(
        instrument: Instrument,
        strike: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        volatility: f64,
        simulations: usize,
        style: OptionStyle,
    ) -> Self {
        Self {
            instrument,
            strike,
            time_to_maturity,
            risk_free_rate,
            volatility,
            simulations,
            style: OptionStyle::European,
            ..Default::default()
        }
    }
}

impl OptionPricing for MonteCarloOption {
    fn price(&self, option_type: OptionType) -> f64 {
        let mut rng = rand::rng();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let dt_sqrt = self.time_to_maturity.sqrt();
        let drift = (self.risk_free_rate - 0.5 * self.volatility.powi(2)) * self.time_to_maturity;
        let discount_factor = (-self.risk_free_rate * self.time_to_maturity).exp();

        let sum_payoff: f64 = (0..self.simulations)
            .map(|_| {
                let z = normal.sample(&mut rng);
                let st = self.instrument.spot * (drift + self.volatility * dt_sqrt * z).exp();
                match option_type {
                    OptionType::Call => (st - self.strike).max(0.0),
                    OptionType::Put => (self.strike - st).max(0.0),
                }
            })
            .sum();

        discount_factor * (sum_payoff / self.simulations as f64)
    }

    fn implied_volatility(&self, market_price: f64, option_type: OptionType) -> f64 {
        // Return 0.0 for unrealistic market prices
        if market_price <= 0.0 || market_price > self.instrument.spot {
            return 0.0;
        }

        let mut sigma = 0.2; // Initial guess
        let tolerance = 1e-5;
        let max_iterations = 100;
        let mut prev_sigma = sigma;

        for _ in 0..max_iterations {
            let price = self.price(option_type);
            let vega = self.vega(option_type);
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

    fn strike(&self) -> f64 {
        self.strike
    }
}

impl Greeks for MonteCarloOption {
    fn delta(&self, option_type: OptionType) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut opt = self.clone();
            opt.instrument.spot += epsilon;
            opt.price(option_type)
        };
        let price_down = {
            let mut opt = self.clone();
            opt.instrument.spot -= epsilon;
            opt.price(option_type)
        };
        (price_up - price_down) / (2.0 * epsilon)
    }

    fn gamma(&self, option_type: OptionType) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut opt = self.clone();
            opt.instrument.spot += epsilon;
            opt.price(option_type)
        };
        let price_down = {
            let mut opt = self.clone();
            opt.instrument.spot -= epsilon;
            opt.price(option_type)
        };
        let price = self.price(option_type);
        (price_up - 2.0 * price + price_down) / (epsilon * epsilon)
    }

    fn theta(&self, option_type: OptionType) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut opt = self.clone();
            opt.time_to_maturity += epsilon;
            opt.price(option_type)
        };
        let price_down = {
            let mut opt = self.clone();
            opt.time_to_maturity -= epsilon;
            opt.price(option_type)
        };
        (price_down - price_up) / (2.0 * epsilon)
    }

    fn vega(&self, option_type: OptionType) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut opt = self.clone();
            opt.volatility += epsilon;
            opt.price(option_type)
        };
        let price_down = {
            let mut opt = self.clone();
            opt.volatility -= epsilon;
            opt.price(option_type)
        };
        (price_up - price_down) / (2.0 * epsilon)
    }

    fn rho(&self, option_type: OptionType) -> f64 {
        let epsilon = 1e-4;
        let price_up = {
            let mut opt = self.clone();
            opt.risk_free_rate += epsilon;
            opt.price(option_type)
        };
        let price_down = {
            let mut opt = self.clone();
            opt.risk_free_rate -= epsilon;
            opt.price(option_type)
        };
        (price_up - price_down) / (2.0 * epsilon)
    }
}

impl Option for MonteCarloOption {
    fn style(&self) -> &OptionStyle {
        &self.style
    }

    fn instrument(&self) -> &Instrument {
        &self.instrument
    }
}
