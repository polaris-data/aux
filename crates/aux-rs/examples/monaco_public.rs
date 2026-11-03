use aux_rs::monaco::{
    MonacoClient, OrderBookRequest, Subscription, TradesRequest, TradingPairsRequest,
};
use std::{error::Error, time::Duration};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = MonacoClient::testnet()?;
    let result = async {
        let pairs = client
            .trading_pairs(TradingPairsRequest {
                is_active: Some(true),
                page_size: Some(100),
                ..Default::default()
            })
            .await?
            .trading_pairs
            .unwrap_or_default();
        let pair = pairs
            .iter()
            .find(|p| p.symbol.as_deref() == Some("BTC/USDC"))
            .or_else(|| pairs.first())
            .ok_or("no active markets")?;
        let id = pair.id.clone().ok_or("missing pair ID")?;
        let mode = pair.market_type.clone().ok_or("missing market type")?;
        println!(
            "{} markets; selected {} ({mode})",
            pairs.len(),
            pair.symbol.as_deref().unwrap_or(&id)
        );
        let book = client
            .order_book(OrderBookRequest {
                trading_pair_id: id.clone(),
                trading_mode: Some(mode.clone()),
                levels: Some(20),
                ..Default::default()
            })
            .await?;
        println!("Best bid: {:?}", book.data.and_then(|b| b.best_bid));
        let trades = client
            .trades(TradesRequest {
                trading_pair_id: id.clone(),
                page_size: Some(5),
                ..Default::default()
            })
            .await?;
        println!("Recent trades: {}", trades.trades.unwrap_or_default().len());
        let mut stream = client
            .subscribe(Subscription::Orderbook {
                trading_pair_id: id,
                mode,
                magnitude: None,
                denomination: None,
            })
            .await?;
        let snapshot = tokio::time::timeout(Duration::from_secs(15), async {
            while let Some(event) = stream.next().await {
                let event = event?;
                if event.kind == "Snapshot" {
                    return Ok::<_, aux_rs::Error<aux_rs::monaco::Error>>(Some(event));
                }
            }
            Ok(None)
        })
        .await??;
        println!("Orderbook snapshot: {}", snapshot.is_some());
        stream.close().await?;
        Ok::<_, Box<dyn Error>>(())
    }
    .await;
    client.transport().shutdown().await?;
    result
}
