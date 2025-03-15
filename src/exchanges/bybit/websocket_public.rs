use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use super::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("bybit_public_websocket")]
#[exchange("bybit")]
pub struct PublicWebsocket;

#[async_trait]
impl ApiProcessor for PublicWebsocket {
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

impl PublicWebsocket {
    const ENDPOINTS: &'static [&'static str] = &[
        "rate-limit",
        "enum",
        "error",
        "websocket/public/orderbook",
        "websocket/public/trade",
        "websocket/public/ticker",
        "websocket/public/kline",
        "websocket/public/all-liquidation",
        "websocket/public/etp-kline",
        "websocket/public/etp-ticker",
        "websocket/public/etp-nav",
        "websocket/public/liquidation"
    ];

    const OUTPUT_FILE: &'static str = "bybit/v5/public_websocket_api.md";
    const TITLE: &'static str = "ByBit V5 Public Websocket API Documentation";
}
