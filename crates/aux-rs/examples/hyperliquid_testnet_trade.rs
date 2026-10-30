//! Explicit, opt-in testnet submit/cancel smoke test. Requires an already
//! authorized/funded testnet account. This example never selects mainnet.
use aux_rs::hyperliquid::HyperliquidClient;
use aux_rs::hyperliquid::{
    CancelByCloid, CancelByCloidRequest, Cloid, ExchangeResponse, ExecutionOptions,
    HyperliquidAuth, ItemStatus, LocalSigner, Order, OrderRequest, Tif,
};
use zeroize::Zeroizing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() != Some("--execute") {
        return Err("Pass --execute with HL_AGENT_PRIVATE_KEY, HL_ACCOUNT_ADDRESS, HL_TESTNET_ORDER_PRICE, and HL_TESTNET_ORDER_SIZE to submit/cancel a testnet post-only order".into());
    }
    let secret = Zeroizing::new(std::env::var("HL_AGENT_PRIVATE_KEY")?);
    let signer: LocalSigner = secret.as_str().parse()?;
    let mut auth = HyperliquidAuth::agent(signer, std::env::var("HL_ACCOUNT_ADDRESS")?.parse()?);
    if let Ok(vault) = std::env::var("HL_VAULT_ADDRESS") {
        auth = auth.with_vault(vault.parse()?);
    }
    let client = HyperliquidClient::testnet()?.with_auth(auth);
    let coin = std::env::var("HL_TESTNET_COIN").unwrap_or_else(|_| "ETH".into());
    let meta = client.meta().await?;
    let asset = meta
        .universe
        .iter()
        .position(|asset| asset.name == coin)
        .ok_or("coin not in the default testnet perp DEX")?;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
    let cloid = Cloid::from_bytes(now.as_nanos().to_be_bytes());
    let mut order = Order::limit(
        asset.try_into()?,
        true,
        std::env::var("HL_TESTNET_ORDER_PRICE")?,
        std::env::var("HL_TESTNET_ORDER_SIZE")?,
        Tif::Alo,
    );
    order.cloid = Some(cloid);
    order.validate_precision(meta.universe[asset].sz_decimals, false)?;
    let placed = client
        .order_with_options(
            OrderRequest::new(vec![order]),
            ExecutionOptions {
                nonce: None,
                expires_after: Some(u64::try_from(now.as_millis())? + 30_000),
            },
        )
        .await;
    // Attempt cancellation by the known client ID even if submission's outcome
    // is unknown. Never replay the order. Callers must reconcile unknown results.
    let cancelled = client
        .cancel_by_cloid(CancelByCloidRequest {
            cancels: vec![CancelByCloid {
                asset: asset.try_into()?,
                cloid,
            }],
        })
        .await;
    println!("Order result: {placed:?}");
    println!("Cancel result: {cancelled:?}");
    let ExchangeResponse::Ok(placed) = placed? else {
        return Err("exchange rejected the order".into());
    };
    if !matches!(
        placed.data.statuses.as_slice(),
        [ItemStatus::Resting { .. }]
    ) {
        return Err("expected a resting post-only testnet order; inspect native statuses".into());
    }
    let ExchangeResponse::Ok(cancelled) = cancelled? else {
        return Err("exchange rejected cancellation".into());
    };
    if !matches!(cancelled.data.statuses.as_slice(), [ItemStatus::Text(status)] if status == "success")
    {
        return Err("cancellation did not report success; reconcile the native outcome".into());
    }
    Ok(())
}
