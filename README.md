
# quantrs  [![CI](https://github.com/carlobortolan/quantrs/actions/workflows/ci.yml/badge.svg)](https://github.com/carlobortolan/quantrs/actions/workflows/ci.yml)  [![codecov](https://codecov.io/gh/carlobortolan/quantrs/graph/badge.svg?token=NJ4HW3OQFY)](https://codecov.io/gh/carlobortolan/quantrs)

Quantrs is a tiny quantitative finance library for Rust. It is designed to be simple and easy to use, with a focus on performance and correctness. It is still in the early stages of development, so expect bugs and breaking changes.

## Features

### Core Features

- [ ] Options pricing
  - [ ] Black-Scholes
  - [ ] Binomial tree
  - [ ] Monte Carlo simulation
  - [ ] Greeks 

### Optional Features

- [ ] Data retrieval
  - [ ] Yahoo Finance
  - [ ] Alpha Vantage
  - [ ] Quandl
  - [ ] IEX Cloud
- [ ] Fixed income & IR
  - [ ] Bond pricing
  - [ ] Duration
  - [ ] Convexity
  - [ ] Yield curve
  - [ ] Term structure
  - [ ] Forward rates
  - [ ] Interest rate models
- [ ] Time series analysis
  - [ ] Moving averages
  - [ ] Volatility
  - [ ] Correlation
  - [ ] Cointegration
  - [ ] ARIMA
  - [ ] GARCH
  - [ ] Kalman filter
- [ ] Portfolio optimization
  - [ ] Mean-variance optimization
  - [ ] Black-Litterman model
  - [ ] Risk parity
  - [ ] Minimum variance
  - [ ] Maximum diversification


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
quantrs = "0.1"
```

## Minimum supported Rust version (MSRV)

This crate requires a Rust version of 1.65.0 or higher. Increases in MSRV will be considered a semver non-breaking API change and require a version increase (PATCH until 1.0.0, MINOR after 1.0.0).

## Contributing

If you find any bugs or have suggestions for improvement, please open a new issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.

---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](mailto:carlobortolan@gmail.com)
