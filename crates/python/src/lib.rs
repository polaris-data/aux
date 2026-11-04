//! The only crate that depends on Python. Protocol adaptation stays in Rust's
//! existing client/adapter crates; owned concrete wrappers cross this boundary.

mod arcus;
mod client;
mod convert;
mod data;
mod derive;
mod errors;
mod kalshi;
mod lighter;
mod metric;
mod monaco;
mod paradex;
mod pascal;
mod polymarket;
mod prediction_signers;
mod qfex;
mod signers;
mod state;
mod subscription;

use pyo3::prelude::*;

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<data::TardisClient>()?;
    module.add_class::<data::TardisMachineClient>()?;
    module.add_class::<data::DatabentoHistoricalClient>()?;
    module.add_class::<data::DatabentoReferenceClient>()?;
    module.add_class::<data::DatabentoLiveClient>()?;
    module.add_class::<metric::MetricClient>()?;
    module.add_class::<metric::MetricChainClient>()?;
    module.add_class::<metric::MetricSigner>()?;
    module.add_function(wrap_pyfunction!(metric::metric_tool, module)?)?;
    module.add_class::<derive::DeriveClient>()?;
    module.add_class::<derive::DeriveSigner>()?;
    module.add_class::<derive::DeriveRpcSession>()?;
    module.add_class::<paradex::ParadexClient>()?;
    module.add_class::<paradex::ParadexSigner>()?;
    module.add_class::<paradex::ParadexRpcSession>()?;
    module.add_class::<client::HyperliquidClient>()?;
    module.add_class::<qfex::QfexClient>()?;
    module.add_class::<arcus::ArcusClient>()?;
    module.add_class::<arcus::ArcusSigner>()?;
    module.add_class::<lighter::LighterClient>()?;
    module.add_class::<monaco::MonacoClient>()?;
    module.add_class::<monaco::MonacoSessionSigner>()?;
    module.add_class::<monaco::MonacoWalletSigner>()?;
    module.add_class::<subscription::Subscription>()?;
    module.add(
        "BookSubscription",
        module.py().get_type::<subscription::Subscription>(),
    )?;
    module.add_class::<signers::HyperliquidSigner>()?;
    module.add_class::<signers::LighterSigner>()?;
    module.add_class::<signers::HyperliquidSigningPayload>()?;
    module.add_class::<signers::LighterSigningPayload>()?;
    module.add_function(wrap_pyfunction!(signers::hyperliquid_action_hash, module)?)?;
    module.add_function(wrap_pyfunction!(
        signers::validate_hyperliquid_order,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(signers::scale_lighter_decimal, module)?)?;
    module.add_function(wrap_pyfunction!(
        signers::hyperliquid_signing_payload,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        signers::hyperliquid_user_signing_payload,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        signers::lighter_auth_signing_payload,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        signers::lighter_transaction_signing_payload,
        module
    )?)?;
    module.add_class::<kalshi::KalshiClient>()?;
    module.add_class::<kalshi::KalshiPerpsClient>()?;
    module.add_class::<polymarket::PolymarketClient>()?;
    module.add_class::<polymarket::PolymarketPerpsClient>()?;
    module.add_class::<pascal::PascalClient>()?;
    module.add_class::<prediction_signers::KalshiSigner>()?;
    module.add_class::<prediction_signers::PolymarketSigner>()?;
    module.add_class::<prediction_signers::PascalSigner>()?;
    module.add_class::<polymarket::PolymarketChainClient>()?;
    module.add_class::<kalshi::KalshiRpcSession>()?;
    module.add_class::<pascal::PascalRpcSession>()?;
    module.add_class::<polymarket::PolymarketRpcSession>()?;
    module.add_class::<pascal::PascalChainClient>()?;
    module.add_function(wrap_pyfunction!(
        polymarket::polymarket_encode_call,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(polymarket::polymarket_helper, module)?)?;
    errors::add_to_module(module)?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
