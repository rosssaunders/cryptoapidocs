use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binanceoptions_public_websocket")]
#[exchange("binanceoptions")]
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
        "derivatives/quick-start",
        "derivatives/option/general-info",
        "derivatives/option/common-definition",
        "derivatives/option/error-code",
        "derivatives/option/websocket-market-streams",
        "derivatives/option/websocket-market-streams/Live-Subscribing-Unsubscribing-to-streams",
        "derivatives/option/websocket-market-streams/New-Symbol-Info",
        "derivatives/option/websocket-market-streams/Open-Interest",
        "derivatives/option/websocket-market-streams/Mark-Price",
        "derivatives/option/websocket-market-streams/Kline-Candlestick-Streams",
        "derivatives/option/websocket-market-streams/24-hour-TICKER-by-underlying-asset-and-expiration-data",
        "derivatives/option/websocket-market-streams/Index-Price-Streams",
        "derivatives/option/websocket-market-streams/24-hour-TICKER",
        "derivatives/option/websocket-market-streams/Trade-Streams",
        "derivatives/option/websocket-market-streams/Partial-Book-Depth-Streams",
    ];

    const OUTPUT_FILE: &'static str = "binance/options/public_websocket_api.md";
    const TITLE: &'static str = "Binance Options Public Websocket API Documentation";
}
