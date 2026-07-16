// Run:  cargo run --release --example data

use quantrs::data::DataProvider;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        // Initialize the Alpha Vantage provider with your API key
        let provider = DataProvider::alpha_vantage("demo");

        // Fetch a real-time global quote
        match provider.get_stock_quote("IBM").await {
            Ok(quote) => println!("Quote: {}", quote),
            Err(e) => eprintln!("Error fetching quote: {}", e),
        }

        // Fetch deep company fundamentals
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
    });
}
