use std::error::Error;

mod exchanges;
mod utils;

use exchanges::binancederivatives::binance_deriviatves_usdm::BinanceDerivativesUSDM;
use exchanges::ApiProcessor;
use exchanges::binancespot::{
    binance_spot_rest::BinanceSpotRest,
    binance_spot_fix::BinanceSpotFix,
    binance_spot_websocket::BinanceSpotWebSocket,
    binance_spot_sbe::BinanceSpotSbe,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Collect all results first
    // let rest_result = BinanceSpotRest::default().process_docs().await?;
    // let fix_result = BinanceSpotFix::default().process_docs().await?;
    // let websocket_result = BinanceSpotWebSocket::default().process_docs().await?;
    // let sbe_result = BinanceSpotSbe::default().process_docs().await?;
    let usdm_result = BinanceDerivativesUSDM::default().process_docs().await?;
    
    // Print the table at the end
    println!("\n| Market | Generated File | Timestamp | Token Count |");
    println!("|--------|----------------|-----------|-------------|");
    
    // let (tokens, timestamp, market) = rest_result;
    // println!("| {} | binance_spot_rest_api_docs.md | {} | {} |", market, timestamp, tokens);
    
    // let (tokens, timestamp, market) = fix_result;
    // println!("| {} | binance_spot_fix_api_docs.md | {} | {} |", market, timestamp, tokens);
    
    // let (tokens, timestamp, market) = websocket_result;
    // println!("| {} | binance_spot_websocket_api_docs.md | {} | {} |", market, timestamp, tokens);
    
    // let (tokens, timestamp, market) = sbe_result;
    // println!("| {} | binance_spot_sbe_api_docs.md | {} | {} |", market, timestamp, tokens);

    let (tokens, timestamp, market) = usdm_result;
    println!("| {} | binance_deriviatves_usdm_api_docs.md | {} | {} |", market, timestamp, tokens);
    
    Ok(())
}
