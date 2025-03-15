use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binanceoptions_private_websocket")]
#[exchange("binanceoptions")]
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
        "derivatives/option/general-info",
        "derivatives/option/common-definition",
        "derivatives/option/error-code",
        "derivatives/option/user-data-streams",
        "derivatives/option/user-data-streams/Start-User-Data-Stream",
        "derivatives/option/user-data-streams/Keepalive-User-Data-Stream",
        "derivatives/option/user-data-streams/Close-User-Data-Stream",
        "derivatives/option/user-data-streams/Event-Risk-level-change",
        "derivatives/option/user-data-streams/Event-Order-update",
        "derivatives/option/user-data-streams/Event-Account-data",
    ];
    
    const OUTPUT_FILE: &'static str = "binance/options/private_websocket_api.md";
    const TITLE: &'static str = "Binance Options Private Websocket API Documentation";
}
