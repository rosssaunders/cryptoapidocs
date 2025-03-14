use std::error::Error;

mod exchanges;
mod utils;

use exchanges::binance::binance_spot_rest::process_rest_docs;
use exchanges::binance::binance_spot_fix::process_fix_docs;
use exchanges::binance::binance_spot_websocket::process_websocket_docs;
use exchanges::binance::binance_spot_sbe::process_sbe_docs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Collect all results first
    let rest_result = process_rest_docs().await?;
    let fix_result = process_fix_docs().await?;
    let websocket_result = process_websocket_docs().await?;
    let sbe_result = process_sbe_docs().await?;
    
    // Print the table at the end
    println!("\n| Market | Generated File | Timestamp | Token Count |");
    println!("|--------|----------------|-----------|-------------|");
    
    let (tokens, timestamp, market) = rest_result;
    println!("| {} | binance_spot_rest_api_docs.md | {} | {} |", market, timestamp, tokens);
    
    let (tokens, timestamp, market) = fix_result;
    println!("| {} | binance_spot_fix_api_docs.md | {} | {} |", market, timestamp, tokens);
    
    let (tokens, timestamp, market) = websocket_result;
    println!("| {} | binance_spot_websocket_api_docs.md | {} | {} |", market, timestamp, tokens);
    
    let (tokens, timestamp, market) = sbe_result;
    println!("| {} | binance_spot_sbe_api_docs.md | {} | {} |", market, timestamp, tokens);
    
    Ok(())
}
