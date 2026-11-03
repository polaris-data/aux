use aux_rs::paradex::{ParadexClient, Subscription};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ParadexClient::testnet()?;
    let configuration = client.get_system_config(Default::default()).await?;
    println!(
        "Paradex testnet chain: {}",
        serde_json::to_value(configuration)?["starknet_chain_id"]
    );
    let mut stream = client
        .subscribe(Subscription::OrderBook {
            market: "ETH-USD-PERP".into(),
            feed_type: "snapshot".into(),
            refresh_rate: "100ms".into(),
            price_tick: None,
        })
        .await?;
    tokio::time::timeout(std::time::Duration::from_secs(15), async {
        while let Some(event) = stream.next().await {
            let event = event?;
            if let Some(frame) = event.sbe_frame().map_err(aux_rs::Error::Exchange)?
                && let aux_rs::paradex::sbe::SbeMessage::BookEvent(book) = frame.message
            {
                println!(
                    "{} bids, {} asks (SBE 1:3)",
                    book.bids.len(),
                    book.asks.len()
                );
                return Ok::<_, aux_rs::Error<aux_rs::paradex::Error>>(());
            }
        }
        Err(aux_rs::Error::Exchange(
            aux_rs::paradex::Error::InvalidRequest("stream ended before an SBE book snapshot"),
        ))
    })
    .await??;
    stream.close().await?;
    client.transport().shutdown().await?;
    Ok(())
}
