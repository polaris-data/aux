//! Read-only testnet smoke: HTTP metadata/book/trades/account and WS snapshots.
use aux_rs::lighter::LighterClient;
use aux_rs::lighter::*;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LighterClient::testnet()?;
    let markets = client
        .order_books(OrderBooksRequest::default())
        .await
        .map_err(|e| format!("metadata: {e}"))?;
    let market = markets
        .order_books
        .iter()
        .find(|m| m.status == "active")
        .ok_or("no active testnet markets")?;
    let market_id = market.market_id;
    let book = client
        .order_book_orders(OrderBookOrdersRequest {
            market_id,
            limit: 10,
        })
        .await
        .map_err(|e| format!("book: {e}"))?;
    let trades = client
        .recent_trades(RecentTradesRequest {
            market_id,
            limit: 2,
        })
        .await
        .map_err(|e| format!("trades: {e}"))?;
    let account = client
        .account(AccountRequest::Index(1))
        .await
        .map_err(|e| format!("account: {e}"))?;
    println!(
        "{} testnet markets; {} ({}): {} asks, {} bids; {} recent trades; {} account(s)",
        markets.order_books.len(),
        market.symbol,
        market.market_id,
        book.asks.len(),
        book.bids.len(),
        trades.trades.len(),
        account.accounts.len()
    );
    for spec in [
        Subscription::OrderBook { market_id },
        Subscription::Trades { market_id },
        Subscription::Account { account_index: 1 },
    ] {
        let mut stream = client.subscribe(spec).await?;
        let event = timeout(Duration::from_secs(15), stream.next())
            .await?
            .ok_or("stream ended before snapshot")??;
        if !event.is_snapshot() {
            return Err("expected native subscription snapshot".into());
        }
        println!(
            "Native {} snapshot",
            match event {
                Event::OrderBook(_) => "book",
                Event::Trades(_) => "trades",
                Event::Account(_) => "account",
                _ => return Err("unexpected stream event".into()),
            }
        );
        stream.unsubscribe().await?;
    }
    Ok(())
}
