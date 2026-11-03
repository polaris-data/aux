use aux_rs::derive::{DeriveClient, GetInstrumentRequest, OrderbookPayloadDoc, Subscription};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DeriveClient::testnet()?;
    let time = client.public_get_time(Default::default()).await?;
    let market = client
        .public_get_instrument(GetInstrumentRequest {
            instrument_name: "ETH-PERP".into(),
            ..Default::default()
        })
        .await?;
    println!(
        "Derive testnet: {time}; {} tick {}",
        market.instrument_name, market.tick_size
    );
    let mut stream = client
        .subscribe(Subscription::Orderbook {
            instrument_name: market.instrument_name,
            group: 10,
            depth: 20,
        })
        .await?;
    tokio::time::timeout(std::time::Duration::from_secs(15), async {
        while let Some(event) = stream.next().await {
            let event = event?;
            if event.channel().is_some() {
                let book: OrderbookPayloadDoc = event.payload().map_err(aux_rs::Error::Exchange)?;
                println!("{} bids, {} asks", book.bids.len(), book.asks.len());
                return Ok::<_, aux_rs::Error<aux_rs::derive::Error>>(());
            }
        }
        Err(aux_rs::Error::Exchange(
            aux_rs::derive::Error::InvalidRequest("stream ended before a book snapshot"),
        ))
    })
    .await??;
    stream.close().await?;
    client.transport().shutdown().await?;
    Ok(())
}
