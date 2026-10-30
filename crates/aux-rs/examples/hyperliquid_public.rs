use aux_rs::hyperliquid::HyperliquidClient;
use aux_rs::hyperliquid::{Event, L2BookRequest, Subscription};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HyperliquidClient::testnet()?;
    let meta = client.meta().await?;
    println!("Testnet perpetual markets: {}", meta.universe.len());
    let book = client.l2_book(L2BookRequest::new("ETH")).await?;
    println!(
        "ETH bids: {}, asks: {}",
        book.levels[0].len(),
        book.levels[1].len()
    );

    let mut subscription = client
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await?;
    let mut trades = client
        .subscribe(Subscription::Trades { coin: "ETH".into() })
        .await?;
    tokio::time::timeout(std::time::Duration::from_secs(30), async {
        while let Some(event) = subscription.next().await {
            if let Event::L2Book(book) = event? {
                println!("ETH snapshot at {}", book.time);
                while let Some(event) = trades.next().await {
                    if let Event::Trades(trades) = event? {
                        println!("ETH trades received: {}", trades.len());
                        return Ok::<_, Box<dyn std::error::Error>>(());
                    }
                }
                return Err(std::io::Error::other("trade stream ended before data").into());
            }
        }
        Err(std::io::Error::other("book stream ended before data").into())
    })
    .await??;
    subscription.unsubscribe().await?;
    trades.unsubscribe().await?;
    Ok(())
}
