use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binance_spot_fix")]
pub struct BinanceSpotFix;

#[async_trait]
impl ApiProcessor for BinanceSpotFix {
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

impl BinanceSpotFix {
    const ENDPOINTS: &'static [&'static str] = &[
        "binance-spot-api-docs/fix-api/general-info",
        "binance-spot-api-docs/fix-api/connect",
        "binance-spot-api-docs/fix-api/market-data",
        "binance-spot-api-docs/fix-api/trading",
        "binance-spot-api-docs/fix-api/account",
        "binance-spot-api-docs/fix-api/errors",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/binance_spot_fix_api_docs.md";
    const TITLE: &'static str = "Binance Spot FIX API Documentation";
}

