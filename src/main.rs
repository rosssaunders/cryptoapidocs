use std::error::Error;
use clap::{Parser, ValueEnum};

mod exchanges;
mod utils;
mod token_counter;

#[derive(Debug, Clone, ValueEnum)]
enum Command {
    #[value(name = "process")]
    Process,
    #[value(name = "count")]
    Count,
}

#[derive(Debug, Clone, ValueEnum)]
enum Exchange {
    #[value(name = "binancespot")]
    BinanceSpot,
    #[value(name = "binanceusdm")]
    BinanceUSDM,
    #[value(name = "binancecoinm")]
    BinanceCoinM,
    #[value(name = "binanceoptions")]
    BinanceOptions,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The command to run (process or count)
    #[arg(value_enum)]
    command: Command,

    /// The exchange to generate API documentation for (only needed for process command)
    #[arg(value_enum, required_if_eq("command", "process"))]
    exchange: Option<Exchange>,
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

    match args.command {
        Command::Process => {
            // Get processors based on exchange type
            let processors = match args.exchange.unwrap() {
                Exchange::BinanceUSDM => exchanges::processor_registry::create_processors_by_exchange("binanceusdm"),
                Exchange::BinanceSpot => exchanges::processor_registry::create_processors_by_exchange("binancespot"),
                Exchange::BinanceCoinM => exchanges::processor_registry::create_processors_by_exchange("binancecoinm"),
                Exchange::BinanceOptions => exchanges::processor_registry::create_processors_by_exchange("binanceoptions"),
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
        }
        Command::Count => {
            token_counter::process_all_docs()?;
        }
    }

    Ok(())
}
