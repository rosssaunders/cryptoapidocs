pub mod binancespot;
pub mod binancederivatives;
pub mod processor_registry;
pub mod doc_processor;

use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait ApiProcessor: Send + Sync {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>>;
    fn get_output_filename(&self) -> String;
}

// Re-export key registry functions
pub use processor_registry::create_processors_by_type;
