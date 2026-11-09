//! Opt-in testnet submission/cancel. Every order, fee, nonce and asset context is caller supplied.
use aux_rs::extended::*;
use zeroize::Zeroizing;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("AUX_EXTENDED_TESTNET_TRADE").as_deref() != Ok("1") {
        return Err("set AUX_EXTENDED_TESTNET_TRADE=1 explicitly".into());
    }
    let client = ExtendedClient::testnet()?.with_auth(Credentials::new(
        Some(Zeroizing::new(std::env::var("AUX_EXTENDED_API_KEY")?)),
        None,
        None,
    )?);
    let signer =
        LocalStarkSigner::from_hex(Zeroizing::new(std::env::var("AUX_EXTENDED_STARK_KEY")?))?;
    let order: serde_json::Value = serde_json::from_str(&std::env::var("AUX_EXTENDED_ORDER")?)?;
    let context: OrderContext =
        serde_json::from_str(&std::env::var("AUX_EXTENDED_ORDER_CONTEXT")?)?;
    let signed = sign_order(client.exchange(), &signer, order, &context).await?;
    let request: PlaceOrderParams = serde_json::from_value(serde_json::json!({"body":signed}))?;
    let response = serde_json::to_value(client.place_order(request).await?)?;
    let id = response["data"]["id"]
        .as_str()
        .map(str::to_owned)
        .unwrap_or_else(|| response["data"]["id"].to_string());
    if id == "null" {
        return Err("submission response omitted native order ID; reconcile externally".into());
    }
    println!("Native REST acceptance received for order {id}; execution requires reconciliation");
    let cancel: CancelOrderParams = serde_json::from_value(serde_json::json!({"order_id":id}))?;
    let _response = client.cancel_order(cancel).await?;
    println!(
        "Cancellation response received; reconcile final native status through account updates/history"
    );
    client.transport().shutdown().await?;
    Ok(())
}
