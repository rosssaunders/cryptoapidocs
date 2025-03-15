use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binanceusdm_public_rest")]
#[exchange("binanceusdm")]
pub struct PublicREST;

#[async_trait]
impl ApiProcessor for PublicREST {
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

impl PublicREST {
    const ENDPOINTS: &'static [&'static str] = &[
        "derivatives/quick-start",
        "derivatives/usds-margined-futures/general-info",
        "derivatives/usds-margined-futures/common-definition",
        "derivatives/usds-margined-futures/error-code",

        "derivatives/usds-margined-futures/market-data/rest-api",
        "derivatives/usds-margined-futures/market-data/rest-api/Check-Server-Time",
        "derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information",
        "derivatives/usds-margined-futures/market-data/rest-api/Order-Book",
        "derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List",
        "derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup",
        "derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List",
        "derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data",
        "derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data",
        "derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data",
        "derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data",
        "derivatives/usds-margined-futures/market-data/rest-api/Premium-Index-Kline-Data",
        "derivatives/usds-margined-futures/market-data/rest-api/Mark-Price",
        "derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History",
        "derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info",
        "derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics",
        "derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker",
        "derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker-v2",
        "derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker",
        "derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price",
        "derivatives/usds-margined-futures/market-data/rest-api/Open-Interest",
        "derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics",
        "derivatives/usds-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio",
        "derivatives/usds-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio",
        "derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio",
        "derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume",
        "derivatives/usds-margined-futures/market-data/rest-api/Basis",
        "derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information",
        "derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index",
        "derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents"
    ];

    const OUTPUT_FILE: &'static str = "binance/usdm/public_rest_api.md";
    const TITLE: &'static str = "Binance USDM Public REST API Documentation";
}
