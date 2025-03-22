# Outlook for Quantrs

This document outlines the planned features and improvements for the `quantrs` library. The goal is to provide a comprehensive and intuitive quantitative finance library for Rust, with support for various financial instruments and models.

## Planned Features

### Black-Scholes

- [x] European Options Price and Greeks
- [x] Cash or Nothing Binary Options Price and Greeks
- [x] Asset or Nothing Binary Options Price and Greeks
- [x] Binary Cash-or-Nothing Options Price and Greeks
- [x] Binary Asset-or-Nothing Options Price and Greeks
- [x] Rainbow Options Price and Greeks
- [ ] FX European Options Price and Greeks
- [ ] Swaption Price and Greeks
- [ ] Caplet/Floorlet Price and Greeks
- [ ] Cap/Floor Price and Greeks

### Black-76

- [ ] European Options Price and Greeks
- [ ] Cash or Nothing Binary Options Price and Greeks
- [ ] Asset or Nothing Binary Options Price and Greeks

### Lattice

- [x] European Options Price and Greeks
- [x] American Options Price and Greeks
- [x] Cash or Nothing Binary Options Price and Greeks
- [x] Asset or Nothing Binary Options Price and Greeks
- [ ] Bermudan Options Price and Greeks
- [ ] Basket Options Price and Greeks
- [x] Rainbow Options Price and Greeks
- [ ] Barrier Options Price and Greeks
- [ ] Double Barrier Options Price and Greeks

### Monte-Carlo

- [x] European Options Price and Greeks
- [x] Cash or Nothing Binary Options Price and Greeks
- [x] Asset or Nothing Binary Options Price and Greeks
- [ ] Basket Options Price and Greeks
- [x] Rainbow Options Price and Greeks
- [ ] Barrier Options Price and Greeks
- [ ] Double Barrier Options Price and Greeks
- [x] Asian Options Price and Greeks
- [ ] Lookback Options Price and Greeks

### Finite Difference

- [ ] European Options Price and Greeks
- [ ] American Options Price and Greeks
- [ ] Barrier Options Price and Greeks
- [ ] Asian Options Price and Greeks
- [ ] Lookback Options Price and Greeks

### Heston

- [ ] European Options Price and Greeks
- [ ] Barrier Options Price and Greeks
- [ ] Double Barrier Options Price and Greeks
- [ ] Asian Options Price and Greeks
- [ ] Lookback Options Price and Greeks
- [ ] Binary Cash-or-Nothing Options Price and Greeks
- [ ] Binary Asset-or-Nothing Options Price and Greeks

### Greeks Calculation

- [ ] Implement missing 1st order Greeks:

  - [ ] Lambda
  - [ ] Epsilon

- [ ] Implement missing 2nd order Greeks:

  - [ ] Vanna
  - [ ] Charm
  - [ ] Vomma
  - [ ] Veta
  - [ ] Vera

- [ ] Implement missing 3rd order Greeks:

  - [ ] Speed
  - [ ] Zomma
  - [ ] Color
  - [ ] Ultima
  - [ ] Parmicharma

### FX

- [ ] FX Options under Black Scholes: Price and Greeks Calculator
- [ ] FX Options under Black Scholes: Price and Greeks with Analysis of Deltas
- [ ] FX Options under Black Scholes: ATM Strikes and Deltas
- [ ] FX Strike from Delta and Volatility
- [ ] FX Smile Volatility for a Given Delta
- [ ] FX Smile Volatility for a given Strike
- [ ] FX Smile Curve
- [ ] FX Smile Strangle from Market Strangle
- [ ] FX Market Strangle from Smile Strangle

### Basket

- [ ] Vasicek's LHP model: Loss Distribution
- [ ] Vasicek's LHP model: Single Tranche CDO Price and Greeks Calculator
- [ ] Vasicek's LHP model: Base Correlation Calculator
- [ ] Vasicek's LHP model: Single Tranche CDO Price and Greeks Analysis
- [ ] Vasicek's LHP model: Single Tranche CDO's Spread
- [ ] Vasicek's HP model: Kth-to-Default Protection Price and Greeks Calculator
- [ ] Vasicek's HP model: Kth-to-Default Protection Price and Greeks Analysis
- [ ] Markowitz Efficient Frontier

### Short Rates

- [ ] Merton Short Rate model
- [ ] Vasicek Short Rate model
- [ ] CIR Short Rate model
- [ ] Ho Lee Short Rate model
- [ ] Hull White Short Rate model (One Factor)

### Yield Curves

- [ ] Holiday generator
- [ ] Day count conventions: Calculate Maturity Date /Add Business Days
- [ ] Daycount Conventions: Calculate No of Days between two Dates
- [ ] Daycount: Calculate cash flow dates and daycount between two dates per given frequency
- [ ] Yield Curve: Interpolation Analysis
- [ ] Yield Curve Interpolation using LIBOR
- [ ] Yield Curve Interpolation using FRA
- [ ] Yield Curve Interpolation using Swap

### Rates

- [ ] Par swap rate and Greeks Calculator
- [ ] Swap Price and Greeks Calculator
- [ ] Swaption and underlying swap
- [ ] Caplet Floorlet Price and Greeks Calculator
- [ ] Cap Floor Price and Greeks Calculator

### Data Retrieval

- [ ] Integrate with financial data providers:
  - [ ] Yahoo Finance
  - [ ] Alpha Vantage
  - [ ] Quandl
  - [ ] IEX Cloud

### Fixed Income & Interest Rate Models

- [ ] Support for fixed income instruments:
  - [ ] Bond pricing
  - [ ] Duration
  - [ ] Convexity
  - [ ] Yield curve construction
  - [ ] Term structure modeling
  - [ ] Forward rate agreements
  - [ ] Interest rate models (e.g., Vasicek, CIR)

### Time Series Analysis

- [ ] Implement time series analysis tools:
  - [ ] Moving averages
  - [ ] Volatility estimation
  - [ ] Correlation and cointegration
  - [ ] ARIMA models
  - [ ] GARCH models
  - [ ] Kalman filter

### Portfolio Optimization

- [ ] Support for portfolio optimization techniques:
  - [ ] Mean-variance optimization
  - [ ] Black-Litterman model
  - [ ] Risk parity
  - [ ] Minimum variance
  - [ ] Maximum diversification

### Rates Space

- [ ] Yield curve modelling, parameterization, linear swap pricing/risk
- [ ] Inflation modelling and swap/linker pricing
- [ ] Vanilla swaptions, SABR, YCSO, mid-curve swaptions, CMS, Bermudans
- [ ] Listed rates futures and options
- [ ] Bond/Repo Pricing
- [ ] MBS modelling and integration into OTC risk
- [ ] Risk based PnL across Fixed Income assets

## License

This project is licensed under either of:

- [MIT license](LICENSE-MIT) or
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](mailto:carlobortolan@gmail.com)
