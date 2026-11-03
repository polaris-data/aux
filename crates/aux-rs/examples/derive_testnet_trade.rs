use aux_rs::derive::*;
use zeroize::Zeroizing;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !std::env::args().any(|v| v == "--execute") {
        println!(
            "Use --execute with DERIVE_PRIVATE_KEY, DERIVE_WALLET, DERIVE_SUBACCOUNT_ID, DERIVE_PRICE and DERIVE_SIZE for a testnet submit/cancel."
        );
        return Ok(());
    }
    let client = DeriveClient::testnet()?;
    let signer = LocalSigner::from_hex(Zeroizing::new(std::env::var("DERIVE_PRIVATE_KEY")?))?;
    let auth = DeriveAuth::new(client.exchange(), &std::env::var("DERIVE_WALLET")?, signer)?;
    let client = client.with_auth(auth);
    let subaccount = std::env::var("DERIVE_SUBACCOUNT_ID")?.parse::<u64>()?;
    let request = CreateOrderRequest {
        subaccount_id: subaccount.into(),
        instrument_name: "ETH-PERP".into(),
        direction: "buy".into(),
        amount: std::env::var("DERIVE_SIZE")?,
        limit_price: std::env::var("DERIVE_PRICE")?,
        max_fee: "1".into(),
        order_type: Some("limit".into()),
        time_in_force: Some("post_only".into()),
        ..Default::default()
    };
    let response = client.private_order(request).await?;
    let id = response.order.order_id;
    println!("Testnet order: {id}");
    let cancelled = client
        .private_cancel(CancelOrderRequest {
            subaccount_id: subaccount.into(),
            instrument_name: "ETH-PERP".into(),
            order_id: id,
            ..Default::default()
        })
        .await?;
    println!(
        "Native cancellation response: {}",
        serde_json::to_value(cancelled)?["order_status"]
    );
    client.transport().shutdown().await?;
    Ok(())
}
