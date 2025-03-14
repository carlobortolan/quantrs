# quantrs

![tests][actions-test-badge]
[![MIT licensed][license-badge]](./LICENSE.md)
[![Crate][crates-badge]][crates-url]
[![docs.rs][docsrs-badge]][docs-url]
[![codecov-quantrs][codecov-badge]][codecov-url]
![Crates.io MSRV][crates-msrv-badge]

[actions-test-badge]: https://github.com/carlobortolan/quantrs/actions/workflows/ci.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/quantrs.svg
[crates-url]: https://crates.io/crates/quantrs
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[docsrs-badge]: https://img.shields.io/docsrs/quantrs
[docs-url]: https://docs.rs/quantrs/*/quantrs
[codecov-badge]: https://codecov.io/gh/carlobortolan/quantrs/graph/badge.svg?token=NJ4HW3OQFY
[codecov-url]: https://codecov.io/gh/carlobortolan/quantrs
[crates-msrv-badge]: https://img.shields.io/crates/msrv/quantrs

Quantrs is a tiny quantitative finance library for Rust. It is designed to be simple and easy to use, with a focus on performance and correctness. It is still in the early stages of development, so expect bugs and breaking changes.

Please check out the documentation [here][docs-url].

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
