//! The only crate that depends on Python. Protocol adaptation stays in Rust's
//! existing client/adapter crates; owned concrete wrappers cross this boundary.

mod arcus;
mod client;
mod convert;
mod errors;
mod lighter;
mod qfex;
mod signers;
mod state;
mod subscription;

use pyo3::prelude::*;

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<client::HyperliquidClient>()?;
    module.add_class::<qfex::QfexClient>()?;
    module.add_class::<arcus::ArcusClient>()?;
    module.add_class::<arcus::ArcusSigner>()?;
    module.add_class::<lighter::LighterClient>()?;
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
    errors::add_to_module(module)?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
