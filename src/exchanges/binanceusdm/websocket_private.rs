use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binanceusdm_private_websocket")]
#[exchange("binanceusdm")]
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
        "derivatives/usds-margined-futures/error-code",
        "derivatives/usds-margined-futures/websocket-api-general-info",
        "derivatives/usds-margined-futures/trade/websocket-api/Modify-Order",
        "derivatives/usds-margined-futures/trade/websocket-api/Cancel-Order",
        "derivatives/usds-margined-futures/trade/websocket-api/Query-Order",
        "derivatives/usds-margined-futures/user-data-streams",
        "derivatives/usds-margined-futures/user-data-streams/Start-User-Data-Stream",
        "derivatives/usds-margined-futures/user-data-streams/Keepalive-User-Data-Stream",
        "derivatives/usds-margined-futures/user-data-streams/Close-User-Data-Stream",
        "derivatives/usds-margined-futures/user-data-streams/Start-User-Data-Stream-Wsp",
        "derivatives/usds-margined-futures/user-data-streams/Keepalive-User-Data-Stream-Wsp",
        "derivatives/usds-margined-futures/user-data-streams/Close-User-Data-Stream-Wsp",
        "derivatives/usds-margined-futures/user-data-streams/Event-User-Data-Stream-Expired",
        "derivatives/usds-margined-futures/user-data-streams/Event-Balance-and-Position-Update",
        "derivatives/usds-margined-futures/user-data-streams/Event-Margin-Call",
        "derivatives/usds-margined-futures/user-data-streams/Event-Order-Update",
        "derivatives/usds-margined-futures/user-data-streams/Event-Trade-Lite",
        "derivatives/usds-margined-futures/user-data-streams/Event-Account-Configuration-Update-previous-Leverage-Update",
        "derivatives/usds-margined-futures/user-data-streams/Event-STRATEGY-UPDATE",
        "derivatives/usds-margined-futures/user-data-streams/Event-GRID-UPDATE",
        "derivatives/usds-margined-futures/user-data-streams/Event-Conditional-Order-Trigger-Reject",
        "derivatives/usds-margined-futures/account/websocket-api",
        "derivatives/usds-margined-futures/account/websocket-api/Futures-Account-Balance",
        "derivatives/usds-margined-futures/account/websocket-api/Account-Information-V2",
        "derivatives/usds-margined-futures/account/websocket-api/Account-Information",
    ];
    
    const OUTPUT_FILE: &'static str = "binance/usdm/private_websocket_api.md";
    const TITLE: &'static str = "Binance USDM Private Websocket API Documentation";
}
