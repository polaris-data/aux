//! Read-only public testnet example. Does not authenticate or place orders.
use aux_rs::qfex::{Channel, OrderBookRequest, QfexClient, RefdataRequest, Subscription};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = QfexClient::testnet()?;
    let markets = client
        .refdata(RefdataRequest {
            ticker: Some("AAPL-USD".into()),
        })
        .await?;
    println!("{} QFEX symbols", markets.data.len());
    let book = client.order_book(OrderBookRequest::new("AAPL-USD")).await?;
    println!(
        "AAPL-USD: {} bids, {} asks",
        book.bids.len(),
        book.asks.len()
    );
    let mut stream = client
        .subscribe(Subscription::new(Channel::OrderBook, "AAPL-USD"))
        .await?;
    tokio::time::timeout(std::time::Duration::from_secs(15), async {
        while let Some(event) = stream.next().await {
            let event = event?;
            if !matches!(event.kind.as_str(), "connected" | "subscribed") {
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
