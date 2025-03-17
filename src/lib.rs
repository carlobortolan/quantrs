//!
//! This crate is designed to be simple and easy to use library for options pricing, portfolio optimization, and risk analysis. It is not intended to be a full-fledged quantitative finance library.
//! The goal is to provide library for pricing options, calculating greeks, and performing basic risk analysis without the need to write complex code or have a PhD in reading quantlib documentation.
//! The library is still in the early stages of development, and many features are not yet implemented.
//!
//! There are no benchmarks yet, but it is expected to be faster than FinancePy, optlib, QuantScale and easier to use than RustQuant or QuantLib.
//!
//! ## Options Pricing
//!
//! For now quantrs only supports options pricing. The following features are available:
//!
//! - Option types: European, American, Binary Cash-or-Nothing, Binary Asset-or-Nothing
//! - Option pricing: Black-Scholes, Binomial Tree, Monte Carlo Simulation
//! - Greeks: Delta, Gamma, Theta, Vega, Rho
//! - Implied volatility
//!
//! ```rust
//! use quantrs::options::*;
//!
//! let option = BinaryOption::cash_or_nothing(Instrument::new().with_spot(100.0), 85.0, OptionType::Call);
//! let model = BlackScholesModel::new(0.78, 0.05, 0.2);
//! let price = model.price(&option);
//! let greeks = OptionGreeks::calculate(&model, option);
//! ```

#![allow(unused_variables)]

pub mod options;
