use aux_rs::paradex::*;
use zeroize::Zeroizing;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !std::env::args().any(|v| v == "--execute") {
        println!(
            "Use --execute with PARADEX_PRIVATE_KEY, PARADEX_ACCOUNT, PARADEX_PRICE and PARADEX_SIZE for a testnet submit/cancel. PARADEX_SUBKEY is optional."
        );
        return Ok(());
    }
    let client = ParadexClient::testnet()?;
    let signer = LocalStarkSigner::from_hex(Zeroizing::new(std::env::var("PARADEX_PRIVATE_KEY")?))?;
    let auth = ParadexAuth::new(
        client.exchange(),
        &std::env::var("PARADEX_ACCOUNT")?,
        signer,
        std::env::var("PARADEX_SUBKEY").ok(),
    )?;
    let client = client.with_auth(auth);
    client.authenticate().await?;
    let body = RequestsOrderRequest {
        market: "ETH-USD-PERP".into(),
        instruction: "POST_ONLY".into(),
        side: "BUY".into(),
        type_: "LIMIT".into(),
        size: std::env::var("PARADEX_SIZE")?,
        price: std::env::var("PARADEX_PRICE")?,
        ..Default::default()
    };
    let response = client
        .orders_new(OrdersNewParams {
            body,
            ..Default::default()
        })
        .await?;
    let response = serde_json::to_value(response)?;
    let id = response["id"]
        .as_str()
        .ok_or("native response has no order id; reconcile before another submission")?
        .to_owned();
    println!("Testnet order: {id}");
    let response = client
        .orders_cancel(OrdersCancelParams {
            order_id: id,
            ..Default::default()
        })
        .await?;
    println!(
        "Native cancellation response: {}",
        serde_json::to_value(response)?
    );
    client.transport().shutdown().await?;
    Ok(())
}
