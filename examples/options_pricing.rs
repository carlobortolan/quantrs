// Run:  cargo run --release --example options_pricing

use quantrs::options::{
    types::RainbowOption, AsianOption, BinaryOption, BinomialTreeModel, BlackScholesModel,
    EuropeanOption, Greeks, Instrument, MonteCarloModel, Option, OptionGreeks, OptionPricing,
    OptionType,
};

fn main() {
    example_from_readme();
    example_black_scholes();
    example_binomial_tree();
    example_greeks();
    example_monte_carlo();
    rainbow_option_example();
    example_asian();
}

fn example_black_scholes() {
    let instrument = Instrument::new().with_spot(100.0);
    let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
    let model = BlackScholesModel::new(1.0, 0.05, 0.2);

    let call_price = model.price(&option);
    println!("Black-Scholes Call Price: {}", call_price);

    let put_price = model.price(&option.flip());
    println!("Black-Scholes Put Price: {}", put_price);

    let market_price = 10.0; // Example market price
    let implied_volatility = model.implied_volatility(&option, market_price);
    println!("Implied Volatility: {}\n", implied_volatility);
}

fn example_binomial_tree() {
    let instrument = Instrument::new().with_spot(100.0);
    let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
    let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

    let call_price = model.price(&option);
    println!("Binomial Tree Call Price: {}", call_price);

    let put_price = model.price(&option.flip());
    println!("Binomial Tree Put Price: {}", put_price);

    let market_price = 10.0; // Example market price
    let implied_volatility = model.implied_volatility(&option, market_price);
    println!("Implied Volatility: {}\n", implied_volatility);
}

fn example_greeks() {
    let instrument = Instrument::new().with_spot(100.0);
    let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
    let model = BlackScholesModel::new(1.0, 0.05, 0.2);

    let greeks = Greeks::calculate(&model, option.clone());

    println!("Delta: {}", greeks.delta);
    println!("Gamma: {}", greeks.gamma);
    println!("Theta: {}", greeks.theta);
    println!("Vega: {}", greeks.vega);
    println!("Rho: {}", greeks.rho);

    // Greeks via function calls
    println!("Delta: {}", model.delta(&option));
    println!("Gamma: {}", model.gamma(&option));
    println!("Theta: {}", model.theta(&option));
    println!("Vega: {}", model.vega(&option));
    println!("Rho: {}\n", model.rho(&option));
}

fn example_asian() {
    let instrument = Instrument::new()
        .with_spot(110.0)
        .with_continuous_dividend_yield(0.0);
    let option = AsianOption::floating(instrument.clone(), OptionType::Call);
    let arithmetic_model = MonteCarloModel::arithmetic(1.0, 0.03, 0.2, 4_000, 20);
    let geometric_model = MonteCarloModel::geometric(1.0, 0.03, 0.2, 4_000, 20);

    let price = arithmetic_model.price(&option);
    println!("Arithmetic Call Price: {}", price);

    let price = arithmetic_model.price(&option.flip());
    println!("Arithmetic Put Price: {}", price);

    let price = geometric_model.price(&option);
    println!("Geometric Call Price: {}", price);

    let price = geometric_model.price(&option.flip());
    println!("Geometric Put Price: {}", price);
}

fn example_monte_carlo() {
    let instrument = Instrument::new().with_spot(100.0);
    let model = MonteCarloModel::arithmetic(1.0, 0.01, 0.3, 1_000, 52);

    let european_option = EuropeanOption::new(instrument.clone(), 100.0, OptionType::Call);
    println!(
        "[Monte Carlo] European Call: {}",
        model.price(&european_option)
    );
    println!(
        "[Monte Carlo] European Put: {}",
        model.price(&european_option.flip())
    );

    let binary_option = BinaryOption::cash_or_nothing(instrument.clone(), 100.0, OptionType::Call);
    println!("[Monte Carlo] Binary Call: {}", model.price(&binary_option));
    println!(
        "[Monte Carlo] Binary Put: {}",
        model.price(&binary_option.flip())
    );

    // let barrier_option = BarrierOption::up(instrument.clone(), 100.0, OptionType::Call);
    // println!("[Monte Carlo] Barrier Call: {}", model.price(&barrier_option));
    // println!("[Monte Carlo] Barrier Put: {}", model.price(&barrier_option.flip()));
    // => 4.895841997908933
    // => 12.15233976468229

    let asian_option = AsianOption::fixed(instrument.clone(), 100.0, OptionType::Call);
    println!("[Monte Carlo] Asian Call: {}", model.price(&asian_option));
    println!(
        "[Monte Carlo] Asian Put: {}",
        model.price(&asian_option.flip())
    );
}

fn example_from_readme() {
    // Create a new instrument with a spot price of 100 and a dividend yield of 2%
    let instrument = Instrument::new()
        .with_spot(100.0)
        .with_continuous_dividend_yield(0.02);

    // Create a new Cash-or-Nothing binary call option with a strike price of 85
    let option = BinaryOption::cash_or_nothing(instrument, 85.0, OptionType::Call);

    // Create a new Black-Scholes model with:
    // - Time to maturity (T) = 0.78 years
    // - Risk-free interest rate (r) = 5%
    // - Volatility (σ) = 20%
    let model = BlackScholesModel::new(0.78, 0.05, 0.2);

    // Calculate the price of the binary call option using the Black-Scholes model
    let price = model.price(&option);
    println!("Price: {}", price);

    // Calculate the Greeks (Delta, Gamma, Theta, Vega, Rho) for the option
    let greeks = Greeks::calculate(&model, option);
    println!("Greeks: {:?}\n", greeks);
}

fn rainbow_option_example() {
    let q = 0.0;

    let asset1 = Instrument::new()
        .with_spot(115.0)
        .with_continuous_dividend_yield(q);
    let asset2 = Instrument::new()
        .with_spot(104.0)
        .with_continuous_dividend_yield(q);
    let asset3 = Instrument::new()
        .with_spot(86.0)
        .with_continuous_dividend_yield(q);

    // Pays 50% of the best return (at maturity), 30% of the second best and 20% of the third best
    let _weights = [0.5, 0.3, 0.2];

    let instrument = Instrument::new()
        .with_assets(vec![(asset1.clone()), (asset2.clone()), (asset3.clone())])
        .with_continuous_dividend_yield(q);

    let best_of = RainbowOption::best_of(instrument.clone(), 105.0);
    let worst_of = RainbowOption::worst_of(instrument.clone(), 105.0);
    let call_on_avg = RainbowOption::call_on_avg(instrument.clone(), 100.0);
    let put_on_avg = RainbowOption::put_on_avg(instrument.clone(), 110.0);
    let all_itm = RainbowOption::all_itm(instrument.clone(), 105.0);
    let all_otm = RainbowOption::all_otm(instrument.clone(), 105.0);
    let call_on_max = RainbowOption::call_on_max(instrument.clone(), 105.0);
    let call_on_min = RainbowOption::call_on_min(instrument.clone(), 80.0);
    let put_on_max = RainbowOption::put_on_max(instrument.clone(), 120.0);
    let put_on_min = RainbowOption::put_on_min(instrument.clone(), 105.0);

    println!("Best-Of Payoff: {}", best_of.payoff(None)); // should be 115.0
    println!("Worst-Of Payoff: {}", worst_of.payoff(None)); // should be 86.0
    println!("Call-On-Avg Payoff: {}", call_on_avg.payoff(None)); // should be 1.6
    println!("Put-On-Avg Payoff: {}", put_on_avg.payoff(None)); // should be 8.3
    println!("All ITM Payoff: {}", all_itm.payoff(None)); // should be 0.0
    println!("All OTM Payoff: {}", all_otm.payoff(None)); // should be 0.0
    println!("Call-On-Max Payoff: {}", call_on_max.payoff(None)); // should be 10.0
    println!("Call-On-Min Payoff: {}", call_on_min.payoff(None)); // should be 6.0
    println!("Put-On-Max Payoff: {}", put_on_max.payoff(None)); // should be 5.0
    println!("Put-On-Min Payoff: {}", put_on_min.payoff(None)); // should be 19.0

    let eur_call_max = EuropeanOption::new(asset1.clone(), 105.0, OptionType::Call);
    let eur_call_min = EuropeanOption::new(asset3.clone(), 80.0, OptionType::Call);
    let eur_put_max = EuropeanOption::new(asset1.clone(), 120.0, OptionType::Put);
    let eur_put_min = EuropeanOption::new(asset3.clone(), 105.0, OptionType::Put);

    // let model = MonteCarloModel::arithmetic(1.0, 0.05, 0.2, 1_000, 252);
    let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);
    // let model = BlackScholesModel::new(1.0, 0.05, 0.2);

    println!(
        "Best-Of Price: {}, should be: {}",
        model.price(&best_of),
        118.03716196015654
    );
    println!(
        "Worst-Of Price: {}, should be: {}",
        model.price(&worst_of),
        83.58825958790867
    );
    println!(
        "Call-On-Avg Price: {}, should be: {}",
        model.price(&call_on_avg),
        11.55384829911662
    );
    println!(
        "Put-On-Avg Price: {}, should be: {}",
        model.price(&put_on_avg),
        9.76633986600689
    );
    println!(
        "Call-On-Max Price: {}, should be: {}",
        model.price(&call_on_max),
        model.price(&eur_call_max)
    );
    println!(
        "Call-On-Min Price: {}, should be: {}",
        model.price(&call_on_min),
        model.price(&eur_call_min)
    );
    println!(
        "Put-On-Max Price: {}, should be: {}",
        model.price(&put_on_max),
        model.price(&eur_put_max)
    );
    println!(
        "Put-On-Min Price: {}, should be: {}",
        model.price(&put_on_min),
        model.price(&eur_put_min)
    );

    // Call-On-Max Price: 18.149769825601027, should be: 18.149769825601027
    // Call-On-Min Price: 12.572331070072991, should be: 12.572331070072991
    // Put-On-Max Price: 8.706509687477691, should be: 8.706509687477691
    // Put-On-Min Price: 16.3115106502782, should be: 16.3115106502782
}
