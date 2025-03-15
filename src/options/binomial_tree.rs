//! Module for binomial tree option pricing model.
//!
//! Binomial tree is a discrete-time option pricing model that provides a simple and efficient way to price options.
//! The model is based on the assumption that the price of the underlying asset can move up or down by a certain factor at each step.
//! The option price is calculated by working backwards through the tree, starting from the final step where the option payoff is known.
//!
//! ## Formula
//!
//! The price of an option using the binomial tree model is calculated by working backwards through the tree, starting from the final step where the option payoff is known.
//!
//! At each node, the option price is calculated as the discounted expected value of the option prices at the next step.
//!
//! ```text
//! C = e^(-rΔt) * (p * Cu + (1 - p) * Cd)
//! ```
//!
//! where:
//! - `C` is the option price at the current node.
//! - `r` is the risk-free interest rate.
//! - `Δt` is the time step (T / N).
//! - `p` is the risk-neutral probability of an upward movement.
//! - `Cu` is the option price at the next node if the price goes up.
//! - `Cd` is the option price at the next node if the price goes down.
//!
//! The risk-neutral probability `p` is calculated as:
//!
//! ```text
//! p = (e^(rΔt) - d) / (u - d)
//! ```
//!
//! where:
//! - `u` is the factor by which the price increases.
//! - `d` is the factor by which the price decreases.
//!
//! The factors `u` and `d` are calculated as:
//!
//! ```text
//! u = e^(σ√Δt)
//! d = 1 / u
//! ```
//!
//! where:
//! - `σ` is the volatility of the underlying asset.
//!
//! The payoff at maturity is calculated as:
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
//! - [Wikipedia - Binomial options pricing model](https://en.wikipedia.org/wiki/Binomial_options_pricing_model)
//! - [Investopedia - Binomial Option Pricing Model](https://www.investopedia.com/terms/b/binomialoptionpricing.asp)
//! - [Options, Futures, and Other Derivatives (9th Edition)](https://www.pearson.com/store/p/options-futures-and-other-derivatives/P1000000000000013194)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::{BinomialTreeOption, OptionType, OptionPricing};
//!
//! let bt_option = BinomialTreeOption {
//!     spot: 100.0,
//!     strike: 100.0,
//!     time_to_maturity: 1.0,
//!     risk_free_rate: 0.05,
//!     volatility: 0.2,
//!     steps: 100,
//!     ..Default::default()
//! };
//!
//! let price = bt_option.price(OptionType::Call);
//! println!("Option price: {}", price);
//! ```
use super::{Greeks, Option, OptionPricing, OptionStyle, OptionType};

/// Binomial tree option pricing model.
#[derive(Debug, Default)]
pub struct BinomialTreeOption {
    /// Base data for the option.
    pub style: OptionStyle,
    /// Current price of the underlying asset.
    pub spot: f64,
    /// Strike price of the option (aka exercise price).
    pub strike: f64,
    /// Time horizon (in years).
    pub time_to_maturity: f64,
    /// Risk-free interest rate (e.g., 0.05 for 5%).
    pub risk_free_rate: f64,
    /// Volatility of the underlying asset (e.g., 0.2 for 20%).
    pub volatility: f64,
    /// Number of steps in the binomial tree.
    pub steps: usize,
}

impl OptionPricing for BinomialTreeOption {
    fn price(&self, option_type: OptionType) -> f64 {
        // Multiplicative up-/downward movements of an asset in a single step of the binomial tree
        let dt = self.time_to_maturity / self.steps as f64;
        let u = (self.volatility * dt.sqrt()).exp();
        let d = 1.0 / u;

        // Risk-neutral probability of an upward movement for a call option
        let p = ((self.risk_free_rate * dt).exp() - d) / (u - d);

        // Discount factor for each step
        let discount_factor = (-self.risk_free_rate * dt).exp();

        // Initialize option values at maturity
        let mut option_values: Vec<f64> = (0..=self.steps)
            .map(|i| {
                self.payoff(
                    self.spot * u.powi(i as i32) * d.powi((self.steps - i) as i32),
                    option_type,
                )
            })
            .collect();

        // Backward induction
        for step in (0..self.steps).rev() {
            for i in 0..=step {
                let expected_value =
                    discount_factor * (p * option_values[i + 1] + (1.0 - p) * option_values[i]);

                if self.style == OptionStyle::American {
                    let early_exercise = self.payoff(
                        self.spot * u.powi(i as i32) * d.powi((step - i) as i32),
                        option_type,
                    );
                    option_values[i] = expected_value.max(early_exercise);
                } else {
                    option_values[i] = expected_value;
                }
            }
        }

        if self.style == OptionStyle::American {
            option_values[0].max(self.strike - self.spot) // TODO: Change to max(0.0, self.payoff(self.spot, option_type))
        } else {
            option_values[0] // Return the root node value
        }
    }

    fn implied_volatility(&self, _market_price: f64, _option_type: OptionType) -> f64 {
        0.2 // TODO: Placeholder value
    }

    fn strike(&self) -> f64 {
        self.strike
    }
}

impl Greeks for BinomialTreeOption {
    fn delta(&self, option_type: OptionType) -> f64 {
        0.5 // TODO: Placeholder value
    }

    fn gamma(&self, option_type: OptionType) -> f64 {
        0.1 // TODO: Placeholder value
    }

    fn theta(&self, option_type: OptionType) -> f64 {
        -0.01 // TODO: Placeholder value
    }

    fn vega(&self, option_type: OptionType) -> f64 {
        0.2 // TODO: Placeholder value
    }

    fn rho(&self, option_type: OptionType) -> f64 {
        0.05 // TODO: Placeholder value
    }
}

impl Option for BinomialTreeOption {
    fn style(&self) -> &OptionStyle {
        &self.style
    }
}
