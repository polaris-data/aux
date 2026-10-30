use aux_rs::hyperliquid::HyperliquidClient;
use aux_rs::hyperliquid::{Address, Event, OpenOrdersRequest, Subscription, UserStateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user: Address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "0x0000000000000000000000000000000000000000".into())
        .parse()?;
    let client = HyperliquidClient::testnet()?;
    let state = client
        .clearinghouse_state(UserStateRequest::new(user))
        .await?;
    let orders = client.open_orders(OpenOrdersRequest::new(user)).await?;
    println!(
        "Account {user}: {} positions, {} open orders",
        state.asset_positions.len(),
        orders.len()
    );
    let mut states = client
        .subscribe(Subscription::ClearinghouseState { user, dex: None })
        .await?;
    let mut orders = client
        .subscribe(Subscription::OpenOrders { user, dex: None })
        .await?;
    let mut fills = client
        .subscribe(Subscription::UserFills {
            user,
            aggregate_by_time: None,
        })
        .await?;
    let mut updates = client
        .subscribe(Subscription::OrderUpdates { user })
        .await?;
    let observed = tokio::time::timeout(std::time::Duration::from_secs(20), async {
        let (mut saw_state, mut saw_orders, mut saw_fills, mut saw_updates_ack) = (false, false, false, false);
        while !(saw_state && saw_orders && saw_fills && saw_updates_ack) {
            tokio::select! {
                event = states.next(), if !saw_state => {
                    if matches!(event.ok_or("state stream closed")??, Event::ClearinghouseState(_)) { saw_state = true; println!("Account state snapshot received"); }
                },
                event = orders.next(), if !saw_orders => {
                    if matches!(event.ok_or("orders stream closed")??, Event::OpenOrders(_)) { saw_orders = true; println!("Open orders snapshot received"); }
                },
                event = fills.next(), if !saw_fills => {
                    if matches!(event.ok_or("fills stream closed")??, Event::UserFills(_)) { saw_fills = true; println!("User fills snapshot received"); }
                },
                event = updates.next(), if !saw_updates_ack => {
                    if matches!(event.ok_or("updates stream closed")??, Event::SubscriptionResponse(_)) { saw_updates_ack = true; println!("Order updates subscription acknowledged"); }
                },
            }
        }
        Ok::<_, Box<dyn std::error::Error>>(())
    }).await;
    states.close().await?;
    orders.close().await?;
    fills.close().await?;
    updates.close().await?;
    observed??;
    Ok(())
}
