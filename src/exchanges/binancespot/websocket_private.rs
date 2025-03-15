use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binancespot_private_websocket")]
#[exchange("binancespot")]
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
        "binance-spot-api-docs/user-data-stream",
        "binance-spot-api-docs/errors",
        "binance-spot-api-docs/filters",
        "binance-spot-api-docs/enums",
        "binance-spot-api-docs/web-socket-streams",
        "binance-spot-api-docs/web-socket-api/general-api-information",
        "binance-spot-api-docs/web-socket-api/request-format",
        "binance-spot-api-docs/web-socket-api/response-format",
        "binance-spot-api-docs/web-socket-api/event-format",
        "binance-spot-api-docs/web-socket-api/request-security",
        "binance-spot-api-docs/web-socket-api/session-authentication",
        "binance-spot-api-docs/web-socket-api/data-sources",
        "binance-spot-api-docs/web-socket-api/general-requests",
        "binance-spot-api-docs/web-socket-api/market-data-requests",
        "binance-spot-api-docs/web-socket-api/authentication-requests",
        "binance-spot-api-docs/web-socket-api/trading-requests",
        "binance-spot-api-docs/web-socket-api/account-requests",
        "binance-spot-api-docs/web-socket-api/user-data-stream-requests",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/private_websocket_api.md";
    const TITLE: &'static str = "Binance Spot Private Websocket API Documentation";
}
