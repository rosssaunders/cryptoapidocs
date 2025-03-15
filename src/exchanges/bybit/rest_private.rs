use std::error::Error;
use async_trait::async_trait;
use crate::exchanges::ApiProcessor;
use super::doc_processor::DocProcessor;
use cryptoapidocs_macros::ProcessorRegistration;

#[derive(Default, ProcessorRegistration)]
#[processor("bybit_private_rest")]
#[exchange("bybit")]
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
        "rate-limit",
        "enum",
        "error",
        "order/create-order",
        "order/amend-order",
        "order/cancel-order",
        "order/open-order",        
        "order/cancel-all",
        "order/order-list",
        "order/execution",
        "order/batch-place",
        "order/batch-amend",
        "order/batch-cancel",
        "order/spot-borrow-quota",
        "order/dcp",
        "position",
        "position/leverage",
        "position/cross-isolate",
        "position/position-mode",
        "position/trading-stop",
        "position/auto-add-margin",
        "position/manual-add-margin",
        "position/close-pnl",
        "position/move-position",
        "position/move-position-history",
        "position/confirm-mmr",
        "position/tpsl-mode",
        "position/set-risk-limit",
        "account/wallet-balance",
        "account/unified-trans-amnt",
        "account/upgrade-unified-account",
        "account/borrow-history",
        "account/repay-liability",
        "account/set-collateral",
        "account/batch-set-collateral",
        "account/collateral-info",
        "account/coin-greeks",
        "account/fee-rate",
        "account/account-info",
        "account/dcp-info",
        "account/transaction-log",
        "account/contract-transaction-log",
        "account/smp-group",
        "account/set-margin-mode",
        "account/set-spot-hedge",
        "account/set-mmp",
        "account/reset-mmp",
        "account/get-mmp-state",
        "asset/delivery",
        "asset/settlement",
        "asset/exchange",
        "asset/coin-info",
        "asset/sub-uid-list",
        "asset/balance/asset-info",
        "asset/balance/all-balance",
        "asset/balance/account-coin-balance",
        "asset/balance/delay-amount",
        "asset/transfer/transferable-coin",
        "asset/transfer/create-inter-transfer",
        "asset/transfer/inter-transfer-list",
        "asset/transfer/unitransfer",
        "asset/transfer/unitransfer-list",
        "asset/deposit/set-deposit-acct",
        "asset/deposit/deposit-record",
        "asset/deposit/sub-deposit-record",
        "asset/deposit/internal-deposit-record",
        "asset/deposit/master-deposit-addr",
        "asset/deposit/sub-deposit-addr",
        "asset/withdraw/withdraw-record",
        "asset/withdraw/vasp-list",
        "asset/withdraw",
        "asset/withdraw/cancel-withdraw",
        "asset/convert/guideline",
        "asset/convert/convert-coin-list",
        "asset/convert/apply-quote",
        "asset/convert/confirm-quote",
        "asset/convert/get-convert-result",
        "asset/convert/get-convert-history",
        "user/create-subuid",
        "user/create-subuid-apikey",
        "user/subuid-list",
        "user/page-subuid",
        "user/fund-subuid-list",
        "user/froze-subuid",
        "user/apikey-info",
        "user/list-sub-apikeys",
        "user/wallet-type",
        "user/modify-master-apikey",
        "user/modify-sub-apikey",
        "user/rm-subuid",
        "user/rm-master-apikey",
        "user/rm-sub-apikey",
        "spot-margin-uta/historical-interest",
        "spot-margin-uta/switch-mode",
        "spot-margin-uta/set-leverage",
        "spot-margin-uta/status",
        "crypto-loan/acct-borrow-collateral",
        "crypto-loan/borrow",
        "crypto-loan/repay",
        "crypto-loan/unpaid-loan-order",
        "crypto-loan/repay-transaction",
        "crypto-loan/completed-loan-order",
        "crypto-loan/reduce-max-collateral-amt",
        "crypto-loan/adjust-collateral",
        "crypto-loan/ltv-adjust-history",
        "otc/margin-product-info",
        "otc/margin-coin-convert-info",
        "otc/loan-info",
        "otc/repay-info",
        "otc/ltv-convert",
        "otc/bind-uid",
        "broker/exchange-earning",
        "broker/account-info",
        "broker/sub-deposit-record",
        "broker/reward/voucher",
        "broker/reward/issue-voucher",
        "broker/reward/get-issue-voucher",
        "earn/create-order",
        "earn/order-history",
        "earn/position",
    ];

    const OUTPUT_FILE: &'static str = "bybit/v5/private_rest_api.md";
    const TITLE: &'static str = "ByBit V5 Private REST API Documentation";
}
