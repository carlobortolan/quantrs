//! Module for calculating the price and Greeks of various types of options.
//!
//! ## Supported models
//!
//! - [Black-Scholes Option Pricing Model](models/black_scholes/struct.BlackScholesModel.html)
//! - [Binomial Option Pricing Model](models/binomial_tree/struct.BinomialTreeModel.html)
//! - [Monte Carlo Option Pricing Model](models/monte_carlo/struct.MonteCarloModel.html)
//!
//! ## Greek calculations
//!
//! This module also provides implementations of the Greeks for each option pricing model.
//! See the [Greeks](trait.Greeks.html) trait for more information.

pub use self::types::*;
pub use greeks::*;
pub use instrument::*;
pub use models::*;
pub use traits::*;

mod greeks;
mod instrument;
mod models;
mod traits;
mod types;
