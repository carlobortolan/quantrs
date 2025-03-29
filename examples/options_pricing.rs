// Run:  cargo run --release --example options_pricing

use quantrs::options::{
    AsianOption, BinaryOption, BinomialTreeModel, Black76Model, BlackScholesModel, EuropeanOption,
    Greeks, Instrument, MonteCarloModel, Option, OptionGreeks, OptionPricing, OptionStrategy,
    OptionType::*, RainbowOption,
};

fn main() {
    example_from_readme();
    example_black_scholes();
    example_binomial_tree();
    example_monte_carlo();
    example_black_76();
    example_greeks();
    example_asian();
    example_rainbow();
    example_strategy();
    example_plots();
}

fn example_from_readme() {
    // Create a new instrument with a spot price of 100 and a dividend yield of 2%
    let instrument = Instrument::new()
        .with_spot(100.0)
        .with_continuous_dividend_yield(0.02);

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

    // Calculate the Greeks (Delta, Gamma, Theta, Vega, Rho) for the option
    println!("Greeks: {:?}", Greeks::calculate(&model, &option));
}

fn example_black_scholes() {
    let instrument = Instrument::new().with_spot(100.0);
    let option = EuropeanOption::new(instrument, 100.0, 1.0, Call);
    let model = BlackScholesModel::new(0.05, 0.2);

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
    let option = EuropeanOption::new(instrument, 100.0, 1.0, Call);
    let model = BinomialTreeModel::new(0.05, 0.2, 100);

    let call_price = model.price(&option);
    println!("Binomial Tree Call Price: {}", call_price);

    let put_price = model.price(&option.flip());
    println!("Binomial Tree Put Price: {}", put_price);
    //
    //let market_price = 10.0; // Example market price
    //let implied_volatility = model.implied_volatility(&option, market_price);
    //println!("Implied Volatility: {}\n", implied_volatility);
}

fn example_monte_carlo() {
    let instrument = Instrument::new().with_spot(100.0);
    let model = MonteCarloModel::arithmetic(0.01, 0.3, 1_000, 52);

    let european_option = EuropeanOption::new(instrument.clone(), 100.0, 1.0, Call);
    println!(
        "[Monte Carlo] European Call: {}",
        model.price(&european_option)
    );
    println!(
        "[Monte Carlo] European Put: {}",
        model.price(&european_option.flip())
    );

    let binary_option = BinaryOption::cash_or_nothing(instrument.clone(), 100.0, 1.0, Call);
    println!("[Monte Carlo] Binary Call: {}", model.price(&binary_option));
    println!(
        "[Monte Carlo] Binary Put: {}",
        model.price(&binary_option.flip())
    );

    // let barrier_option = BarrierOption::up(instrument.clone(), 100.0, Call);
    // println!("[Monte Carlo] Barrier Call: {}", model.price(&barrier_option));
    // println!("[Monte Carlo] Barrier Put: {}", model.price(&barrier_option.flip()));
    // => 4.895841997908933
    // => 12.15233976468229

    let asian_option = AsianOption::fixed(instrument.clone(), 100.0, 1.0, Call);
    println!("[Monte Carlo] Asian Call: {}", model.price(&asian_option));
    println!(
        "[Monte Carlo] Asian Put: {}",
        model.price(&asian_option.flip())
    );
}

fn example_black_76() {
    let instrument = Instrument::new().with_spot(2006.0);
    let option = EuropeanOption::new(instrument, 2100.0, 0.08493, Call);
    let model = Black76Model::new(0.050067, 0.35);

    let call_price = model.price(&option);
    println!("Black-76 Call Price: {}", call_price);

    let put_price = model.price(&option.flip());
    println!("Black-76 Put Price: {}", put_price);

    let market_price = 10.0; // Example market price
    let implied_volatility = model.implied_volatility(&option, market_price);
    println!("Implied Volatility: {}\n", implied_volatility);
}

fn example_greeks() {
    let instrument = Instrument::new().with_spot(100.0);
    let option = EuropeanOption::new(instrument, 100.0, 1.0, Call);
    let model = BlackScholesModel::new(0.05, 0.2);

    let greeks = Greeks::calculate(&model, &option);

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
    let option = AsianOption::floating(instrument.clone(), 1.0, Call);
    let arithmetic_model = MonteCarloModel::arithmetic(0.03, 0.2, 4_000, 20);
    let geometric_model = MonteCarloModel::geometric(0.03, 0.2, 4_000, 20);

    let price = arithmetic_model.price(&option);
    println!("Arithmetic Call Price: {}", price);

    let price = arithmetic_model.price(&option.flip());
    println!("Arithmetic Put Price: {}", price);

    let price = geometric_model.price(&option);
    println!("Geometric Call Price: {}", price);

    let price = geometric_model.price(&option.flip());
    println!("Geometric Put Price: {}", price);
}

fn example_rainbow() {
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

    let best_of = RainbowOption::best_of(instrument.clone(), 105.0, 1.0);
    let worst_of = RainbowOption::worst_of(instrument.clone(), 105.0, 1.0);
    let call_on_avg = RainbowOption::call_on_avg(instrument.clone(), 100.0, 1.0);
    let put_on_avg = RainbowOption::put_on_avg(instrument.clone(), 110.0, 1.0);
    let all_itm = RainbowOption::all_itm(instrument.clone(), 105.0, 1.0);
    let all_otm = RainbowOption::all_otm(instrument.clone(), 105.0, 1.0);
    let call_on_max = RainbowOption::call_on_max(instrument.clone(), 105.0, 1.0);
    let call_on_min = RainbowOption::call_on_min(instrument.clone(), 80.0, 1.0);
    let put_on_max = RainbowOption::put_on_max(instrument.clone(), 120.0, 1.0);
    let put_on_min = RainbowOption::put_on_min(instrument.clone(), 105.0, 1.0);

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

    let eur_call_max = EuropeanOption::new(asset1.clone(), 105.0, 1.0, Call);
    let eur_call_min = EuropeanOption::new(asset3.clone(), 80.0, 1.0, Call);
    let eur_put_max = EuropeanOption::new(asset1.clone(), 120.0, 1.0, Put);
    let eur_put_min = EuropeanOption::new(asset3.clone(), 105.0, 1.0, Put);

    // let model = MonteCarloModel::arithmetic(1.0, 0.05, 0.2, 1_000, 252);
    let model = BinomialTreeModel::new(0.05, 0.2, 100);
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

fn example_strategy() {
    // let model = BlackScholesModel::new(0.0025, 0.15);
    // let model = MonteCarloModel::geometric(0.05, 0.2, 10_000, 365);
    let model = BinomialTreeModel::new(0.05, 0.2, 100);
    let instrument = Instrument::new().with_spot(50.0);

    ////////////////////
    /* STOCK & OPTION */

    let call = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call);
    println!(
        "[Covered Call: {:?}], given stock: {}, call: {}",
        model.covered_call(&instrument, &call)(50.0),
        instrument.spot(),
        model.price(&call)
    );

    let put = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put);
    println!(
        "[Protective Put: {:?}], given stock: {}, put: {}",
        model.protective_put(&instrument, &put)(50.0),
        instrument.spot(),
        model.price(&put)
    );

    let otm_put = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put);
    let otm_call = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call);
    println!(
        "[Collar: {:?}], given stock: {}, put: {}, call: {}",
        model.collar(&instrument, &otm_put, &otm_call)(50.0),
        instrument.spot(),
        model.price(&otm_put),
        model.price(&otm_call)
    );

    let atm_put = EuropeanOption::new(instrument.clone(), 50.0, 1.0, Put);
    let otm_call = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call);
    let otm_put = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put);
    println!(
        "[Fence: {:?}], given stock: {}, put: {}, put: {}, call: {}",
        model.fence(&instrument, &atm_put, &otm_put, &otm_call)(50.0),
        instrument.spot(),
        model.price(&atm_put),
        model.price(&otm_call),
        model.price(&otm_put)
    );

    // [Covered Call: 50.46060396445954], given stock: 50, call: 0.4606039644595379
    // [Protective Put: 50.19404262184266], given stock: 50, put: 0.19404262184266008

    ////////////
    /* SIMPLE */

    let itm_put = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Put);
    let itm_call = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Call);
    println!(
        "[Guts: {:?}], given put: {}, call: {}",
        model.guts(&itm_put, &itm_call)(50.0),
        model.price(&itm_put),
        model.price(&itm_call)
    );

    let atm_put = EuropeanOption::new(instrument.clone(), 50.0, 1.0, Put);
    let atm_call = EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call);
    println!(
        "[Straddle: {:?}], given put: {}, call: {}",
        model.straddle(&atm_put, &atm_call)(50.0),
        model.price(&atm_put),
        model.price(&atm_call)
    );

    let otm_put = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put);
    let otm_call = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call);
    println!(
        "[Strangle: {:?}], given put: {}, call: {}",
        model.strangle(&otm_put, &otm_call)(50.0),
        model.price(&otm_put),
        model.price(&otm_call)
    );

    let otm_put = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put);
    let otm_call = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call);
    println!(
        "[Risk Reversal: {:?}], given put: {}, call: {}",
        model.risk_reversal(&otm_put, &otm_call)(50.0),
        model.price(&otm_put),
        model.price(&otm_call)
    );

    ///////////////
    /* BUTTERFLY */

    let lower_wing = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Call);
    let body = EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call);
    let upper_wing = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call);
    println!(
        "[Butterfly: {:?}], given lower: {}, body: {}, upper: {}",
        model.butterfly(&lower_wing, &body, &upper_wing)(50.0),
        model.price(&lower_wing),
        model.price(&body),
        model.price(&upper_wing)
    );

    let otm_put = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put);
    let atm_put = EuropeanOption::new(instrument.clone(), 50.0, 1.0, Put);
    let atm_call = EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call);
    let otm_call = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call);
    println!(
        "[Iron Butterfly: {:?}], given otm_put: {}, atm_put: {}, atm_call: {}, otm_call: {}",
        model.iron_butterfly(&otm_put, &atm_put, &atm_call, &otm_call)(50.0),
        model.price(&otm_put),
        model.price(&atm_put),
        model.price(&atm_call),
        model.price(&otm_call)
    );

    let o1 = EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call);
    let o2 = EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call);
    let o3 = EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call);
    let o4 = EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call);
    let o5 = EuropeanOption::new(instrument.clone(), 80.0, 1.0, Call);
    let o6 = EuropeanOption::new(instrument.clone(), 80.0, 1.0, Call);
    println!(
        "[Christmas Tree Butterfly: {:?}], given o1: {}, o2: {}, o3: {}, o4: {}, o5: {}, o6: {}",
        model.christmas_tree_butterfly(&o1, &o2, &o3, &o4, &o5, &o6)(50.0),
        model.price(&o1),
        model.price(&o2),
        model.price(&o3),
        model.price(&o4),
        model.price(&o5),
        model.price(&o6)
    );

    ////////////
    /* SPREAD */

    let short = EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call);
    let long1 = EuropeanOption::new(instrument.clone(), 55.0, 1.0, Call);
    let long2 = EuropeanOption::new(instrument.clone(), 55.0, 1.0, Call);
    println!(
        "[Back Spread: {:?}], given short: {}, long1: {}, long2: {}",
        model.back_spread(&short, &long1, &long2)(50.0),
        model.price(&short),
        model.price(&long1),
        model.price(&long2),
    );

    let long = EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call);
    let short1 = EuropeanOption::new(instrument.clone(), 55.0, 1.0, Call);
    let short2 = EuropeanOption::new(instrument.clone(), 55.0, 1.0, Call);
    println!(
        "[Ladder: {:?}], given long: {}, short1: {}, short2: {}",
        model.ladder(&long, &short1, &short2)(50.0),
        model.price(&short),
        model.price(&long1),
        model.price(&long2),
    );

    let front_month = EuropeanOption::new(instrument.clone(), 50.0, 1.0 / 12.0, Call);
    let back_month = EuropeanOption::new(instrument.clone(), 50.0, 2.0 / 12.0, Call);
    println!(
        "[Calendar Spread: {:?}], given front: {}, back: {}",
        model.calendar_spread(&front_month, &back_month)(50.0),
        model.price(&front_month),
        model.price(&back_month)
    );

    let front_month = EuropeanOption::new(instrument.clone(), 60.0, 1.0 / 12.0, Call);
    let back_month_long = EuropeanOption::new(instrument.clone(), 75.0, 2.0 / 12.0, Call);
    let back_month_short = EuropeanOption::new(instrument.clone(), 60.0, 1.0 / 12.0, Call);
    println!(
        "[Diagonal Spread: {:?}], given front: {}, back short: {}, back long: {}",
        model.diagonal_spread(&front_month, &back_month_short, &back_month_long)(50.0),
        model.price(&front_month),
        model.price(&back_month_short),
        model.price(&back_month_long)
    );

    ////////////
    /* CONDOR */

    let itm_call_long = EuropeanOption::new(instrument.clone(), 30.0, 1.0, Call);
    let itm_call_short = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Call);
    let otm_call_short = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call);
    let otm_call_long = EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call);
    println!(
        "[Condor: {:?}], given itm_call_long: {}, itm_call_short: {}, otm_call_short: {}, otm_call_long: {}",
        model.condor(&itm_call_long, &itm_call_short, &otm_call_short, &otm_call_long)(50.0),
        model.price(&itm_call_long),
        model.price(&itm_call_short),
        model.price(&otm_call_short),
        model.price(&otm_call_long)
    );

    let otm_put_long = EuropeanOption::new(instrument.clone(), 30.0, 1.0, Put);
    let otm_put_short = EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put);
    let otm_call_short = EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call);
    let otm_call_long = EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call);
    println!(
        "[Iron Condor: {:?}], given itm_call_long: {}, itm_call_short: {}, otm_call_short: {}, otm_call_long: {}",
        model.iron_condor(&otm_put_long, &otm_put_short, &otm_call_short, &otm_call_long)(50.0),
        model.price(&otm_put_long),
        model.price(&otm_put_short),
        model.price(&otm_call_short),
        model.price(&otm_call_long)
    );

    // ==> OTM options are cheaper, ATM options have moderate values, and ITM options have higher premiums.

    ////////////
    /* PLOTS */
    let options = vec![EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call)];
    let _ = model.plot_strategy_breakdown(
        "Covered Call",
        model.covered_call(&instrument, &options[0]),
        20.0..80.0,
        "examples/images/covered_call.png",
        &options,
    );
    // => Covered Call: examples/images/covered_call.png

    let options = vec![EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put)];
    let _ = model.plot_strategy_breakdown(
        "Protective Put",
        model.protective_put(&instrument, &options[0]),
        20.0..80.0,
        "examples/images/protective_put.png",
        &options,
    );
    // => Protective Put: examples/images/protective_put.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Collar",
        model.collar(&instrument, &options[0], &options[1]),
        20.0..80.0,
        "examples/images/collar.png",
        &options,
    );
    // => Protective Put: examples/images/collar.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 50.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Fence",
        model.fence(&instrument, &options[0], &options[1], &options[2]),
        20.0..80.0,
        "examples/images/fence.png",
        &options,
    );
    // => Protective Put: examples/images/fence.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 60.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 40.0, 1.0, Call),
    ];

    let _ = model.plot_strategy_breakdown(
        "Guts",
        model.guts(&options[0], &options[1]),
        20.0..80.0,
        "examples/images/guts_strategy.png",
        &options,
    );
    // => Guts: examples/images/guts_strategy.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 50.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Straddle",
        model.straddle(&options[0], &options[1]),
        20.0..80.0,
        "examples/images/straddle_strategy.png",
        &options,
    );
    // => Straddle: examples/images/straddle_strategy.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Strangle",
        model.strangle(&options[0], &options[1]),
        20.0..80.0,
        "examples/images/strangle_strategy.png",
        &options,
    );
    // => Strangle: examples/images/strangle_strategy.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Risk Reversal",
        model.risk_reversal(&options[0], &options[1]),
        20.0..80.0,
        "examples/images/risk_reversal.png",
        &options,
    );
    // => Strangle: examples/images/risk_reversal.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 40.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Butterfly",
        model.butterfly(&options[0], &options[1], &options[2]),
        20.0..80.0,
        "examples/images/butterfly_strategy.png",
        &options,
    );
    // => Butterfly: examples/images/butterfly_strategy.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 50.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Iron Butterfly",
        model.iron_butterfly(&options[0], &options[1], &options[2], &options[3]),
        20.0..80.0,
        "examples/images/iron_butterfly_strategy.png",
        &options,
    );
    // => Iron Butterfly: examples/images/iron_butterfly_strategy.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 80.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 80.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Christmas Tree Butterfly",
        model.christmas_tree_butterfly(
            &options[0],
            &options[1],
            &options[2],
            &options[3],
            &options[4],
            &options[5],
        ),
        20.0..80.0,
        "examples/images/christmas_tree_butterfly_strategy.png",
        &options,
    );
    // => Christmas Tree Butterfly: examples/images/christmas_tree_butterfly_strategy.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 30.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 40.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Condor",
        model.condor(&options[0], &options[1], &options[2], &options[3]),
        20.0..80.0,
        "examples/images/condor_strategy.png",
        &options,
    );
    // => Condor: examples/images/condor_strategy.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 30.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 40.0, 1.0, Put),
        EuropeanOption::new(instrument.clone(), 60.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 70.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Iron Condor",
        model.iron_condor(&options[0], &options[1], &options[2], &options[3]),
        20.0..80.0,
        "examples/images/iron_condor_strategy.png",
        &options,
    ); // => Iron Condor: examples/images/iron_condor_strategy.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 55.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 55.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Back Spread",
        model.back_spread(&options[0], &options[1], &options[2]),
        20.0..80.0,
        "examples/images/back_spread_strategy.png",
        &options,
    ); // => Back Spread: examples/images/back_spread_strategy.png

    let options = vec![
        EuropeanOption::new(instrument.clone(), 50.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 55.0, 1.0, Call),
        EuropeanOption::new(instrument.clone(), 55.0, 1.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Ladder",
        model.ladder(&options[0], &options[1], &options[2]),
        20.0..80.0,
        "examples/images/ladder_strategy.png",
        &options,
    );

    let options = vec![
        EuropeanOption::new(instrument.clone(), 50.0, 1.0 / 12.0, Call),
        EuropeanOption::new(instrument.clone(), 50.0, 2.0 / 12.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Calendar Spread",
        model.calendar_spread(&options[0], &options[1]),
        20.0..80.0,
        "examples/images/calendar_spread_strategy.png",
        &options,
    ); // => Calendar Spread: examples/images/calendar_spread_strategy.png

    let options = vec![
        EuropeanOption::new(Instrument::new().with_spot(48.0), 49.0, 1.0 / 12.0, Call),
        EuropeanOption::new(Instrument::new().with_spot(48.0), 49.0, 1.0 / 12.0, Call),
        EuropeanOption::new(Instrument::new().with_spot(48.0), 50.0, 2.0 / 12.0, Call),
    ];
    let _ = model.plot_strategy_breakdown(
        "Diagonal Spread",
        model.diagonal_spread(&options[0], &options[1], &options[2]),
        40.0..60.0,
        "examples/images/diagonal_spread_strategy.png",
        &options,
    ); //model.diagonal_spread(&options[0], &options[1], &options[2])(50.0);
}

fn example_plots() {
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
    let _ = model.plot_strategy_breakdown(
        "Condor Example",
        model.condor(&options[0], &options[1], &options[2], &options[3]),
        80.0..120.0,
        "examples/images/strategy.png",
        &options,
    );
}
