use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quantrs::options::*;

fn black_scholes_benchmark(c: &mut Criterion) {
    // Setup
    let instrument = Instrument::new()
        .with_spot(100.0)
        .with_continuous_dividend_yield(0.02);
    let model = BlackScholesModel::new(0.78, 0.05, 0.2);

    // Benchmark European Option
    let option = EuropeanOption::new(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("black_scholes_price_european_call", |b| {
        b.iter(|| model.price(black_box(&option)))
    });
    c.bench_function("black_scholes_price_european_put", |b| {
        b.iter(|| model.price(black_box(&option.flip())))
    });

    // Benchmark Cash or Nothing Binary Option
    let option = BinaryOption::cash_or_nothing(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("black_scholes_price_cash_or_nothing_call", |b| {
        b.iter(|| model.price(black_box(&option)))
    });
    c.bench_function("black_scholes_price_cash_or_nothing_put", |b| {
        b.iter(|| model.price(black_box(&option.flip())))
    });

    // Benchmark Asset or Nothing Binary Option
    let option = BinaryOption::asset_or_nothing(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("black_scholes_price_asset_or_nothing_call", |b| {
        b.iter(|| model.price(black_box(&option)))
    });
    c.bench_function("black_scholes_price_asset_or_nothing_put", |b| {
        b.iter(|| model.price(black_box(&option.flip())))
    });
}

fn binomial_tree_benchmark(c: &mut Criterion) {
    // Setup
    let instrument = Instrument::new()
        .with_spot(100.0)
        .with_continuous_dividend_yield(0.02);
    let model = BinomialTreeModel::new(0.78, 0.05, 0.2, 100);

    // Benchmark European Option
    let option = EuropeanOption::new(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("binomial_tree_price_european_call", |b| {
        b.iter(|| model.price(black_box(&option)))
    });
    c.bench_function("binomial_tree_price_european_put", |b| {
        b.iter(|| model.price(black_box(&option.flip())))
    });

    // Benchmark American Option
    let option = AmericanOption::new(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("binomial_tree_price_american_call", |b| {
        b.iter(|| model.price(black_box(&option)))
    });
    c.bench_function("binomial_tree_price_american_put", |b| {
        b.iter(|| model.price(black_box(&option.flip())))
    });
}

fn monte_carlo_benchmark(c: &mut Criterion) {
    // Setup
    let instrument = Instrument::new()
        .with_spot(100.0)
        .with_continuous_dividend_yield(0.02);
    let geom_model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);
    let arith_model = MonteCarloModel::arithmetic(1.0, 0.05, 0.2, 1_000, 20);

    // Benchmark European Option (geometric average)
    let option = EuropeanOption::new(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("monte_carlo_geom_price_european_call", |b| {
        b.iter(|| geom_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_geom_price_european_put", |b| {
        b.iter(|| geom_model.price(black_box(&option.flip())))
    });

    // Benchmark European Option (arithmetic average)
    let option = EuropeanOption::new(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("monte_carlo_arith_price_european_call", |b| {
        b.iter(|| arith_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_arith_price_european_put", |b| {
        b.iter(|| arith_model.price(black_box(&option.flip())))
    });

    // Benchmark Cash Or Nothing Binary Option (geometric model)
    let option = BinaryOption::cash_or_nothing(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("monte_carlo_geom_price_cash_or_nothing_call", |b| {
        b.iter(|| geom_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_geom_price_cash_or_nothing_put", |b| {
        b.iter(|| geom_model.price(black_box(&option.flip())))
    });

    // Benchmark Cash Or Nothing Binary Option (arithmetic model)
    let option = BinaryOption::cash_or_nothing(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("monte_carlo_arith_price_cash_or_nothing_call", |b| {
        b.iter(|| arith_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_arith_price_cash_or_nothing_put", |b| {
        b.iter(|| arith_model.price(black_box(&option.flip())))
    });

    // Benchmark Asset Or Nothing Binary Option (geometric model)
    let option = BinaryOption::asset_or_nothing(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("monte_carlo_geom_price_asset_or_nothing_call", |b| {
        b.iter(|| geom_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_geom_price_asset_or_nothing_put", |b| {
        b.iter(|| geom_model.price(black_box(&option.flip())))
    });

    // Benchmark Asset Or Nothing Binary Option (arithmetic model)
    let option = BinaryOption::asset_or_nothing(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("monte_carlo_arith_price_asset_or_nothing_call", |b| {
        b.iter(|| arith_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_arith_price_asset_or_nothing_put", |b| {
        b.iter(|| arith_model.price(black_box(&option.flip())))
    });

    // Benchmark Asian Fixed Strike Option (geometric model)
    let option = AsianOption::fixed(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("monte_carlo_geom_price_asian_fixed_strike_call", |b| {
        b.iter(|| geom_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_geom_price_asian_fixed_strike_put", |b| {
        b.iter(|| geom_model.price(black_box(&option.flip())))
    });

    // Benchmark Asian Fixed Strike Option (arithmetic model)
    let option = AsianOption::fixed(instrument.clone(), 85.0, OptionType::Call);
    c.bench_function("monte_carlo_arith_price_asian_fixed_strike_call", |b| {
        b.iter(|| arith_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_arith_price_asian_fixed_strike_put", |b| {
        b.iter(|| arith_model.price(black_box(&option.flip())))
    });

    // Benchmark Asian Floating Strike Option (geometric model)
    let option = AsianOption::floating(instrument.clone(), OptionType::Call);
    c.bench_function("monte_carlo_geom_price_asian_floating_strike_call", |b| {
        b.iter(|| geom_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_geom_price_asian_floating_strike_put", |b| {
        b.iter(|| geom_model.price(black_box(&option.flip())))
    });

    // Benchmark Asian Floating Strike Option (arithmetic model)
    let option = AsianOption::floating(instrument.clone(), OptionType::Call);
    c.bench_function("monte_carlo_arith_price_asian_floating_strike_call", |b| {
        b.iter(|| arith_model.price(black_box(&option)))
    });
    c.bench_function("monte_carlo_arith_price_asian_floating_strike_put", |b| {
        b.iter(|| arith_model.price(black_box(&option.flip())))
    });
}

criterion_group!(
    black_scholes_benches,
    black_scholes_benchmark,
    binomial_tree_benchmark,
    monte_carlo_benchmark
);

criterion_main!(black_scholes_benches);
