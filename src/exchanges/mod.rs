pub mod binancespot;
pub mod binancecommon;
pub mod binancederivatives;

use std::error::Error;
use binancespot::{
    binance_spot_rest::BinanceSpotRest,
    binance_spot_fix::BinanceSpotFix,
    binance_spot_websocket::BinanceSpotWebSocket,
    binance_spot_sbe::BinanceSpotSbe,
};
use binancederivatives::{
    coinm_rest_private::PrivateRest,
    coinm_rest_public::PublicREST,
    binance_derivatives_usdm::BinanceDerivativesUSDM,
};

pub trait ApiProcessor {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>>;
    fn get_output_filename(&self) -> String;
}

pub enum ApiProcessorType {
    SpotRest(BinanceSpotRest),
    SpotFix(BinanceSpotFix),
    SpotWebSocket(BinanceSpotWebSocket),
    SpotSbe(BinanceSpotSbe),
    DerivativesUSDM(BinanceDerivativesUSDM),
    DerivativesCOINMPrivateRest(PrivateRest),
    DerivativesCOINMPublicREST(PublicREST),
}

impl Default for ApiProcessorType {
    fn default() -> Self {
        Self::SpotRest(BinanceSpotRest::default())
    }
}

impl ApiProcessor for ApiProcessorType {
    async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        match self {
            Self::SpotRest(p) => p.process_docs().await,
            Self::SpotFix(p) => p.process_docs().await,
            Self::SpotWebSocket(p) => p.process_docs().await,
            Self::SpotSbe(p) => p.process_docs().await,
            Self::DerivativesUSDM(p) => p.process_docs().await,
            Self::DerivativesCOINMPrivateRest(p) => p.process_docs().await,
            Self::DerivativesCOINMPublicREST(p) => p.process_docs().await,
        }
    }

    fn get_output_filename(&self) -> String {
        match self {
            Self::SpotRest(p) => p.get_output_filename(),
            Self::SpotFix(p) => p.get_output_filename(),
            Self::SpotWebSocket(p) => p.get_output_filename(),
            Self::SpotSbe(p) => p.get_output_filename(),
            Self::DerivativesUSDM(p) => p.get_output_filename(),
            Self::DerivativesCOINMPrivateRest(p) => p.get_output_filename(),
            Self::DerivativesCOINMPublicREST(p) => p.get_output_filename(),
        }
    }
}
