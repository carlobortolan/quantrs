//!
//! This crate is designed to be simple and easy to use library for options pricing, portfolio optimization, and risk analysis. It is not intended to be a full-fledged quantitative finance library.
//! The goal is to provide library for pricing options, calculating greeks, and performing basic risk analysis without the need to write complex code or have a PhD in reading quantlib documentation.
//! The library is still in the early stages of development, and many features are not yet implemented.
//!
//! Compared to other popular options pricing libraries, quantrs is _significantly_ faster:
//! - **29x faster** than `QuantLib` (python bindings)
//! - **113x faster** than `py_vollib`
//! - **15x faster** than `RustQuant`
//! - **2.7x faster** than `Q-Fin`
//!
//! _You can find the benchmarks at [quantrs.pages.dev/report](https://quantrs.pages.dev/report/)_.
//! ```

#![allow(unused_variables)]

#[macro_use]
mod macros {
    pub mod logging_macros;
    pub mod math_macros;
    pub mod validation_macros;
}

pub mod data;
pub mod fixed_income;
pub mod options;

pub use data::*;
pub use fixed_income::*;
pub use options::*;

#[cfg(feature = "python")]
pub mod python;
