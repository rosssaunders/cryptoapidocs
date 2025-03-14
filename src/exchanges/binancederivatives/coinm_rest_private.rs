use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::{ApiProcessor, binancecommon::BinanceApiProcessor};
use crate::register_processor;

#[derive(Default)]
pub struct PrivateRest;

#[async_trait]
impl ApiProcessor for PrivateRest {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        BinanceApiProcessor::process_docs(self).await
    }

    fn get_output_filename(&self) -> String {
        BinanceApiProcessor::get_output_filename(self)
    }
}

impl BinanceApiProcessor for PrivateRest {
    const ENDPOINTS: &'static [&'static str] = &[
        "derivatives/quick-start",
        "derivatives/coin-margined-futures/general-info",
        "derivatives/coin-margined-futures/common-definition",
        
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
        "derivatives/coin-margined-futures/trade/rest-api/Position-Information",
        "derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode",
        "derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type",
        "derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage",
        "derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation",
        "derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin",
        "derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History",

        "derivatives/coin-margined-futures/account/rest-api",
        "wallet/asset/user-universal-transfer",
        "derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance",
        "derivatives/coin-margined-futures/account/rest-api/Get-Future-Account-Transaction-History-List",
        "derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate",
        "derivatives/coin-margined-futures/account/rest-api/Account-Information",
        "derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Symbol",
        "derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Pair",
        "derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode",
        "derivatives/coin-margined-futures/account/rest-api/Get-Income-History",
        "derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History",
        "derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id",
        "derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History",
        "derivatives/coin-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id",
        "derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History",
        "derivatives/coin-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id",
        "derivatives/coin-margined-futures/error-code"
    ];

    const OUTPUT_FILE: &'static str = "binance/derivatives/binance_derivatives_coinm_private_rest_api_docs.md";
    const TITLE: &'static str = "Binance Derivatives COINM Private REST API Documentation";
}

// Register this processor
register_processor!("binance_derivatives_coinm_private", PrivateRest);
