//! Module implementing Mean-Variance Portfolio Optimization
//!
//! Current Support:
//! - Portfolio struct to hold assets, mean returns, covariance matrix
//! - Calculation of mean returns and covariance matrix from CSV data
//! - Support for Simple and Log returns calculation methods
//! Next Steps:
//! - Implement optimization algorithms (e.g., Markowitz optimization)

use csv::ReaderBuilder;
use ndarray::{s, Array2, Axis};
use ndarray_stats::CorrelationExt;
use std::fmt;

#[derive(Debug)]
pub enum ReturnsCalculation {
    Simple,
    Log,
}

#[derive(Debug)]
/// Struct representing a portfolio of assets.
pub struct Portfolio {
    /// List of asset tickers in the portfolio.
    tickers: Vec<String>,
    /// Mean returns of the assets.
    mean_returns: Vec<f64>,
    /// Covariance matrix of the asset returns.
    covariance_matrix: Array2<f64>,
    /// Risk-free rate for the market
    risk_free_rate: f64,
    /// Expected return for the portfolio
    expected_return: f64,
    /// Method used to calculate returns (Simple or Log)
    returns_calculation: ReturnsCalculation,
    /// Weights of the assets in the portfolio (if calculated)
    /// None if not yet calculated
    weights: Option<Vec<f64>>,
    /// Internal storage of returns data
    _returns: Array2<f64>,
}

impl Portfolio {
    pub fn new(
        data_path: &str,
        risk_free_rate: f64,
        expected_return: f64,
        returns_calc: ReturnsCalculation,
    ) -> Self {
        // Read data from CSV file given file path
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(data_path)
            .expect("Failed to read CSV file");

        // Extract tickers from headers
        let tickers: Vec<String> = rdr
            .headers()
            .expect("Failed to read headers")
            .iter()
            .map(|s| s.to_string())
            .collect();

        // Read records and parse to f64, defaulting to 0.0 on parse failure
        let records: Vec<Vec<f64>> = rdr
            .records()
            .map(|result| {
                result
                    .expect("Failed to read record")
                    .iter()
                    .map(|s| s.parse::<f64>().unwrap_or(0.0))
                    .collect()
            })
            .collect();

        let n = tickers.len();
        let m = records.len();

        // Create ndarray from records to store prices
        let prices = Array2::from_shape_vec((m, n), records.into_iter().flatten().collect())
            .expect("Failed to create prices array");

        // Calculate returns based on specified method
        let returns = match returns_calc {
            ReturnsCalculation::Simple => Self::calculate_simple_returns(&prices),
            ReturnsCalculation::Log => Self::calculate_log_returns(&prices),
        };

        // Calculate mean returns
        let mean_returns = returns.mean_axis(Axis(0)).unwrap().to_vec();

        // Calculate covariance matrix
        // Using unbiased estimator (N-1 in denominator) for sample covariance
        // ddof = 1.
        // If you want population covariance, use ddof = 0
        // Transpose of returns is used as ndarray-stats expects variables in rows
        let covariance_matrix = returns
            .t()
            .cov(1.0)
            .expect("Failed to compute covariance matrix");

        Self {
            tickers: tickers,
            mean_returns: mean_returns,
            covariance_matrix: covariance_matrix,
            risk_free_rate: risk_free_rate,
            expected_return: expected_return,
            returns_calculation: returns_calc,
            weights: None,
            _returns: returns,
        }
    }

    /// Function to calculate log returns
    fn calculate_log_returns(prices: &Array2<f64>) -> Array2<f64> {
        let log_prices = prices.mapv(|x| x.ln());
        let log_returns = &log_prices.slice(s![1.., ..]) - &log_prices.slice(s![..-1, ..]);
        log_returns.to_owned()
    }

    /// Function to calculate simple returns
    fn calculate_simple_returns(prices: &Array2<f64>) -> Array2<f64> {
        let simple_returns = (&prices.slice(s![1.., ..]) - &prices.slice(s![..-1, ..]))
            / &prices.slice(s![..-1, ..]);
        simple_returns.to_owned()
    }
}

/// Implement Display trait for pretty-printing the Portfolio
impl fmt::Display for Portfolio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Portfolio")?;
        writeln!(f, "Tickers: {:?}", self.tickers)?;
        writeln!(f, "Risk-Free Rate: {:.4}", self.risk_free_rate)?;
        writeln!(f, "Expected Return: {:.4}", self.expected_return)?;
        writeln!(f, "Returns Calculation: {}", self.returns_calculation)?;
        writeln!(f, "Weights: {:?}", self.weights)?;

        // Print mean returns as percentages
        writeln!(f, "\nMean Returns (%):")?;
        for (i, &mean_return) in self.mean_returns.iter().enumerate() {
            writeln!(f, "{:>8}: {:>8.4}%", self.tickers[i], mean_return * 100.0)?;
        }

        // Print covariance matrix in a readable format
        writeln!(
            f,
            "\nCovariance Matrix ({} x {}):",
            self.covariance_matrix.nrows(),
            self.covariance_matrix.ncols()
        )?;
        // Print header with ticker names
        write!(f, "         ")?; // spacing for row labels
        for ticker in &self.tickers {
            write!(f, "{:>10}", ticker)?;
        }
        writeln!(f)?;

        // Print each row with ticker name as label
        for (i, row) in self.covariance_matrix.outer_iter().enumerate() {
            write!(f, "{:>8} ", self.tickers[i])?; // row label
            for value in row {
                write!(f, "{:>10.6}", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Implement Display trait for ReturnsCalculation enum
impl fmt::Display for ReturnsCalculation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReturnsCalculation::Simple => write!(f, "Simple"),
            ReturnsCalculation::Log => write!(f, "Log"),
        }
    }
}
