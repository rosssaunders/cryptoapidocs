use std::error::Error;

mod exchanges;
mod utils;

use exchanges::binance::binance_spot_rest::process_docs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    process_docs().await?;

    Ok(())
}
