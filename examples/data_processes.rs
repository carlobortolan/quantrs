use quantrs::data::DataProvider;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async {
        example_data_module().await;
        example_company_overview().await;
    });
}

async fn example_data_module() {
    // This example demonstrates how to use the DataProvider to fetch stock quotes.

    // Create a new DataProvider instance using Alpha Vantage.
    // Replace "demo" with your actual Alpha Vantage API key.
    let dp = DataProvider::alpha_vantage("demo");

    // Fetch the stock quote for IBM.
    match dp.get_stock_quote("IBM").await {
        Ok(quote) => {
            // Now you can simply use {} to get pretty formatted output
            println!("{}", quote);
        }
        Err(e) => {
            eprintln!("Error fetching quote: {}", e);
        }
    }
}

async fn example_company_overview() {
    // This example demonstrates how to use the DataProvider to fetch company overview data.

    // Create a new DataProvider instance using Alpha Vantage.
    // Replace "demo" with your actual Alpha Vantage API key.
    let dp = DataProvider::alpha_vantage("demo");

    // Fetch the company overview for IBM.
    let overview = dp.get_company_overview("IBM").await.unwrap();
    println!("IBM Company Overview: {:?}", overview);
}
