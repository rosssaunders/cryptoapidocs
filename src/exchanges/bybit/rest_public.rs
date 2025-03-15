use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use super::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("bybit_public_rest")]
#[exchange("bybit")]
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
        "rate-limit",
        "enum",
        "error",
        "market/time",
        "market/kline",
        "market/mark-kline",
        "market/index-kline",
        "market/premium-index-kline",
        "market/instrument",
        "market/orderbook",
        "market/tickers",
        "market/history-fund-rate",
        "market/recent-trade",
        "market/open-interest",
        "market/iv",
        "market/insurance",
        "market/risk-limit",
        "market/delivery-price",
        "market/long-short-ratio",
        "spot-margin-uta/vip-margin",
        "spot-margin-uta/tier-collateral-ratio",
        "crypto-loan/collateral-coin",
        "crypto-loan/loan-coin",
        "otc/margin-product-info",
        "otc/margin-coin-convert-info",
    ];

    const OUTPUT_FILE: &'static str = "bybit/v5/public_rest_api.md";
    const TITLE: &'static str = "ByBit V5 Public REST API Documentation";
}
