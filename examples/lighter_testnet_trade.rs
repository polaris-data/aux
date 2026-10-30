//! Opt-in testnet only; requires a registered API key and funded testnet account.
//! Local primitives are unaudited. See crates/exchanges/lighter/SIGNING.md.
use aux_rs::lighter::LighterClient;
use aux_rs::lighter::*;
use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() != Some("--execute") {
        return Err("Pass --execute with LIGHTER_API_PRIVATE_KEY, LIGHTER_ACCOUNT_INDEX, LIGHTER_API_KEY_INDEX, LIGHTER_TESTNET_MARKET_ID, LIGHTER_TESTNET_ORDER_PRICE and LIGHTER_TESTNET_ORDER_SIZE to submit/cancel a post-only testnet ask".into());
    }
    let signer = Arc::new(LocalSigner::from_hex(Zeroizing::new(std::env::var(
        "LIGHTER_API_PRIVATE_KEY",
    )?))?);
    let auth = LighterAuth::new(
        signer,
        Network::Testnet.chain_id(),
        std::env::var("LIGHTER_ACCOUNT_INDEX")?.parse()?,
        std::env::var("LIGHTER_API_KEY_INDEX")?.parse()?,
    )?;
    let client = LighterClient::testnet()?.with_auth(auth);
    let market_id = std::env::var("LIGHTER_TESTNET_MARKET_ID")?.parse()?;
    let markets = client
        .order_books(OrderBooksRequest {
            market_id: Some(market_id),
        })
        .await?;
    let market = markets
        .order_books
        .first()
        .ok_or("testnet market not found")?;
    let now = i64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())?;
    let client_order_index = now * 100 % ((1i64 << 48) - 1);
    let order = CreateOrder {
        market_index: market_id,
        client_order_index,
        base_amount: market.scale_size(&std::env::var("LIGHTER_TESTNET_ORDER_SIZE")?)?,
        price: market.scale_price(&std::env::var("LIGHTER_TESTNET_ORDER_PRICE")?)?,
        is_ask: 1,
        order_type: 0,
        time_in_force: 2,
        reduce_only: 0,
        trigger_price: 0,
        order_expiry: now + 600_000,
    };
    let mut stream = client.subscribe_account_orders().await?;
    let placed = client.create_order(order, TxOptions::short_lived()?).await;
    let placed = match placed {
        Ok(v) => v,
        Err(error) => {
            stream.unsubscribe().await?;
            eprintln!(
                "Testnet submit failed: {error}; client order index {client_order_index}. Reconcile native account/transaction state before any nonce resync. No replay or automatic resync was attempted."
            );
            return Err(error.into());
        }
    };
    println!(
        "Testnet submission accepted: {} (client order {client_order_index})",
        placed.tx_hash
    );
    // Acceptance is not execution. Poll the native active-order endpoint.
    let mut resting = false;
    for _ in 0..10 {
        if client
            .active_orders(Some(market_id))
            .await?
            .orders
            .iter()
            .any(|o| o.client_order_index == client_order_index)
        {
            resting = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    let cancelled = client
        .cancel_order(
            CancelOrder {
                market_index: market_id,
                index: client_order_index,
            },
            TxOptions::short_lived()?,
        )
        .await?;
    println!("Testnet cancellation accepted: {}", cancelled.tx_hash);
    let mut absent = false;
    for _ in 0..10 {
        if client
            .active_orders(Some(market_id))
            .await?
            .orders
            .iter()
            .all(|o| o.client_order_index != client_order_index)
        {
            absent = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    stream.unsubscribe().await?;
    if !resting || !absent {
        return Err(
            "Expected both a resting order and its subsequent removal; reconcile native outcomes"
                .into(),
        );
    }
    Ok(())
}
