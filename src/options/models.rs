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

pub use binomial_tree::BinomialTreeModel;
pub use black_76::Black76Model;
pub use black_scholes::BlackScholesModel;
pub use finite_diff::FiniteDiffModel;
pub use heston::HestonModel;
pub use monte_carlo::MonteCarloModel;

mod binomial_tree;
mod black_76;
mod black_scholes;
mod finite_diff;
mod heston;
mod monte_carlo;
