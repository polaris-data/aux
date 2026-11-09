//! Public Phoenix reads and one native book frame; no credentials or writes.
use aux_rs::phoenix::{PhoenixClient, Subscription};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PhoenixClient::mainnet()?;
    let _markets = client.list_markets(Default::default()).await?;
    println!("Phoenix market metadata received");
    let symbol = std::env::var("AUX_PHOENIX_SYMBOL").unwrap_or_else(|_| "SOL".into());
    let mut stream = client
        .subscribe(Subscription(
            serde_json::json!({"channel":"orderbook","symbol":symbol}),
        ))
        .await?;
    let event = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let event = stream.next().await.ok_or("stream closed")??;
            if event.0["channel"] == "orderbook" {
                return Ok::<_, Box<dyn std::error::Error>>(event);
            }
        }
    })
    .await??;
    println!(
        "Phoenix native stream frame: {}",
        event
            .0
            .get("channel")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("control")
    );
    stream.close().await?;
    client.transport().shutdown().await?;
    Ok(())
}
