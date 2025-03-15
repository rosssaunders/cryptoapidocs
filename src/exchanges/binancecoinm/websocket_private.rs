use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binancecoinm_private_websocket")]
#[exchange("binancecoinm")]
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
        "derivatives/quick-start",
        "derivatives/coin-margined-futures/websocket-api-general-info",
        "derivatives/coin-margined-futures/common-definition",
        "derivatives/coin-margined-futures/error-code",
        "derivatives/coin-margined-futures/user-data-streams",
        "derivatives/coin-margined-futures/user-data-streams/Start-User-Data-Stream",
        "derivatives/coin-margined-futures/user-data-streams/Keepalive-User-Data-Stream",
        "derivatives/coin-margined-futures/user-data-streams/Close-User-Data-Stream",
        "derivatives/coin-margined-futures/user-data-streams/Start-User-Data-Stream-Wsp",
        "derivatives/coin-margined-futures/user-data-streams/Keepalive-User-Data-Stream-Wsp",
        "derivatives/coin-margined-futures/user-data-streams/Close-User-Data-Stream-Wsp",
        "derivatives/coin-margined-futures/user-data-streams/Event-User-Data-Stream-Expired",
        "derivatives/coin-margined-futures/user-data-streams/Event-Margin-Call",
        "derivatives/coin-margined-futures/user-data-streams/Event-Balance-and-Position-Update",
        "derivatives/coin-margined-futures/user-data-streams/Event-Order-Update",
        "derivatives/coin-margined-futures/user-data-streams/Event-Account-Configuration-Update",
        "derivatives/coin-margined-futures/user-data-streams/Event-STRATEGY-UPDATE",
        "derivatives/coin-margined-futures/user-data-streams/Event-GRID-UPDATE",
        "derivatives/coin-margined-futures/trade/websocket-api",
        "derivatives/coin-margined-futures/trade/websocket-api/Modify-Order",
        "derivatives/coin-margined-futures/trade/websocket-api/Cancel-Order",
        "derivatives/coin-margined-futures/trade/websocket-api/Query-Order",
        "derivatives/coin-margined-futures/trade/websocket-api/Position-Information",
        "derivatives/coin-margined-futures/account/websocket-api",
        "derivatives/coin-margined-futures/account/websocket-api/Account-Information",
    ];

    const OUTPUT_FILE: &'static str = "binance/coinm/private_websocket_api.md";
    const TITLE: &'static str = "Binance COIN-M Private Websocket API Documentation";
}
