use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binancecoinm_public_websocket")]
#[exchange("binancecoinm")]
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
        "derivatives/coin-margined-futures/websocket-api-general-info",
        "derivatives/coin-margined-futures/common-definition",
        "derivatives/coin-margined-futures/error-code",
        "derivatives/coin-margined-futures/websocket-market-streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Live-Subscribing-Unsubscribing-to-streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Aggregate-Trade-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Index-Price-Stream",
        "derivatives/coin-margined-futures/websocket-market-streams/Mark-Price-Stream",
        "derivatives/coin-margined-futures/websocket-market-streams/Kline-Candlestick-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Continuous-Contract-Kline-Candlestick-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Mark-Price-of-All-Symbols-of-a-Pair",
        "derivatives/coin-margined-futures/websocket-market-streams/Index-Kline-Candlestick-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Mark-Price-Kline-Candlestick-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Individual-Symbol-Mini-Ticker-Stream",
        "derivatives/coin-margined-futures/websocket-market-streams/All-Market-Mini-Tickers-Stream",
        "derivatives/coin-margined-futures/websocket-market-streams/Individual-Symbol-Ticker-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/All-Market-Tickers-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Individual-Symbol-Book-Ticker-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/All-Book-Tickers-Stream",
        "derivatives/coin-margined-futures/websocket-market-streams/Liquidation-Order-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/All-Market-Liquidation-Order-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Contract-Info-Stream",
        "derivatives/coin-margined-futures/websocket-market-streams/Partial-Book-Depth-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/Diff-Book-Depth-Streams",
        "derivatives/coin-margined-futures/websocket-market-streams/How-to-manage-a-local-order-book-correctly",
    ];

    const OUTPUT_FILE: &'static str = "binance/coinm/public_websocket_api.md";
    const TITLE: &'static str = "Binance COIN-M Public Websocket API Documentation";
}
