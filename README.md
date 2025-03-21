# quantrs

![tests][actions-test-badge]
[![MIT licensed][license-badge]](./LICENSE.md)
[![Crate][crates-badge]][crates-url]
[![docs.rs][docsrs-badge]][docs-url]
[![codecov-quantrs][codecov-badge]][codecov-url]
![Crates.io MSRV][crates-msrv-badge]
![Crates.io downloads][crates-download-badge]

[actions-test-badge]: https://github.com/carlobortolan/quantrs/actions/workflows/ci.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/quantrs.svg
[crates-url]: https://crates.io/crates/quantrs
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[docsrs-badge]: https://img.shields.io/docsrs/quantrs
[docs-url]: https://docs.rs/quantrs/*/quantrs
[codecov-badge]: https://codecov.io/gh/carlobortolan/quantrs/graph/badge.svg?token=NJ4HW3OQFY
[codecov-url]: https://codecov.io/gh/carlobortolan/quantrs
[crates-msrv-badge]: https://img.shields.io/crates/msrv/quantrs
[crates-download-badge]: https://img.shields.io/crates/d/quantrs

Quantrs is a tiny quantitative finance library for Rust.
It is designed to be as intuitive and easy to use as possible so that you can work with derivatives without the need to write complex code or have a PhD in reading QuantLib documentation.
The library is still in the early stages of development, and many features are not yet implemented.

Please check out the documentation [here][docs-url].

## Features

### Options Pricing

Quantrs supports options pricing with various models for both vanilla and exotic options as well as options trading strategies for both basic options spreads and non-directional strategies.

<details>
<summary><i>Click to see supported models</i></summary>

|                             | Black-Scholes   | Black-76 | Lattice      | ³Monte-Carlo | Finite Diff   | Heston |
| --------------------------- | --------------- | -------- | ------------ | ------------ | ------------- | ------ |
| European                    | ✅              | ⏳       | ✅           | ✅           | ⏳            | ⏳     |
| American                    | ❌              | ❌       | ✅           | ❌ (L. Sq.)  | ⏳            | ❌     |
| Bermudan                    | ❌              | ❌       | ⏳           | ❌ (L. Sq.)  | ❌ (complex)  | ❌     |
| ¹Basket                     | ⏳ (∀component) | ❌       | ⏳ (approx.) | ⏳           | ❌            | ❌     |
| ¹Rainbow                    | ✅ (∀component) | ❌       | ✅           | ✅           | ❌            | ❌     |
| ²Barrier                    | ❌ (mod. BSM)   | ❌       | ⏳           | ⏳           | ⏳            | ⏳     |
| ²Double Barrier             | ❌ (mod. BSM)   | ❌       | ⏳           | ⏳           | ❌ (complex)  | ⏳     |
| ²Asian (fixed strike)       | ❌ (mod. BSM)   | ❌       | ❌           | ✅           | ⏳            | ⏳     |
| ²Asian (floating strike)    | ❌ (mod. BSM)   | ❌       | ❌           | ✅           | ⏳            | ⏳     |
| ²Lookback (fixed strike)    | ⏳              | ❌       | ❌           | ⏳           | ⏳            | ⏳     |
| ²Lookback (floating strike) | ⏳              | ❌       | ❌           | ⏳           | ⏳            | ⏳     |
| ²Binary Cash-or-Nothing     | ✅              | ⏳       | ✅           | ✅           | ❌ (mod. PDE) | ⏳     |
| ²Binary Asset-or-Nothing    | ✅              | ⏳       | ✅           | ✅           | ❌ (mod. PDE) | ⏳     |
| Greeks (Δ,ν,Θ,ρ,Γ)          | ✅              | ⏳       | ⏳           | ❌           | ❌            | ❌     |
| Implied Volatility          | ✅              | ⏳       | ⏳           | ❌           | ❌            | ❌     |

> ¹ _"Exotic" options with standard exercise style; only differ in their payoff value_\
> ² _Non-vanilla path-dependent "exotic" options_\
> ³ _MC simulates underlying price paths based on geometric Brownian motion for Black-Scholes models and geometric average price paths for Asian and Lookback options_\
> ✅ = Supported, ⏳ = Planned / In progress, ❌ = Not supported / Not applicable

</details>

<details>
<summary><i>Click to see supported strategies</i></summary>

| Strategy Name    | Type         | Description                                                                  |
| ---------------- | ------------ | ---------------------------------------------------------------------------- |
| Covered Call     | Income       | Long stock + short call                                                      |
| Protective Put   | Hedging      | Long stock + long put                                                        |
| Straddle         | Volatility   | Long call + long put (same strike)                                           |
| Strangle         | Volatility   | Long OTM call + long OTM put                                                 |
| Butterfly Spread | ¹Spread      | Long ATM call, short two middle calls, long OTM call                         |
| Iron Butterfly   | ¹Spread      | Short straddle + long wings                                                  |
| Condor Spread    | ¹Spread      | Long OTM call, short ITM call, short ITM put, long OTM put                   |
| Iron Condor      | ¹Spread      | Short strangle + long wings                                                  |
| Calendar Spread  | ²Time Spread | Long far-expiry call + short near-expiry call                                |
| Diagonal Spread  | ³Time Spread | Long far-expiry call (higher strike) + short near-expiry call (lower strike) |
| Back Spread      | Directional  | Long 2 calls + short 1 ITM call                                              |
| Christmas Tree   | ¹Complex     | Long 1 ITM call, short 3 middle calls, long 2 OTM calls                      |

> ¹ _Also referred to as 'vertical'_\
> ² _Also referred to as 'horizontal'_\
> ³ _Also referred to as 'diagonal'_\

</details>

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
quantrs = "0.1.5"
```

Now if you want to e.g., model binary call options using the Black-Scholes model, you can:

```rust
use quantrs::options::*;

// Create a new instrument with a spot price of 100 and a dividend yield of 2%
let instrument = Instrument::new()
    .with_spot(100.0)
    .with_continuous_dividend_yield(0.02);

// Create a new Cash-or-Nothing binary call option with:
// - Strike price (K) = 85
// - Time to maturity (T) = 0.78 years
let option = BinaryOption::cash_or_nothing(instrument, 85.0, 0.78, OptionType::Call);

// Create a new Black-Scholes model with:
// - Risk-free interest rate (r) = 5%
// - Volatility (σ) = 20%
let model = BlackScholesModel::new(0.05, 0.2);

// Calculate the price of the binary call option using the Black-Scholes model
println!("Price: {}", model.price(&option));

// Calculate the Greeks (Delta, Gamma, Theta, Vega, Rho) for the option
println!("Greeks: {:?}", Greeks::calculate(&model, &option));
```

This will output:

```text
Price: 0.8006934914644723
Greeks: Greeks { delta: 0.013645840354947947, gamma: -0.0008813766475726433, theta: 0.17537248302290848, vega: -1.3749475702133236, rho: 0.4398346243436515 }
```

### Plotting

Quantrs also supports plotting option prices and strategies using the `plotters` backend:

<details>
<summary><i>Click to see example code</i></summary>

```rust
// Create a new instrument with a spot price of 100 and a dividend yield of 2%
let instrument = Instrument::new()
    .with_spot(100.0)
    .with_continuous_dividend_yield(0.02);

// Create a vector of European call options with different strike prices
let options = vec![
    EuropeanOption::new(instrument.clone(), 30.0, 1.0, OptionType::Call),
    EuropeanOption::new(instrument.clone(), 40.0, 1.0, OptionType::Call),
    EuropeanOption::new(instrument.clone(), 60.0, 1.0, OptionType::Call),
    EuropeanOption::new(instrument.clone(), 70.0, 1.0, OptionType::Call),
];

// Create a new Black-Scholes model with:
// - Risk-free interest rate (r) = 5%
// - Volatility (σ) = 20%
let model = BlackScholesModel::new(0.05, 0.2);

model.plot_strategy_breakdown(
    "Condor Example",
    model.condor(&options[0], &options[1], &options[2], &options[3]),
    20.0..80.0,
    "path/to/destination.png",
    &options,
);
```

</details>

<div align="center">
  <img src="https://github.com/user-attachments/assets/0298807f-43ed-4458-9c7d-43b0f70defea" alt="condor_strategy" width="600"/>
</div>

See the [documentation][docs-url] for more information and examples.

## Benchmarks

Compared to other popular options pricing libraries, quantrs is _significantly_ faster:

<!-- - **⏳x faster** `QuantLib` (C++ bindings) -->

- **29x faster** than `QuantLib` (python bindings)
- **113x faster** than `py_vollib`
- **15x faster** than `RustQuant`
- **2.7x faster** than `Q-Fin`

| Library                                                | Mean Execution Time (μs) | Median Execution Time (μs) | Standard Deviation (μs) | Operations / Second (OPS) |
| ------------------------------------------------------ | ------------------------ | -------------------------- | ----------------------- | ------------------------- |
| quantrs                                                | 0.0971                   | 0.0970                     | 0.0007                  | 10,142,000                |
| [QuantLib](https://www.quantlib.org) (cpp)             | n.a.                     | n.a.                       | n.a.                    | n.a.                      |
| [QuantLib](https://pypi.org/project/QuantLib) (py)     | 2.8551                   | 2.8630                     | 0.9391                  | 350,250                   |
| [py_vollib](https://github.com/vollib/py_vollib)       | 10.9959                  | 10.8950                    | 1.1398                  | 90,943                    |
| [Q-Fin](https://github.com/romanmichaelpaolucci/Q-Fin) | 0.2622                   | 0.2603                     | 0.0356                  | 3,813,700                 |
| [RustQuant](https://github.com/avhz/RustQuant)         | 1.4777                   | 1.4750                     | 0.0237                  | 676,727                   |

You can find the benchmarks at [quantrs.pages.dev/report](https://quantrs.pages.dev/report/)

_Published benchmarks have been measured on a selfhosted VM with 32 GB RAM, AMD Ryzen 7 PRO 6850U @ 2.70GHz, and Manjaro Linux x86_64_

## Minimum supported Rust version (MSRV)

This crate requires a Rust version of 1.77.0 or higher. Increases in MSRV will be considered a semver non-breaking API change and require a version increase (PATCH until 1.0.0, MINOR after 1.0.0).

## Outlook

See [OUTLOOK.md](OUTLOOK.md) for a list of planned features and improvements.

## Contributing

If you find any bugs or have suggestions for improvement, please open a new issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.

---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](mailto:carlobortolan@gmail.com)

```

```
