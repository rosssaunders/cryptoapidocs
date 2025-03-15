use std::error::Error;
use clap::{Parser, ValueEnum};

mod exchanges;
mod utils;

#[derive(Debug, Clone, ValueEnum)]
enum Exchange {
    #[value(name = "binancespot")]
    BinanceSpot,
    #[value(name = "binanceusdm")]
    BinanceUSDM,
    #[value(name = "binancecoinm")]
    BinanceCoinM,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The exchange to generate API documentation for
    #[arg(value_enum)]
    exchange: Exchange,
}

// Define the structure to hold results
struct ExchangeResult {
    market: String,
    filename: String,
    timestamp: String,
    tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Get processors based on exchange type
    let processors = match args.exchange {
        Exchange::BinanceUSDM => exchanges::processor_registry::create_processors_by_exchange("binanceusdm"),
        Exchange::BinanceSpot => exchanges::processor_registry::create_processors_by_exchange("binancespot"),
        Exchange::BinanceCoinM => exchanges::processor_registry::create_processors_by_exchange("binancecoinm"),
    };

    // Process each exchange and collect results
    let mut results = Vec::new();
    for processor in processors {
        let (tokens, timestamp, market) = processor.process_docs().await?;
        results.push(ExchangeResult {
            market,
            filename: processor.get_output_filename(),
            timestamp,
            tokens,
        });
    }
    
    // Print the table header
    println!("\n| Market | Generated File | Timestamp | Token Count |");
    println!("|--------|----------------|-----------|-------------|");
    
    // Print all results
    for result in results {
        println!("| {} | {} | {} | {} |", 
            result.market, 
            result.filename, 
            result.timestamp, 
            result.tokens
        );
    }

    Ok(())
}
