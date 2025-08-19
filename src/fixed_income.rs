//! Module for fixed income securities
//!
//! Provides types and traits for various fixed income instruments, including bonds, cash flows, and day count conventions.
//!
//! This module includes:
//!
//! - **Bond Pricing**: Functions for calculating the present value of bonds, including yield to maturity and duration.
//! - **Bonds**: Definitions for different types of bonds
//! - **Cash Flow**: Structures and methods for handling cash flows associated with fixed income securities.
//! - **Day Count Conventions**: Implementations of various day count conventions used in fixed income calculations.
//! - **Types**: Additional types for specialized fixed income instruments.
//!
//! ## Supported instruments
//!
//! - [Treasury Bonds](bonds/struct.TreasuryBond.html)
//! - [Corporate Bonds](bonds/struct.CorporateBond.html)
//! - [Floating Rate Bonds](bonds/struct.FloatingRateBond.html)
//! - [Zero-Coupon Bonds](bonds/struct.ZeroCouponBond.html)

pub use self::bond_pricing::*;
pub use self::bonds::*;
pub use self::cashflow::*;
pub use self::types::*;
pub use traits::*;

mod bond_pricing;
mod bonds;
mod cashflow;
mod day_count;
mod traits;
mod types;
