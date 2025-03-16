use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use super::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("bybit_private_websocket")]
#[exchange("bybit")]
pub struct PrivateWebsocket;

#[async_trait]
impl ApiProcessor for PrivateWebsocket {
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

impl PrivateWebsocket {
    const ENDPOINTS: &'static [&'static str] = &[
        "rate-limit",
        "enum",
        "error",
        "ws/connect",
        "websocket/private/position",
        "websocket/private/execution",
        "websocket/private/fast-execution",
        "websocket/private/order",
        "websocket/private/wallet",
        "websocket/private/greek",
        "websocket/private/dcp",
        "websocket/trade/guideline"
    ];
    
    const OUTPUT_FILE: &'static str = "bybit/v5/private_websocket_api.md";
    const TITLE: &'static str = "ByBit V5 Private Websocket API Documentation";
}
