//! Explicitly opt-in. Retain the original idempotency key/payload on unknown outcomes.
use aux_rs::monaco::*;
use std::{env, error::Error, time::Duration};
use zeroize::Zeroizing;
fn required(name: &str) -> Result<String, Box<dyn Error>> {
    env::var(name).map_err(|_| format!("set {name}").into())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if !env::args().any(|arg| arg == "--execute") {
        return Err("pass --execute to submit and cancel a testnet order".into());
    }
    let wallet =
        LocalWalletSigner::from_hex(Zeroizing::new(required("MONACO_TESTNET_WALLET_KEY")?))?;
    let client_id = required("MONACO_TESTNET_CLIENT_ID")?;
    let pair_id = required("MONACO_TESTNET_PAIR_ID")?;
    let price = required("MONACO_TESTNET_PRICE")?;
    let quantity = required("MONACO_TESTNET_QUANTITY")?;
    let key = required("MONACO_TESTNET_IDEMPOTENCY_KEY")?;
    let public = MonacoClient::testnet()?;
    let auth = public
        .login(
            &wallet,
            client_id,
            LocalSessionSigner::generate()?,
            LoginOptions::default(),
        )
        .await?;
    let client = public.with_auth(auth);
    let result = async {
        let market = client
            .trading_pairs(TradingPairsRequest {
                page_size: Some(100),
                ..Default::default()
            })
            .await?
            .trading_pairs
            .unwrap_or_default()
            .into_iter()
            .find(|m| m.id.as_deref() == Some(&pair_id))
            .ok_or("pair not found on first page")?;
        let mut stream = client
            .subscribe_account(AccountSubscription::Orders {
                trading_pair_id: Some(pair_id.clone()),
                mode: market.market_type.clone(),
            })
            .await?;
        tokio::time::timeout(Duration::from_secs(15), async {
            while let Some(event) = stream.next().await {
                if event?.kind == "Snapshot" {
                    return Ok::<_, aux_rs::Error<aux_rs::monaco::Error>>(());
                }
            }
            Err(aux_rs::Error::Exchange(
                aux_rs::monaco::Error::Authentication("order stream ended before baseline"),
            ))
        })
        .await??;
        let mut order = CreateOrderRequest::limit(pair_id, "BUY", price, quantity);
        order.idempotency_key = Some(key);
        order.trading_mode = market.market_type.clone();
        order.post_only = Some(true);
        if market.market_type.as_deref() == Some("MARGIN") {
            order.leverage = Some(required("MONACO_TESTNET_LEVERAGE")?);
        }
        let placed = client.place_order(&market, order).await?;
        println!("Native order outcome: {placed:?}");
        if let Some(id) = placed.order_id {
            println!(
                "Cancel: {:?}",
                client
                    .cancel_order(CancelOrderRequest {
                        order_id: id,
                        ..Default::default()
                    })
                    .await?
            );
        }
        stream.close().await?;
        Ok::<_, Box<dyn Error>>(())
    }
    .await;
    let _ = client.revoke_session().await;
    client.transport().shutdown().await?;
    result
}
