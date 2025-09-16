# quantrs (python bindings)

![tests][actions-test-badge]
[![MIT/Apache 2.0 licensed][license-badge]](./LICENSE.md)
[![Crate][crates-badge]][crates-url]
[![docs.rs][docsrs-badge]][docs-url]
[![codecov-quantrs][codecov-badge]][codecov-url]
![Crates.io MSRV][crates-msrv-badge]
![Crates.io downloads][crates-download-badge]

[actions-test-badge]: https://github.com/carlobortolan/quantrs/actions/workflows/ci.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/quantrs.svg
[crates-url]: https://crates.io/crates/quantrs
[license-badge]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg
[docsrs-badge]: https://img.shields.io/docsrs/quantrs
[docs-url]: https://docs.rs/quantrs/*/quantrs
[codecov-badge]: https://codecov.io/gh/carlobortolan/quantrs/graph/badge.svg?token=NJ4HW3OQFY
[codecov-url]: https://codecov.io/gh/carlobortolan/quantrs
[crates-msrv-badge]: https://img.shields.io/crates/msrv/quantrs
[crates-download-badge]: https://img.shields.io/crates/d/quantrs

Quantrs is a tiny quantitative finance library for Rust. These are the Python bindings for it.
It is designed to be as intuitive and easy to use as possible so that you can work with derivatives without the need to write complex code or have a PhD in reading QuantLib documentation.
The library is still in the early stages of development and many features are not yet implemented.

Please check out the documentation for the original Rust crate [here][docs-url] and at [github.com/carlobortolan/quantrs](https://github.com/carlobortolan/quantrs).

Currently, the bindings only cover the FI module.

## Installation

```bash
pip install quantrs
```

## Usage

### Day Count Conventions

```python
import quantrs

# Create day count convention
day_count = quantrs.DayCount("ACT/365F")

# Calculate year fraction between dates
year_frac = day_count.year_fraction("2025-01-01", "2025-07-01")
print(f"Year fraction: {year_frac}")

# Calculate day count
days = day_count.day_count("2025-01-01", "2025-07-01")
print(f"Day count: {days}")

# Convenience function
year_frac = quantrs.calculate_year_fraction("2025-01-01", "2025-07-01", "ACT/365F")
```

### Bond Pricing

```python
import quantrs

# Create zero-coupon bond
bond = quantrs.ZeroCouponBond(face_value=1000.0, maturity="2030-12-31")

# Calculate bond price
price = bond.price(
    settlement="2025-06-19",
    ytm=0.04,  # 4% yield to maturity
    day_count=quantrs.DayCount("ACT/365F")
)
print(f"Bond price: ${price:.2f}")
```

## Benchmarks

Compared to other popular and well-maintained (i.e., actively developed, well-documented, and feature-rich) options pricing libraries, quantrs (Rust) competes well in terms of performance:
E.g., for pricing a European call with the Merton Black-Scholes model, quantrs is:

- **87x faster** than `py_vollib`
- **29x faster** than `QuantLib` (python bindings)
- **15x faster** than `RustQuant`
- **3x faster** than `Q-Fin`
- **1.7x slower** than `QuantLib` (cpp)

| Library                                                | Mean Execution Time (μs) | Median Execution Time (μs) | Standard Deviation (μs) | Operations / Second (OPS) |
| ------------------------------------------------------ | ------------------------ | -------------------------- | ----------------------- | ------------------------- |
| quantrs                                                | 0.0971                   | 0.0970                     | 0.0007                  | 10,142,000                |
| quantrs (py)                                           | TODO                     | TODO                       | TODO                    | TODO                      |
| [py_vollib](https://github.com/vollib/py_vollib)       | 8.5341                   | 8.5210                     | 0.8129                  | 117,176                   |
| [QuantLib](https://pypi.org/project/QuantLib) (py)     | 2.8551                   | 2.8630                     | 0.9391                  | 350,250                   |
| [RustQuant](https://github.com/avhz/RustQuant)         | 1.4777                   | 1.4750                     | 0.0237                  | 676,727                   |
| [Q-Fin](https://github.com/romanmichaelpaolucci/Q-Fin) | 0.2900                   | 0.2906                     | 0.0448                  | 3,447,870                 |
| [QuantLib](https://www.quantlib.org) (cpp)             | 0.0556                   | n.a.                       | n.a.                    | 17,958,600                |

You can find the benchmarks at [quantrs.pages.dev/report](https://quantrs.pages.dev/report/).

_Published benchmarks have been measured on a selfhosted VM with 32 GB RAM, AMD Ryzen 7 PRO 6850U @ 2.70GHz, and Manjaro Linux x86_64_

## Contributing

If you find any bugs or have suggestions for improvement, please open a new issue or PR. See [OUTLOOK.md](OUTLOOK.md) for a list of planned features and improvements.

You can generate and test the Python bindings with:

```sh
python -m venv .venv
source .venv/bin/activate
pip install maturin
# Build and install your package in editable mode
maturin develop --features python
# Run tests
pytest bindings/python/tests
```

## Disclaimer

This library is not intended for professional use. It is a hobby project and should be treated as such.

The python bindings are automatically generated using [PyO3](https://pyo3.rs) and may not cover all features of the Rust library.

## License

This project is licensed under either of:

- [MIT license](LICENSE-MIT.md) or
- [Apache License, Version 2.0](LICENSE-APACHE.md)

at your option.

---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](mailto:carlobortolan@gmail.com)
