use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binanceoptions_public_rest")]
#[exchange("binanceoptions")]
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
        "derivatives/option/general-info",
        "derivatives/option/common-definition",
        "derivatives/option/error-code",
        "derivatives/option/market-data",
        "derivatives/option/market-data/24hr-Ticker-Price-Change-Statistics",
        "derivatives/option/market-data/Exchange-Information",
        "derivatives/option/market-data/Historical-Exercise-Records",
        "derivatives/option/market-data/Open-Interest",
        "derivatives/option/market-data/Order-Book",
        "derivatives/option/market-data/Recent-Trades-List",
        "derivatives/option/market-data/Recent-Block-Trade-List",
        "derivatives/option/market-data/Symbol-Price-Ticker",
        "derivatives/option/market-data/Kline-Candlestick-Data",
        "derivatives/option/market-data/Test-Connectivity",
        "derivatives/option/market-data/Old-Trades-Lookup",
        "derivatives/option/market-data/Option-Mark-Price",
    ];

    const OUTPUT_FILE: &'static str = "binance/options/public_rest_api.md";
    const TITLE: &'static str = "Binance Options Public REST API Documentation";
}
