use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binancespot_sbe")]
#[exchange("binancespot")]
pub struct SbeMarketData;

#[async_trait]
impl ApiProcessor for SbeMarketData {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        let processor = DocProcessor::new(
            Self::ENDPOINTS,
            Self::OUTPUT_FILE,
            Self::TITLE
        );
        processor.process_docs().await
    }

    fn get_output_filename(&self) -> String {
        String::from(Self::OUTPUT_FILE)
    }
}

impl SbeMarketData {
    const ENDPOINTS: &'static [&'static str] = &[
        "binance-spot-api-docs/sbe-market-data-streams",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/market_data_sbe_api.md";
    const TITLE: &'static str = "Binance Spot SBE API Documentation";
}

