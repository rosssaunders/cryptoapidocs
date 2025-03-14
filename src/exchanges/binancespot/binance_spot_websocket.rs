use std::error::Error;
use crate::exchanges::{ApiProcessor, binancecommon::BinanceApiProcessor};

#[derive(Default)]
pub struct BinanceSpotWebSocket;

impl ApiProcessor for BinanceSpotWebSocket {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        BinanceApiProcessor::process_docs(self).await
    }

    fn get_output_filename(&self) -> String {
        BinanceApiProcessor::get_output_filename(self)
    }
}

impl BinanceApiProcessor for BinanceSpotWebSocket {
    const ENDPOINTS: &'static [&'static str] = &[
        "binance-spot-api-docs/filters",
        "binance-spot-api-docs/enums",
        "binance-spot-api-docs/websocket-api",
        "binance-spot-api-docs/errors",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/binance_spot_websocket_api_docs.md";
    const TITLE: &'static str = "Binance Spot WebSocket API Documentation";
}
