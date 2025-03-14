use std::error::Error;
use crate::exchanges::{ApiProcessor, binancecommon::BinanceApiProcessor};

#[derive(Default)]
pub struct BinanceSpotFix;

impl ApiProcessor for BinanceSpotFix {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        BinanceApiProcessor::process_docs(self).await
    }

    fn get_output_filename(&self) -> String {
        BinanceApiProcessor::get_output_filename(self)
    }
}

impl BinanceApiProcessor for BinanceSpotFix {
    const ENDPOINTS: &'static [&'static str] = &[
        "binance-spot-api-docs/filters",
        "binance-spot-api-docs/enums",
        "binance-spot-api-docs/fix-api",
        "binance-spot-api-docs/errors",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/binance_spot_fix_api_docs.md";
    const TITLE: &'static str = "Binance Spot FIX API Documentation";
}

