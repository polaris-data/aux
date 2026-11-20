#![cfg(feature = "ws")]

use aux_rs_core::TransportErrorKind;
use aux_rs_hyperdash::{
    Channel, Error, HyperdashClient, HyperdashConfig, StreamId, StreamMessage, SubscribeRequest,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::time::Duration;
use tokio::{
    net::{TcpListener, TcpStream},
    time::timeout,
};
use tokio_tungstenite::{
    WebSocketStream, accept_hdr_async,
    tungstenite::{
        Message,
        handshake::server::{ErrorResponse, Request, Response},
        protocol::{CloseFrame, frame::coding::CloseCode},
    },
};

const KEY: &str = "hdk_sk_fixture_secret";
const READY: &str = r#"{"op":"ready","key_id":"test","environment":"live"}"#;
type Socket = WebSocketStream<TcpStream>;

#[allow(clippy::result_large_err)]
fn authenticate(req: &Request, response: Response) -> Result<Response, ErrorResponse> {
    assert_eq!(req.uri().path(), "/v1/stream");
    assert_eq!(req.headers()["authorization"], format!("Bearer {KEY}"));
    Ok(response)
}

async fn server() -> (TcpListener, HyperdashConfig) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let config = HyperdashConfig {
        ws_url: format!("ws://{address}/v1/stream"),
        api_key: Some(KEY.into()),
        stream_ready_timeout: Duration::from_secs(2),
        ..Default::default()
    };
    (listener, config)
}

async fn accept(listener: TcpListener) -> Socket {
    accept_hdr_async(listener.accept().await.unwrap().0, authenticate)
        .await
        .unwrap()
}

async fn send(socket: &mut Socket, message: impl Into<String>) {
    socket.send(Message::text(message.into())).await.unwrap();
}

async fn read(socket: &mut Socket) -> Value {
    let frame = timeout(Duration::from_secs(2), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    serde_json::from_str(frame.to_text().unwrap()).unwrap()
}

fn subscribe(channel: Channel, coin: &str) -> SubscribeRequest {
    SubscribeRequest {
        channel,
        coin: Some(coin.into()),
        address: None,
        interval: None,
        cursor: Some("saved/+cursor".into()),
    }
}

#[tokio::test]
async fn multiplexes_and_bills_only_explicit_acknowledgements() {
    let (listener, config) = server().await;
    let peer = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send(&mut socket, r#"{"op":"ping"}"#).await;
        assert_eq!(read(&mut socket).await, json!({"op": "pong"}));
        send(&mut socket, READY).await;
        assert_eq!(
            read(&mut socket).await,
            json!({
                "op": "subscribe", "channel": "trades", "coin": "BTC", "cursor": "saved/+cursor",
            })
        );
        assert_eq!(
            read(&mut socket).await,
            json!({
                "op": "subscribe", "channel": "liquidations", "coin": "*", "cursor": "saved/+cursor",
            })
        );
        send(&mut socket, r#"{"op":"subscribed","sub":"trades-id","channel":"trades","coin":"BTC","cursor":null}"#).await;
        send(
            &mut socket,
            r#"{"op":"subscribed","sub":2,"channel":"liquidations","coin":"*"}"#,
        )
        .await;
        send(
            &mut socket,
            r#"{"op":"gap","sub":2,"from":400,"to":500,"detail":"history unavailable"}"#,
        )
        .await;
        send(&mut socket, r#"{"op":"data","sub":"trades-id","batch":18446744073709551615,"cursor":"checkpoint","events":[{"px":"1.2300","future_field":42}],"credits":1}"#).await;
        send(&mut socket, r#"{"op":"data","sub":2,"batch":"batch-two","cursor":"second","events":[],"credits":0}"#).await;
        assert!(
            timeout(Duration::from_millis(40), socket.next())
                .await
                .is_err()
        );
        send(&mut socket, r#"{"op":"ping"}"#).await;
        assert_eq!(read(&mut socket).await, json!({"op": "pong"}));
        send(&mut socket, r#"{"op":"credits_low","remaining":9}"#).await;
        assert_eq!(
            read(&mut socket).await,
            json!({
                "op": "ack", "sub": "trades-id", "batch": 18_446_744_073_709_551_615u64,
            })
        );
        send(
            &mut socket,
            r#"{"op":"acked","sub":"trades-id","batch":18446744073709551615,"credits_charged":1}"#,
        )
        .await;
        assert_eq!(
            read(&mut socket).await,
            json!({"op": "unsubscribe", "sub": 2})
        );
        send(&mut socket, r#"{"op":"unsubscribed","sub":2}"#).await;
        send(
            &mut socket,
            r#"{"op":"future_control","custom":"retained"}"#,
        )
        .await;
        socket
            .close(Some(CloseFrame {
                code: CloseCode::Restart,
                reason: "restart".into(),
            }))
            .await
            .unwrap();
    });
    let client = HyperdashClient::with_config(config, Default::default()).unwrap();
    let stream = client.open_stream().await.unwrap();
    assert!(matches!(
        stream.next().await.unwrap(),
        Some(StreamMessage::Ready(_))
    ));
    stream
        .subscribe(subscribe(Channel::Trades, "BTC"))
        .await
        .unwrap();
    stream
        .subscribe(subscribe(Channel::Liquidations, "*"))
        .await
        .unwrap();
    assert!(
        matches!(stream.next().await.unwrap(), Some(StreamMessage::Subscribed(r)) if r.cursor == Some(None))
    );
    assert!(
        matches!(stream.next().await.unwrap(), Some(StreamMessage::Subscribed(r)) if r.cursor.is_none())
    );
    assert!(
        matches!(stream.next().await.unwrap(), Some(StreamMessage::Gap(g)) if g.from == 400 && g.to == 500)
    );
    let Some(StreamMessage::Data(batch)) = stream.next().await.unwrap() else {
        panic!("missing trade batch")
    };
    assert_eq!(batch.events[0]["px"], "1.2300");
    assert_eq!(batch.events[0]["future_field"], 42);
    assert_eq!(batch.cursor, "checkpoint");
    assert_eq!(batch.batch, StreamId::Integer(18_446_744_073_709_551_615));
    assert!(
        matches!(stream.next().await.unwrap(), Some(StreamMessage::Data(r)) if r.sub == StreamId::Integer(2))
    );
    assert!(
        matches!(stream.next().await.unwrap(), Some(StreamMessage::CreditsLow(r)) if r.remaining == 9)
    );
    assert!(matches!(
        stream.ack(batch.sub.clone(), StreamId::Integer(1)).await,
        Err(Error::InvalidRequest(_))
    ));
    stream
        .ack(batch.sub.clone(), batch.batch.clone())
        .await
        .unwrap();
    assert!(matches!(
        stream.ack(batch.sub, batch.batch).await,
        Err(Error::InvalidRequest(_))
    ));
    assert!(
        matches!(stream.next().await.unwrap(), Some(StreamMessage::Acked(r)) if r.credits_charged == 1)
    );
    stream.unsubscribe(StreamId::Integer(2)).await.unwrap();
    assert!(matches!(
        stream.next().await.unwrap(),
        Some(StreamMessage::Unsubscribed(_))
    ));
    assert!(
        matches!(stream.next().await.unwrap(), Some(StreamMessage::Unknown { native }) if native["custom"] == "retained")
    );
    assert!(
        matches!(stream.next().await, Err(Error::StreamClosed { code: Some(1012), reason: Some(reason) }) if reason == "restart")
    );
    assert!(stream.next().await.unwrap().is_none());
    stream.close().await;
    assert!(stream.closed());
    peer.await.unwrap();
}

#[tokio::test]
async fn preserves_terminal_vendor_details_and_redacts_the_key() {
    let (listener, config) = server().await;
    let peer = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send(&mut socket, READY).await;
        send(&mut socket, format!(
            r#"{{"op":"error","code":"credits_exhausted","detail":"Bearer {KEY}","retry_after_s":2}}"#
        )).await;
        socket
            .close(Some(CloseFrame {
                code: CloseCode::Policy,
                reason: format!("credits_exhausted: {KEY}").into(),
            }))
            .await
            .unwrap();
    });
    let client = HyperdashClient::with_config(config, Default::default()).unwrap();
    let stream = client.open_stream().await.unwrap();
    assert!(matches!(
        stream.next().await.unwrap(),
        Some(StreamMessage::Ready(_))
    ));
    assert!(
        matches!(stream.next().await.unwrap(), Some(StreamMessage::Error(error))
        if error.code == "credits_exhausted" && error.detail == "[redacted]" && error.retry_after_s == Some(2))
    );
    assert!(
        matches!(stream.next().await, Err(Error::StreamClosed { code: Some(1008), reason: Some(reason) })
        if reason == "credits_exhausted: [redacted]")
    );
    peer.await.unwrap();
}

#[tokio::test]
async fn overflow_terminates_and_discards_queued_messages() {
    let (listener, mut config) = server().await;
    config.stream_capacity = 1;
    let peer = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send(&mut socket, READY).await;
        send(&mut socket, r#"{"op":"credits_low","remaining":9}"#).await;
        assert!(matches!(
            socket.next().await,
            Some(Ok(Message::Close(_))) | None
        ));
    });
    let client = HyperdashClient::with_config(config, Default::default()).unwrap();
    let stream = client.open_stream().await.unwrap();
    timeout(Duration::from_secs(3), peer)
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(stream.next().await, Err(Error::Transport(error)) if error.kind == TransportErrorKind::Overflow)
    );
    assert!(stream.next().await.unwrap().is_none());
}

#[tokio::test]
async fn cancelled_read_is_reusable_and_close_interrupts_the_next_read() {
    let (listener, config) = server().await;
    let peer = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send(&mut socket, READY).await;
        assert!(matches!(
            socket.next().await,
            Some(Ok(Message::Close(_))) | None
        ));
    });
    let client = HyperdashClient::with_config(config, Default::default()).unwrap();
    let stream = client.open_stream().await.unwrap();
    stream.next().await.unwrap().unwrap();
    assert!(
        timeout(Duration::from_millis(20), stream.next())
            .await
            .is_err()
    );
    let (result, ()) = tokio::join!(stream.next(), stream.close());
    assert!(result.unwrap().is_none());
    timeout(Duration::from_secs(3), peer)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn startup_waits_for_ready_and_times_out() {
    let (listener, mut config) = server().await;
    config.stream_ready_timeout = Duration::from_millis(40);
    let peer = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        assert!(matches!(
            socket.next().await,
            Some(Ok(Message::Close(_))) | None
        ));
    });
    let client = HyperdashClient::with_config(config, Default::default()).unwrap();
    assert!(
        matches!(client.open_stream().await, Err(Error::Transport(error)) if error.kind == TransportErrorKind::Timeout)
    );
    timeout(Duration::from_secs(3), peer)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn all_documented_channels_share_one_connection() {
    let (listener, config) = server().await;
    let channels = [
        (Channel::Fills, "fills", "address"),
        (Channel::Ledger, "ledger", "address"),
        (Channel::Twap, "twap", "address"),
        (Channel::Orders, "orders", "address"),
        (Channel::Funding, "funding", "address"),
        (Channel::Account, "account", "address"),
        (Channel::Trades, "trades", "coin"),
        (Channel::Liquidations, "liquidations", "coin"),
        (Channel::Candles, "candles", "coin"),
        (Channel::Ctx, "ctx", "coin"),
        (Channel::Book, "book", "coin"),
        (Channel::Bbo, "bbo", "coin"),
        (Channel::LiquidationLevels, "liquidation-levels", "coin"),
        (Channel::StopLevels, "stop-levels", "coin"),
        (Channel::Mids, "mids", "all"),
        (Channel::Markets, "markets", "all"),
    ];
    let wallet = "0x0000000000000000000000000000000000000001";
    let peer = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send(&mut socket, READY).await;
        for (_, wire, target) in channels {
            let message = read(&mut socket).await;
            assert_eq!(message["op"], "subscribe");
            assert_eq!(message["channel"], wire);
            match target {
                "address" => assert_eq!(message["address"], wallet),
                "coin" => assert_eq!(message["coin"], "ETH"),
                _ => assert!(message.get("coin").is_none()),
            }
            if wire == "candles" {
                assert_eq!(message["interval"], "15m");
            }
        }
    });
    let client = HyperdashClient::with_config(config, Default::default()).unwrap();
    let stream = client.open_stream().await.unwrap();
    for (channel, _, target) in channels {
        let interval = (channel == Channel::Candles)
            .then_some(aux_rs_hyperdash::CandleInterval::FifteenMinutes);
        stream
            .subscribe(SubscribeRequest {
                channel,
                address: (target == "address").then(|| wallet.into()),
                coin: (target == "coin").then(|| "ETH".into()),
                interval,
                cursor: None,
            })
            .await
            .unwrap();
    }
    timeout(Duration::from_secs(3), peer)
        .await
        .unwrap()
        .unwrap();
    stream.close().await;
}
