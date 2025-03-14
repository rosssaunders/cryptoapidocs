pub mod binancespot;
pub mod binancecommon;
pub mod binancederivatives;

use std::error::Error;

pub trait ApiProcessor {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>>;
}
