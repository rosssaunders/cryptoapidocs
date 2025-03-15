use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binanceusdm_public_websocket")]
#[exchange("binanceusdm")]
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
        "derivatives/usds-margined-futures/websocket-api-general-info",
        "derivatives/usds-margined-futures/common-definition",
        "derivatives/usds-margined-futures/error-code",
        "derivatives/usds-margined-futures/market-data/websocket-api",
        "derivatives/usds-margined-futures/market-data/websocket-api/Symbol-Price-Ticker",
        "derivatives/usds-margined-futures/market-data/websocket-api/Symbol-Order-Book-Ticker",
        "derivatives/usds-margined-futures/websocket-market-streams",
        "derivatives/usds-margined-futures/websocket-market-streams/Live-Subscribing-Unsubscribing-to-streams",
        "derivatives/usds-margined-futures/websocket-market-streams/Aggregate-Trade-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/Mark-Price-Stream",
        "derivatives/usds-margined-futures/websocket-market-streams/Mark-Price-Stream-for-All-market",
        "derivatives/usds-margined-futures/websocket-market-streams/Kline-Candlestick-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/Continuous-Contract-Kline-Candlestick-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/Individual-Symbol-Mini-Ticker-Stream",
        "derivatives/usds-margined-futures/websocket-market-streams/All-Market-Tickers-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/Individual-Symbol-Ticker-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/All-Market-Mini-Tickers-Stream",
        "derivatives/usds-margined-futures/websocket-market-streams/Individual-Symbol-Book-Ticker-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/All-Book-Tickers-Stream",
        "derivatives/usds-margined-futures/websocket-market-streams/Liquidation-Order-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/All-Market-Liquidation-Order-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/Partial-Book-Depth-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/Diff-Book-Depth-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/How-to-manage-a-local-order-book-correctly",
        "derivatives/usds-margined-futures/websocket-market-streams/Composite-Index-Symbol-Information-Streams",
        "derivatives/usds-margined-futures/websocket-market-streams/Contract-Info-Stream",
        "derivatives/usds-margined-futures/websocket-market-streams/Multi-Assets-Mode-Asset-Index",
    ];

    const OUTPUT_FILE: &'static str = "binance/usdm/public_websocket_api.md";
    const TITLE: &'static str = "Binance USDM Public Websocket API Documentation";
}
