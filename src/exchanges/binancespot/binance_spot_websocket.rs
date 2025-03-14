use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binance_spot_websocket")]
pub struct BinanceSpotWebsocket;

#[async_trait]
impl ApiProcessor for BinanceSpotWebsocket {
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

impl BinanceSpotWebsocket {
    const ENDPOINTS: &'static [&'static str] = &[
        "binance-spot-api-docs/websocket-api/general-info",
        "binance-spot-api-docs/websocket-api/connect",
        "binance-spot-api-docs/websocket-api/market-data",
        "binance-spot-api-docs/websocket-api/trading",
        "binance-spot-api-docs/websocket-api/account",
        "binance-spot-api-docs/websocket-api/errors",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/binance_spot_websocket_api_docs.md";
    const TITLE: &'static str = "Binance Spot Websocket API Documentation";
}
