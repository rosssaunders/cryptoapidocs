use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binanceoptions_private_rest")]
#[exchange("binanceoptions")]
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
        "derivatives/quick-start",
        "derivatives/option/general-info",
        "derivatives/option/common-definition",
        "derivatives/option/error-code",
        "derivatives/option/account",
        "derivatives/option/account/Account-Funding-Flow",
        "derivatives/option/account/Funds-Transfer",
        "derivatives/option/account/Get-Download-Id-For-Option-Transaction-History",
        "derivatives/option/account/Get-Option-Transaction-History-Download-Link-by-Id",
        "derivatives/option/trade",
        "derivatives/option/trade/Place-Multiple-Orders",
        "derivatives/option/trade/Cancel-Option-Order",
        "derivatives/option/trade/Cancel-Multiple-Option-Orders",
        "derivatives/option/trade/Cancel-All-Option-Orders-By-Underlying",
        "derivatives/option/trade/Cancel-all-Option-orders-on-specific-symbol",
        "derivatives/option/trade/Query-Single-Order",
        "derivatives/option/trade/Query-Option-Order-History",
        "derivatives/option/trade/Query-Current-Open-Option-Orders",
        "derivatives/option/trade/Option-Position-Information",
        "derivatives/option/trade/User-Exercise-Record",
        "derivatives/option/trade/Account-Trade-List",
        "derivatives/option/market-maker-endpoints",
        "derivatives/option/market-maker-endpoints/Get-Market-Maker-Protection-Config",
        "derivatives/option/market-maker-endpoints/Get-Auto-Cancel-All-Open-Orders-Config",
        "derivatives/option/market-maker-endpoints/Set-Market-Maker-Protection-Config",
        "derivatives/option/market-maker-endpoints/Auto-Cancel-All-Open-Orders-Heartbeat",
        "derivatives/option/market-maker-endpoints/Reset-Market-Maker-Protection-Config",
        "derivatives/option/market-maker-endpoints/Set-Auto-Cancel-All-Open-Orders-Config",
        "derivatives/option/market-maker-block-trade",
        "derivatives/option/market-maker-block-trade/Cancel-Block-Trade-Order",
        "derivatives/option/market-maker-block-trade/Extend-Block-Trade-Order",
        "derivatives/option/market-maker-block-trade/Query-Block-Trade-Order",
        "derivatives/option/market-maker-block-trade/Accept-Block-Trade-Order",
        "derivatives/option/market-maker-block-trade/Query-Block-Trade-Detail",
        "derivatives/option/market-maker-block-trade/Account-Block-Trade-List"
    ];

    const OUTPUT_FILE: &'static str = "binance/options/private_rest_api.md";
    const TITLE: &'static str = "Binance Options Private REST API Documentation";
}
