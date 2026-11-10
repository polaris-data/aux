//! Public Extended testnet reads and one native book frame; no orders.
use aux_rs::extended::{ExtendedClient, Subscription};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network = std::env::var("AUX_EXTENDED_NETWORK").unwrap_or_else(|_| "testnet".into());
    let client = match network.as_str() {
        "testnet" => ExtendedClient::testnet()?,
        "mainnet" => ExtendedClient::mainnet()?,
        _ => return Err("AUX_EXTENDED_NETWORK must be mainnet or testnet".into()),
    };
    let _markets = client.get_markets(Default::default()).await?;
    println!("Extended {network} market metadata received");
    let market = std::env::var("AUX_EXTENDED_MARKET").unwrap_or_else(|_| "BTC-USD".into());
    let mut stream = client
        .subscribe(Subscription {
            channel: "orderbooks".into(),
            market: Some(market),
            params: serde_json::json!({}),
            rpc: std::env::var("AUX_EXTENDED_RPC").as_deref() == Ok("1"),
        })
        .await?;
    let event = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let event = stream.next().await.ok_or("stream closed")??;
            if matches!(
                event.0["type"].as_str(),
                Some("SNAPSHOT" | "ORDERBOOK.SNAPSHOT" | "ORDERBOOKS.SNAPSHOT")
            ) {
                return Ok::<_, Box<dyn std::error::Error>>(event);
            }
        }
    })
    .await??;
    println!(
        "Extended native stream frame: {}",
        event
            .0
            .get("type")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("control")
    );
    stream.close().await?;
    client.transport().shutdown().await?;
    Ok(())
}
