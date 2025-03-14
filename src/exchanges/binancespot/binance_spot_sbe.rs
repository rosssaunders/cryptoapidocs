use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binance_spot_sbe")]
pub struct BinanceSpotSbe;

#[async_trait]
impl ApiProcessor for BinanceSpotSbe {
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

impl BinanceSpotSbe {
    const ENDPOINTS: &'static [&'static str] = &[
        "binance-spot-api-docs/sbe-api/general-info",
        "binance-spot-api-docs/sbe-api/connect",
        "binance-spot-api-docs/sbe-api/market-data",
        "binance-spot-api-docs/sbe-api/trading",
        "binance-spot-api-docs/sbe-api/account",
        "binance-spot-api-docs/sbe-api/errors",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/binance_spot_sbe_api_docs.md";
    const TITLE: &'static str = "Binance Spot SBE API Documentation";
}

