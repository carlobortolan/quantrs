use quantrs::data::DataProvider;

#[tokio::main]
async fn main() {
    example_data_module().await;
}

async fn example_data_module() {
    // This example demonstrates how to use the DataProvider to fetch stock quotes.

    // Create a new DataProvider instance using Alpha Vantage.
    // Replace "demo" with your actual Alpha Vantage API key.
    let dp = DataProvider::AlphaVantage(String::from("demo"));

    // Fetch the stock quote for IBM.
    let quote = dp.get_stock_quote("IBM").await.unwrap();
    println!("IBM Quote: {:?}", quote);
}
