use std::error::Error;
use crate::exchanges::{ApiProcessor, binancecommon::BinanceApiProcessor};

#[derive(Default)]
pub struct BinanceSpotRest;

impl ApiProcessor for BinanceSpotRest {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        BinanceApiProcessor::process_docs(self).await
    }

    fn get_output_filename(&self) -> String {
        BinanceApiProcessor::get_output_filename(self)
    }
}

impl BinanceApiProcessor for BinanceSpotRest {
    const ENDPOINTS: &'static [&'static str] = &[
        "binance-spot-api-docs/filters",
        "binance-spot-api-docs/enums",
        "binance-spot-api-docs/rest-api/general-api-information",
        "binance-spot-api-docs/rest-api/http-return-codes",
        "binance-spot-api-docs/rest-api/error-codes",
        "binance-spot-api-docs/rest-api/general-information-on-endpoints",
        "binance-spot-api-docs/rest-api/limits",
        "binance-spot-api-docs/rest-api/data-sources",
        "binance-spot-api-docs/rest-api/endpoint-security-type",
        "binance-spot-api-docs/rest-api/general-endpoints",
        "binance-spot-api-docs/rest-api/market-data-endpoints",
        "binance-spot-api-docs/rest-api/trading-endpoints",
        "binance-spot-api-docs/rest-api/account-endpoints",
        "binance-spot-api-docs/rest-api/user-data-stream",
        "binance-spot-api-docs/rest-api/user-data-stream-endpoints",
        "binance-spot-api-docs/rest-api/errors",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/binance_spot_rest_api_docs.md";
    const TITLE: &'static str = "Binance Spot REST API Documentation";
}