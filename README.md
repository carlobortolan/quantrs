# quantrs

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

Quantrs is a tiny quantitative finance library for Rust.
It is designed to be as intuitive and easy to use as possible so that you can work with derivatives without the need to write complex code or have a PhD in reading QuantLib documentation.
The library is still in the early stages of development and many features are not yet implemented.

Please check out the documentation [here][docs-url].

## Features

### Options Pricing

Quantrs supports options pricing with various models for both vanilla and exotic options as well as options trading strategies for both basic options spreads and non-directional strategies.

<details>
<summary><i>Click to see supported models</i></summary>

|                             | Black-Scholes   | Black-76 | Lattice      | ³Monte-Carlo | Finite Diff   | Heston |
| --------------------------- | --------------- | -------- | ------------ | ------------ | ------------- | ------ |
| European                    | ✅              | ✅       | ✅           | ✅           | ⏳            | ⏳     |
| American                    | ❌              | ❌       | ✅           | ❌ (L. Sq.)  | ⏳            | ❌     |
| Bermudan                    | ❌              | ❌       | ✅           | ❌ (L. Sq.)  | ❌ (complex)  | ❌     |
| ¹Basket                     | ⏳ (∀component) | ❌       | ⏳ (approx.) | ⏳           | ❌            | ❌     |
| ¹Rainbow                    | ✅ (∀component) | ❌       | ✅           | ✅           | ❌            | ❌     |
| ²Barrier                    | ❌ (mod. BSM)   | ❌       | ⏳           | ⏳           | ⏳            | ⏳     |
| ²Double Barrier             | ❌ (mod. BSM)   | ❌       | ⏳           | ⏳           | ❌ (complex)  | ⏳     |
| ²Asian (fixed strike)       | ❌ (mod. BSM)   | ❌       | ❌           | ✅           | ⏳            | ⏳     |
| ²Asian (floating strike)    | ❌ (mod. BSM)   | ❌       | ❌           | ✅           | ⏳            | ⏳     |
| ²Lookback (fixed strike)    | ❌              | ❌       | ❌           | ✅           | ⏳            | ⏳     |
| ²Lookback (floating strike) | ✅              | ❌       | ❌           | ✅           | ⏳            | ⏳     |
| ²Binary Cash-or-Nothing     | ✅              | ❌       | ✅           | ✅           | ❌ (mod. PDE) | ⏳     |
| ²Binary Asset-or-Nothing    | ✅              | ❌       | ✅           | ✅           | ❌ (mod. PDE) | ⏳     |
| Greeks (Δ,ν,Θ,ρ,Γ)          | ✅              | ✅       | ⏳           | ❌           | ❌            | ❌     |
| Implied Volatility          | ✅              | ⏳       | ⏳           | ❌           | ❌            | ❌     |

> ¹ _"Exotic" options with standard exercise style; only differ in their payoff value_\
> ² _Non-vanilla path-dependent "exotic" options_\
> ³ _MC simulates underlying price paths based on geometric Brownian motion for Black-Scholes models and both arithmetic or geometric average price paths for Asian and Lookback options_\
> ✅ = Supported, ⏳ = Planned / In progress, ❌ = Not supported / Not applicable

<!--Bachelier and Modified Bachelier-->

</details>

<details>
<summary><i>Click to see supported strategies</i></summary>

| Strategy Name            | Type         | Description                                                                                       |
| ------------------------ | ------------ | ------------------------------------------------------------------------------------------------- |
| Covered Call             | Income       | Long stock + short call                                                                           |
| Protective Put           | Hedging      | Long stock + long put                                                                             |
| Guts                     | Volatility   | Long ITM call + long ITM put                                                                      |
| Straddle                 | Volatility   | Long ATM call + long ATM put                                                                      |
| Strangle                 | Volatility   | Long OTM call + long OTM put                                                                      |
| Butterfly Spread         | ¹Spread      | Long ITM call, short two ATM calls, long OTM call (or all puts)                                   |
| Iron Butterfly           | ¹Spread      | Short straddle + long wings                                                                       |
| Christmas Tree Butterfly | ¹Spread      | Long 1 ATM call, short 3 OTM calls, long 2 high-strike OTM calls (or all puts)                    |
| Condor Spread            | ¹Spread      | Long low-strike ITM call, short ITM call, short OTM call, long high-strike OTM call (or all puts) |
| Iron Condor              | ¹Spread      | Short strangle + long wings                                                                       |
| Calendar Spread          | ²Time Spread | Long far-expiry ATM call + short near-expiry ATM call (or all puts)                               |
| Diagonal Spread          | ³Time Spread | Short near-expiry OTM call + long far-expiry further OTM call (or all puts)                       |
| Back Spread              | Directional  | Long 2 OTM calls + short 1 ATM call (or all puts)                                                 |

> ¹ _Also referred to as 'vertical'_\
> ² _Also referred to as 'horizontal'_\
> ³ _Also referred to as 'diagonal'_

</details>

### Fixed Income

- Bond Types
  - [x] _Zero-Coupon Bonds_
  - [ ] _Treasury Bonds_ (fixed-rate coupon)
  - [ ] _Corporate Bonds_ (fixed-rate coupon with credit spreads)
  - [ ] _Floating-Rate Bonds_ (variable coupon with caps/floors)
- [ ] Duration (_Macaulay_, _Modified_, _Effective_)
- [ ] Convexity
- [ ] Yield Measures (_YTM_, _YTC_, _YTW_)
- [x] Day Count Conventions (_ACT/365F_, _ACT/365_, _ACT/360_, _30/360 US_, _30/360 Eurobond_, _ACT/ACT ISDA_, _ACT/ACT ICMA_)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
quantrs = "0.1.7"
```

Now if you want to e.g., calculate the arbitrage-free price of a binary cash-or-nothing call using the Black-Scholes model, you can:

```rust
use quantrs::options::*;

// Create a new instrument with a spot price of 100 and a dividend yield of 2%
let instrument = Instrument::new().with_spot(100.0).with_cont_yield(0.02);

// Create a new Cash-or-Nothing binary call option with:
// - Strike price (K) = 85
// - Time to maturity (T) = 0.78 years
let option = BinaryOption::cash_or_nothing(instrument, 85.0, 0.78, Call);

// Create a new Black-Scholes model with:
// - Risk-free interest rate (r) = 5%
// - Volatility (σ) = 20%
let model = BlackScholesModel::new(0.05, 0.2);

// Calculate the price of the binary call option using the Black-Scholes model
println!("Price: {}", model.price(&option));

// Calculate first order greeks for the option
println!("{:?}", Greeks::calculate(&model, &option));
```

This will output:

```text
Price: 0.8006934914644723
Greeks { delta: 0.013645840354947947, gamma: -0.0008813766475726433, theta: 0.17537248302290848, vega: -1.3749475702133236, rho: 0.4398346243436515 }
```

### Plotting

Quantrs also supports plotting option prices and strategies using the `plotters` backend.

E.g., Plot the P/L of a slightly skewed Condor spread consisting of fixed-strike Asian calls using the Monte-Carlo model with the geometric average price path:

<details>
<summary><i>Click to see example code</i></summary>

```rust
use quantrs::options::*;

// Create a new instrument with a spot price of 100 and a dividend yield of 2%
let instrument = Instrument::new().with_spot(100.0).with_cont_yield(0.02);

// Create a vector of fixed-strike Asian calls options with different strike prices
let options = vec![
    AsianOption::fixed(instrument.clone(), 85.0, 1.0, Call),
    AsianOption::fixed(instrument.clone(), 95.0, 1.0, Call),
    AsianOption::fixed(instrument.clone(), 102.0, 1.0, Call),
    AsianOption::fixed(instrument.clone(), 115.0, 1.0, Call),
];

// Create a new Monte-Carlo model with:
// - Risk-free interest rate (r) = 5%
// - Volatility (σ) = 20%
// - Number of simulations = 10,000
// - Number of time steps = 252
let model = MonteCarloModel::geometric(0.05, 0.2, 10_000, 252);

// Plot a breakdown of the Condor spread with a spot price range of [80,120]
model.plot_strategy_breakdown(
    "Condor Example",
    model.condor(&options[0], &options[1], &options[2], &options[3]),
    80.0..120.0,
    "examples/images/strategy.png",
    &options,
);
```

</details>

![condor_strategy](./examples/images/strategy.png)

<!--<div align="center">
  <img src="https://github.com/user-attachments/assets/0298807f-43ed-4458-9c7d-43b0f70defea" alt="condor_strategy" width="600"/>
</div>-->

See the [documentation][docs-url] for more information and examples.

## Benchmarks

Compared to other popular and well-maintained (i.e., actively developed, well-documented, and feature-rich) options pricing libraries, quantrs competes well in terms of performance:
E.g., for pricing a European call with the Merton Black-Scholes model, quantrs is:

- **87x faster** than `py_vollib`
- **29x faster** than `QuantLib` (python bindings)
- **15x faster** than `RustQuant`
- **3x faster** than `Q-Fin`
- **1.7x slower** than `QuantLib` (cpp)

| Library                                                | Mean Execution Time (μs) | Median Execution Time (μs) | Standard Deviation (μs) | Operations / Second (OPS) |
| ------------------------------------------------------ | ------------------------ | -------------------------- | ----------------------- | ------------------------- |
| quantrs                                                | 0.0971                   | 0.0970                     | 0.0007                  | 10,142,000                |
| [py_vollib](https://github.com/vollib/py_vollib)       | 8.5341                   | 8.5210                     | 0.8129                  | 117,176                   |
| [QuantLib](https://pypi.org/project/QuantLib) (py)     | 2.8551                   | 2.8630                     | 0.9391                  | 350,250                   |
| [RustQuant](https://github.com/avhz/RustQuant)         | 1.4777                   | 1.4750                     | 0.0237                  | 676,727                   |
| [Q-Fin](https://github.com/romanmichaelpaolucci/Q-Fin) | 0.2900                   | 0.2906                     | 0.0448                  | 3,447,870                 |
| [QuantLib](https://www.quantlib.org) (cpp)             | 0.0556                   | n.a.                       | n.a.                    | 17,958,600                |

You can find the benchmarks at [quantrs.pages.dev/report](https://quantrs.pages.dev/report/).

_Published benchmarks have been measured on a selfhosted VM with 32 GB RAM, AMD Ryzen 7 PRO 6850U @ 2.70GHz, and Manjaro Linux x86_64_

## Minimum supported Rust version (MSRV)

This crate requires a Rust version of 1.82.0 or higher. Increases in MSRV will be considered a semver non-breaking API change and require a version increase (PATCH until 1.0.0, MINOR after 1.0.0).

## Contributing

If you find any bugs or have suggestions for improvement, please open a new issue or PR. See [OUTLOOK.md](OUTLOOK.md) for a list of planned features and improvements.

## Disclaimer

This library is not intended for professional use. It is a hobby project and should be treated as such.

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
