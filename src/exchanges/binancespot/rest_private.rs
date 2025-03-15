use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binancespot_private_rest")]
#[exchange("binancespot")]
pub struct PrivateREST;

#[async_trait]
impl ApiProcessor for PrivateREST {
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

impl PrivateREST {
    const ENDPOINTS: &'static [&'static str] = &[
        "binance-spot-api-docs/filters",
        "binance-spot-api-docs/enums",
        "binance-spot-api-docs/errors",
        "binance-spot-api-docs/rest-api/general-api-information",
        "binance-spot-api-docs/rest-api/http-return-codes",
        "binance-spot-api-docs/rest-api/error-codes",
        "binance-spot-api-docs/rest-api/general-information-on-endpoints",
        "binance-spot-api-docs/rest-api/limits",
        "binance-spot-api-docs/rest-api/data-sources",
        "binance-spot-api-docs/rest-api/endpoint-security-type",
        "binance-spot-api-docs/rest-api/general-endpoints",
        "binance-spot-api-docs/rest-api/trading-endpoints",
        "binance-spot-api-docs/rest-api/account-endpoints",
        "binance-spot-api-docs/user-data-stream",
        "binance-spot-api-docs/rest-api/user-data-stream-endpoints",
    ];

    const OUTPUT_FILE: &'static str = "binance/spot/private_rest_api.md";
    const TITLE: &'static str = "Binance Spot Private REST API Documentation";
}