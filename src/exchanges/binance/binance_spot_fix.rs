use std::error::Error;
use crate::exchanges::binance::doc_processor::DocProcessor;

const ENDPOINTS: &[&str] = &[
    "filters",
    "enums",
    "fix-api",
    "errors",
];

const OUTPUT_FILE: &str = "binance_spot_fix_api_docs.md";
const TITLE: &str = "Binance Spot FIX API Documentation";

pub async fn process_fix_docs() -> Result<(u32, String, String), Box<dyn Error>> {
    let processor = DocProcessor::new(ENDPOINTS, OUTPUT_FILE, TITLE);
    processor.process_docs().await
}
