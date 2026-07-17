// Run:  cargo run --release --example data

use quantrs::data::DataProvider;
use std::time::Duration;
use tokio::runtime::Builder;

const YOUR_ALPHA_VANTAGE_API_KEY: &str = "demo"; // Replace with your actual Alpha Vantage API key (demo key only works for IBM)
const YOUR_MASSIVE_API_KEY: &str = "YOUR_API_KEY"; // Replace with your actual Massive API key

async fn alpha_vantage_demo() {
    println!("=== ALPHA VANTAGE ===");

    // Initialize the Alpha Vantage provider with your API key
    let provider = DataProvider::alpha_vantage(YOUR_ALPHA_VANTAGE_API_KEY);

    // Fetch a real-time global quote
    match provider.get_stock_quote("IBM").await {
        Ok(quote) => println!("Quote: {}", quote),
        Err(e) => eprintln!("Error fetching quote: {}", e),
    }

    // Alpha Vantage free tier limits to 1 request per second.
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Fetch fundamentals for IBM
    match provider.get_company_overview("IBM").await {
        Ok(company) => {
            println!("Company: {}", company.name);
            println!(
                "P/E Ratio: {}",
                company
                    .pe_ratio
                    .map(|v| v.to_string())
                    .unwrap_or("N/A".to_string())
            );
            println!(
                "Dividend Yield: {:.2}%",
                company.dividend_yield.unwrap_or(0.0) * 100.0
            );
        }
        Err(e) => eprintln!("Error fetching overview: {}", e),
    }
}

async fn yahoo_finance_demo() {
    println!("\n=== YAHOO FINANCE ===");

    // Initialize the Yahoo Finance provider (no API key required)
    let yf_provider = DataProvider::yahoo_finance();

    // Fetch a real-time global quote
    match yf_provider.get_stock_quote("IBM").await {
        Ok(quote) => println!("Quote: {}", quote),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Fetch fundamentals for IBM
    match yf_provider.get_company_overview("IBM").await {
        Ok(company) => {
            println!("Fundamentals:\n{}", company);
            println!("52-Week High: ${:.2}", company.week_52_high.unwrap_or(0.0));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn massive_demo() {
    println!("\n=== MASSIVE (formerly Polygon.io) ===");

    // Initialize the Massive provider
    // NOTE: .com
    let massive_provider = DataProvider::massive(YOUR_MASSIVE_API_KEY);

    // Fetch a real-time global quote
    match massive_provider.get_stock_quote("IBM").await {
        Ok(quote) => println!("Quote: {}", quote),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Fetch fundamentals for IBM
    match massive_provider.get_company_overview("IBM").await {
        Ok(company) => {
            println!("Company: {}", company.name);
            println!("Exchange: {}", company.exchange);
            println!(
                "Market Cap: {}",
                company
                    .market_capitalization
                    .map(|v| format!("${:.2}", v))
                    .unwrap_or("N/A".to_string())
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn main() {
    // We only need to create the runtime once in main!
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        alpha_vantage_demo().await;
        yahoo_finance_demo().await;
        massive_demo().await;
    });
}
