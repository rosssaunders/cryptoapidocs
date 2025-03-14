use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use crate::exchanges::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("binance_derivatives_coinm_private")]
pub struct CoinMRestPrivate;

#[async_trait]
impl ApiProcessor for CoinMRestPrivate {
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

impl CoinMRestPrivate {
    const ENDPOINTS: &'static [&'static str] = &[
        "derivatives/coin-margined-futures/trade/rest-api",
        "derivatives/coin-margined-futures/trade/rest-api/Place-Multiple-Orders",
        "derivatives/coin-margined-futures/trade/rest-api/Modify-Order",
        "derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders",
        "derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History",
        "derivatives/coin-margined-futures/trade/rest-api/Cancel-Order",
        "derivatives/coin-margined-futures/trade/rest-api/Cancel-Multiple-Orders",
        "derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders",
        "derivatives/coin-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders",
        "derivatives/coin-margined-futures/trade/rest-api/Query-Order",
        "derivatives/coin-margined-futures/trade/rest-api/All-Orders",
        "derivatives/coin-margined-futures/trade/rest-api/Current-All-Open-Orders",
        "derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order",
        "derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders",
        "derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List",
        "derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type",
        "derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode",
        "derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage",
        "derivatives/coin-margined-futures/trade/rest-api/Change-Multi-Assets-Mode",
        "derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin",
        "derivatives/coin-margined-futures/trade/rest-api/Position-Information-V2",
        "derivatives/coin-margined-futures/trade/rest-api/Position-Information-V3",
        "derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation",
        "derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History",
        "derivatives/coin-margined-futures/trade/rest-api/New-Order-Test",

        "derivatives/coin-margined-futures/account/rest-api",
        "wallet/asset/user-universal-transfer",
        "derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance-V3",
        "derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance-V2",
        "derivatives/coin-margined-futures/account/rest-api/Account-Information-V3",
        "derivatives/coin-margined-futures/account/rest-api/Account-Information-V2",
        "derivatives/coin-margined-futures/account/rest-api/Get-Future-Account-Transaction-History-List",
        "derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate",
        "derivatives/coin-margined-futures/account/rest-api/Account-Config",
        "derivatives/coin-margined-futures/account/rest-api/Symbol-Config",
        "derivatives/coin-margined-futures/account/rest-api/Query-Rate-Limit",
        "derivatives/coin-margined-futures/account/rest-api/Notional-and-Leverage-Brackets",
        "derivatives/coin-margined-futures/account/rest-api/Get-Current-Multi-Assets-Mode",
        "derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode",
        "derivatives/coin-margined-futures/account/rest-api/Get-Income-History",
        "derivatives/coin-margined-futures/account/rest-api/Futures-Trading-Quantitative-Rules-Indicators",
        "derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History",
        "derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id",
        "derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History",
        "derivatives/coin-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id",
        "derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History",
        "derivatives/coin-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id",
        "derivatives/coin-margined-futures/account/rest-api/Toggle-BNB-Burn-On-Futures-Trade",
        "derivatives/coin-margined-futures/account/rest-api/Get-BNB-Burn-Status",
    ];

    const OUTPUT_FILE: &'static str = "binance/derivatives/binance_derivatives_coinm_private_api_docs.md";
    const TITLE: &'static str = "Binance Derivatives COIN-M Private REST API Documentation";
}
