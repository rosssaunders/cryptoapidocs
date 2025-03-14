pub mod binancespot;
pub mod binancecommon;
pub mod binancederivatives;
pub mod processor_registry;

use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait ApiProcessor: Send + Sync {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>>;
    fn get_output_filename(&self) -> String;
}

// Re-export the registration macro
pub use crate::register_processor;

// Re-export key registry functions
pub use processor_registry::{create_processor, create_processors_by_type, get_registered_processor_names};
