//! Traits for stock market data providers.

#![allow(async_fn_in_trait)]
use crate::data::{DataError, Fundamentals, Quote};

pub trait QuoteProvider {
    async fn get_stock_quote(&self, symbol: &str) -> Result<Quote, DataError>;
}

pub trait FundamentalsProvider {
    async fn get_company_overview(&self, symbol: &str) -> Result<Fundamentals, DataError>;
}
