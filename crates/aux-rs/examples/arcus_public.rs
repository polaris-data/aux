//! Read-only public testnet example. Does not authenticate or place orders.
use aux_rs::arcus::{ArcusClient, Channel, MarketsRequest, OrderBookRequest, Subscription};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ArcusClient::testnet()?;
    let markets = client
        .markets(MarketsRequest {
            market: Some("BTC-USD".into()),
        })
        .await?;
    println!("{} Arcus markets", markets.markets.len());
    let book = client.order_book(OrderBookRequest::new("BTC-USD")).await?;
    println!(
        "BTC-USD: {} bids, {} asks",
        book.bids.len(),
        book.asks.len()
    );
    let mut stream = client
        .subscribe(Subscription::market(Channel::OrderBook, "BTC-USD"))
        .await?;
    tokio::time::timeout(std::time::Duration::from_secs(15), async {
        while let Some(event) = stream.next().await {
            let event = event?;
            if event.kind != "connected" {
                println!("Native {} event", event.kind);
                return Ok::<_, Box<dyn std::error::Error>>(());
            }
        }
        Err(std::io::Error::other("stream ended before market data").into())
    })
    .await??;
    stream.close().await?;
    client.transport().shutdown().await?;
    Ok(())
}
