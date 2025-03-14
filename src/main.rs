use std::error::Error;

mod exchanges;
mod utils;

use exchanges::binancederivatives::coinm_rest_public::PublicREST;
use exchanges::ApiProcessor;
use exchanges::ApiProcessorType;
use exchanges::binancederivatives::coinm_rest_private::PrivateRest;
use exchanges::binancederivatives::binance_derivatives_usdm::BinanceDerivativesUSDM;
use exchanges::binancespot::{
    binance_spot_rest::BinanceSpotRest,
    binance_spot_fix::BinanceSpotFix,
    binance_spot_websocket::BinanceSpotWebSocket,
    binance_spot_sbe::BinanceSpotSbe,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define the structure to hold results
    struct ExchangeResult {
        market: String,
        filename: String,
        timestamp: String,
        tokens: u32,
    }

    // Create a vector of API processors
    let processors = vec![
        ApiProcessorType::SpotRest(BinanceSpotRest::default()),
        ApiProcessorType::SpotFix(BinanceSpotFix::default()),
        ApiProcessorType::SpotWebSocket(BinanceSpotWebSocket::default()),
        ApiProcessorType::SpotSbe(BinanceSpotSbe::default()),
        ApiProcessorType::DerivativesUSDM(BinanceDerivativesUSDM::default()),
        ApiProcessorType::DerivativesCOINMPrivateRest(PrivateRest::default()),
        ApiProcessorType::DerivativesCOINMPublicREST(PublicREST::default()),
    ];

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
