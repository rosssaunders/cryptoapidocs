pub mod doc_processor;

use std::error::Error;
use async_trait::async_trait;

use doc_processor::DocProcessor;

use super::ApiProcessor;

#[async_trait]
pub trait BinanceApiProcessor: ApiProcessor {
    const ENDPOINTS: &'static [&'static str];
    const OUTPUT_FILE: &'static str;
    const TITLE: &'static str;

    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        let doc_processor = DocProcessor::new(
            Self::ENDPOINTS,
            Self::OUTPUT_FILE,
            Self::TITLE,
        );
        doc_processor.process_docs().await
    }

    fn get_output_filename(&self) -> String {
        Self::OUTPUT_FILE.to_string()
    }
}
