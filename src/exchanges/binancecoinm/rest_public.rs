use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binancecoinm_public_rest")]
#[exchange("binancecoinm")]
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
        "derivatives/coin-margined-futures/general-info",
        "derivatives/coin-margined-futures/common-definition",
        "derivatives/coin-margined-futures/error-code",

        "derivatives/coin-margined-futures/market-data/rest-api",
        "derivatives/coin-margined-futures/market-data/rest-api/Check-Server-time",
        "derivatives/coin-margined-futures/market-data/rest-api/Exchange-Information",
        "derivatives/coin-margined-futures/market-data/rest-api/Order-Book",
        "derivatives/coin-margined-futures/market-data/rest-api/Recent-Trades-List",
        "derivatives/coin-margined-futures/market-data/rest-api/Old-Trades-Lookup",
        "derivatives/coin-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List",
        "derivatives/coin-margined-futures/market-data/rest-api/Index-Price-and-Mark-Price",
        "derivatives/coin-margined-futures/market-data/rest-api/Get-Funding-Rate-History-of-Perpetual-Futures",
        "derivatives/coin-margined-futures/market-data/rest-api/Get-Funding-Info",
        "derivatives/coin-margined-futures/market-data/rest-api/Kline-Candlestick-Data",
        "derivatives/coin-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data",
        "derivatives/coin-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data",
        "derivatives/coin-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data",
        "derivatives/coin-margined-futures/market-data/rest-api/Premium-Index-Kline-Data",
        "derivatives/coin-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics",
        "derivatives/coin-margined-futures/market-data/rest-api/Symbol-Price-Ticker",
        "derivatives/coin-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker",
        "derivatives/coin-margined-futures/market-data/rest-api/Open-Interest",
        "derivatives/coin-margined-futures/market-data/rest-api/Open-Interest-Statistics",
        "derivatives/coin-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio",
        "derivatives/coin-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio",
        "derivatives/coin-margined-futures/market-data/rest-api/Long-Short-Ratio",
        "derivatives/coin-margined-futures/market-data/rest-api/Taker-Buy-Sell-Volume",
        "derivatives/coin-margined-futures/market-data/rest-api/Basis",
        "derivatives/coin-margined-futures/market-data/rest-api/Index-Constituents",
    ];

    const OUTPUT_FILE: &'static str = "binance/coinm/public_rest_api.md";
    const TITLE: &'static str = "Binance COIN-M Public REST API Documentation";
}
