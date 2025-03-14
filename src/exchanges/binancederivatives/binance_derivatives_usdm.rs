use std::error::Error;
use crate::exchanges::{ApiProcessor, binancecommon::BinanceApiProcessor};

#[derive(Default)]
pub struct BinanceDerivativesUSDM;

impl ApiProcessor for BinanceDerivativesUSDM {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        BinanceApiProcessor::process_docs(self).await
    }

    fn get_output_filename(&self) -> String {
        BinanceApiProcessor::get_output_filename(self)
    }
}

impl BinanceApiProcessor for BinanceDerivativesUSDM {
    const ENDPOINTS: &'static [&'static str] = &[
        "derivatives/quick-start",
        "derivatives/usds-margined-futures/general-info",
        "derivatives/usds-margined-futures/websocket-api-general-info",
        "derivatives/usds-margined-futures/common-definition",

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
        "derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents",

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
        "derivatives/usds-margined-futures/faq/stp-faq",
        "derivatives/usds-margined-futures/error-code"
    ];

    const OUTPUT_FILE: &'static str = "binance/derivatives/binance_deriviatves_usdm_api_docs.md";
    const TITLE: &'static str = "Binance Deriviates USDM REST API Documentation";
}
