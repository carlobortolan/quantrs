use statrs::function::erf::erf;

pub fn black_scholes_call_price(
    spot: f64,
    strike: f64,
    time_to_maturity: f64,
    risk_free_rate: f64,
    volatility: f64,
) -> f64 {
    let d1 = ((spot / strike).ln()
        + (risk_free_rate + 0.5 * volatility.powi(2)) * time_to_maturity)
        / (volatility * time_to_maturity.sqrt());
    let d2 = d1 - volatility * time_to_maturity.sqrt();

    spot * norm_cdf(d1) - strike * (-risk_free_rate * time_to_maturity).exp() * norm_cdf(d2)
}

fn norm_cdf(x: f64) -> f64 {
    (1.0 + erf(x / (2.0_f64).sqrt())) / 2.0
}
