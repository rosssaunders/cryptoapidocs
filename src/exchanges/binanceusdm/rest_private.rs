use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::binancecommon::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binanceusdm_private_rest")]
#[exchange("binanceusdm")]
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
        "derivatives/usds-margined-futures/general-info",
        "derivatives/usds-margined-futures/common-definition",
        "derivatives/usds-margined-futures/faq/stp-faq",
        "derivatives/usds-margined-futures/error-code",

        "derivatives/usds-margined-futures/trade/rest-api",
        "derivatives/usds-margined-futures/trade/rest-api/Place-Multiple-Orders",
        "derivatives/usds-margined-futures/trade/rest-api/Modify-Order",
        "derivatives/usds-margined-futures/trade/rest-api/Modify-Multiple-Orders",
        "derivatives/usds-margined-futures/trade/rest-api/Get-Order-Modify-History",
        "derivatives/usds-margined-futures/trade/rest-api/Cancel-Order",
        "derivatives/usds-margined-futures/trade/rest-api/Cancel-Multiple-Orders",
        "derivatives/usds-margined-futures/trade/rest-api/Cancel-All-Open-Orders",
        "derivatives/usds-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders",
        "derivatives/usds-margined-futures/trade/rest-api/Query-Order",
        "derivatives/usds-margined-futures/trade/rest-api/All-Orders",
        "derivatives/usds-margined-futures/trade/rest-api/Current-All-Open-Orders",
        "derivatives/usds-margined-futures/trade/rest-api/Query-Current-Open-Order",
        "derivatives/usds-margined-futures/trade/rest-api/Users-Force-Orders",
        "derivatives/usds-margined-futures/trade/rest-api/Account-Trade-List",
        "derivatives/usds-margined-futures/trade/rest-api/Change-Margin-Type",
        "derivatives/usds-margined-futures/trade/rest-api/Change-Position-Mode",
        "derivatives/usds-margined-futures/trade/rest-api/Change-Initial-Leverage",
        "derivatives/usds-margined-futures/trade/rest-api/Change-Multi-Assets-Mode",
        "derivatives/usds-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin",
        "derivatives/usds-margined-futures/trade/rest-api/Position-Information-V2",
        "derivatives/usds-margined-futures/trade/rest-api/Position-Information-V3",
        "derivatives/usds-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation",
        "derivatives/usds-margined-futures/trade/rest-api/Get-Position-Margin-Change-History",
        "derivatives/usds-margined-futures/trade/rest-api/New-Order-Test",

        "derivatives/usds-margined-futures/account/rest-api",
        "wallet/asset/user-universal-transfer",
        "derivatives/usds-margined-futures/account/rest-api/Futures-Account-Balance-V3",
        "derivatives/usds-margined-futures/account/rest-api/Futures-Account-Balance-V2",
        "derivatives/usds-margined-futures/account/rest-api/Account-Information-V3",
        "derivatives/usds-margined-futures/account/rest-api/Account-Information-V2",
        "derivatives/usds-margined-futures/account/rest-api/Get-Future-Account-Transaction-History-List",
        "derivatives/usds-margined-futures/account/rest-api/User-Commission-Rate",
        "derivatives/usds-margined-futures/account/rest-api/Account-Config",
        "derivatives/usds-margined-futures/account/rest-api/Symbol-Config",
        "derivatives/usds-margined-futures/account/rest-api/Query-Rate-Limit",
        "derivatives/usds-margined-futures/account/rest-api/Notional-and-Leverage-Brackets",
        "derivatives/usds-margined-futures/account/rest-api/Get-Current-Multi-Assets-Mode",
        "derivatives/usds-margined-futures/account/rest-api/Get-Current-Position-Mode",
        "derivatives/usds-margined-futures/account/rest-api/Get-Income-History",
        "derivatives/usds-margined-futures/account/rest-api/Futures-Trading-Quantitative-Rules-Indicators",
        "derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History",
        "derivatives/usds-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id",
        "derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History",
        "derivatives/usds-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id",
        "derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History",
        "derivatives/usds-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id",
        "derivatives/usds-margined-futures/account/rest-api/Toggle-BNB-Burn-On-Futures-Trade",
        "derivatives/usds-margined-futures/account/rest-api/Get-BNB-Burn-Status",
        
        "derivatives/usds-margined-futures/convert",
        "derivatives/usds-margined-futures/convert/Send-quote-request",
        "derivatives/usds-margined-futures/convert/Accept-Quote",
        "derivatives/usds-margined-futures/convert/Order-Status",

        "derivatives/usds-margined-futures/portfolio-margin-endpoints",
    ];

    const OUTPUT_FILE: &'static str = "binance/usdm/private_rest_api.md";
    const TITLE: &'static str = "Binance USDM Private REST API Documentation";
}
