use aux_zig::{DeriveClient, Error, KalshiClient};
use serde_json::json;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> aux_zig::Result<()> {
    let origin = std::env::var("AUX_FIXTURE_ORIGIN")
        .unwrap()
        .replace("http:", "ws:");
    let derive = DeriveClient::new()
        .into_async()
        .open_public_rpc(Some(&format!("{origin}/rpc/derive-public")), &json!({}))
        .await?;
    let denied = derive
        .subscribe_channels(&json!({"channels":["rejection-fixture"]}))
        .await;
    assert!(
        matches!(denied, Err(Error::SessionResponse(error)) if error.venue == "derive" && error.body["error"]["data"]["seq"] == 9007199254740993u64)
    );
    derive.unsubscribe_channels(&json!({"channels":[]})).await?;
    derive.close();

    let kalshi = KalshiClient::new()
        .with_credentials("fixture-k", "07".repeat(32))
        .into_async()
        .open_rpc(
            Some(&format!("{origin}/rpc/kalshi")),
            &json!({"timeout_ms":80}),
        )
        .await?;
    assert!(matches!(
        kalshi.subscribe(&json!({"step":1})).await,
        Err(Error::PendingOutcome)
    ));
    assert_eq!(kalshi.pending().await?.unwrap()["method"], "subscribe");
    assert_eq!(
        kalshi.next_event().await?.unwrap()["value"],
        9007199254740993u64
    );
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_eq!(kalshi.reconcile().await?["ack"], "subscribe");
    assert!(kalshi.pending().await?.is_none());
    kalshi.list_subscriptions(&json!({"step":2})).await?;
    kalshi.update_subscription(&json!({"step":3})).await?;
    kalshi.unsubscribe(&json!({"step":4})).await?;
    kalshi.close();
    kalshi.close();
    Ok(())
}
