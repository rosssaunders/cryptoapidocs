use std::error::Error;
use crate::exchanges::{ApiProcessor, binancecommon::BinanceApiProcessor};

#[derive(Default)]
pub struct BinanceSpotSbe;

impl ApiProcessor for BinanceSpotSbe {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        BinanceApiProcessor::process_docs(self).await
    }
}

impl BinanceApiProcessor for BinanceSpotSbe {
    const ENDPOINTS: &'static [&'static str] = &[
        "binance-spot-api-docs/filters",
        "binance-spot-api-docs/enums",
        "binance-spot-api-docs/sbe-api",
        "binance-spot-api-docs/errors",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/binance_spot_sbe_api_docs.md";
    const TITLE: &'static str = "Binance Spot SBE API Documentation";
}

