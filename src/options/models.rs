//! Module for various option pricing models.
//!
//! ## Supported models
//!
//! - [Black-Scholes Option Pricing Model](black_scholes/struct.BlackScholesModel.html)
//! - [Binomial Option Pricing Model](binomial_tree/struct.BinomialTreeModel.html)
//! - [Monte Carlo Option Pricing Model](monte_carlo/struct.MonteCarloModel.html)
//!
//! ## Greek calculations
//!
//! This module also provides implementations of the Greeks for each option pricing model.
//! See the [Greeks](options/trait.Greeks.html) trait for more information.

pub mod binomial_tree;
pub mod black_scholes;
pub mod monte_carlo;

pub use binomial_tree::BinomialTreeModel;
pub use black_scholes::BlackScholesModel;
pub use monte_carlo::MonteCarloModel;
