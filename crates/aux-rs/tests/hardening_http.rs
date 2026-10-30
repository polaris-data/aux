#![cfg(all(feature = "http", feature = "hyperliquid", feature = "lighter"))]
use aux_rs::hyperliquid::HyperliquidClient;
use aux_rs::{hyperliquid as hl, lighter as lt, *};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::oneshot,
    time::{Instant, timeout},
};
use zeroize::Zeroizing;

const META: &str = include_str!("../../exchanges/hyperliquid/tests/fixtures/meta.json");
async fn request(socket: &mut TcpStream) -> (String, Vec<u8>) {
    let mut bytes = Vec::new();
    let end = loop {
        if let Some(i) = bytes.windows(4).position(|v| v == b"\r\n\r\n") {
            break i + 4;
        }
        let mut b = [0; 1024];
        let n = socket.read(&mut b).await.unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&b[..n]);
    };
    let head = String::from_utf8(bytes[..end].to_vec()).unwrap();
    let len = head
        .lines()
        .find_map(|l| {
            l.split_once(':')
                .filter(|(k, _)| k.eq_ignore_ascii_case("content-length"))
                .map(|(_, v)| v.trim().parse::<usize>().unwrap())
        })
        .unwrap_or(0);
    while bytes.len() < end + len {
        let mut b = [0; 1024];
        let n = socket.read(&mut b).await.unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&b[..n]);
    }
    (head, bytes[end..end + len].to_vec())
}
async fn reply(socket: &mut TcpStream, status: u16, body: &str, headers: &str) {
    socket.write_all(format!("HTTP/1.1 {status} Response\r\nContent-Length: {}\r\nConnection: close\r\n{headers}\r\n{body}",body.len()).as_bytes()).await.unwrap();
}
fn config() -> TransportConfig {
    TransportConfig {
        timeout: Duration::from_millis(300),
        read_retry: BackoffConfig {
            max_retries: 2,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(50),
        },
        ..Default::default()
    }
}

#[tokio::test]
async fn native_post_reads_retry_bounded_statuses_and_keep_the_same_request() {
    for status in [408, 429, 500, 502, 503, 504] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let mut bodies = Vec::new();
            for i in 0..3 {
                let (mut socket, _) = listener.accept().await.unwrap();
                let (head, body) = request(&mut socket).await;
                assert!(head.starts_with("POST /info "));
                bodies.push(body);
                reply(
                    &mut socket,
                    if i < 2 { status } else { 200 },
                    if i < 2 { "temporary" } else { META },
                    "Retry-After: 0\r\n",
                )
                .await;
            }
            assert!(bodies.windows(2).all(|pair| pair[0] == pair[1]));
        });
        let p = hl::HyperliquidClient::with_config(
            hl::Hyperliquid::custom(format!("http://{addr}"), "ws://unused"),
            config(),
        )
        .unwrap();
        p.meta().await.unwrap();
        server.await.unwrap();
    }
}
#[tokio::test]
async fn exhausted_retries_preserve_final_native_error_and_never_retry_permanent_errors() {
    for (status, count) in [(503, 3), (400, 1), (401, 1), (403, 1), (307, 1)] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            for _ in 0..count {
                let (mut s, _) = listener.accept().await.unwrap();
                request(&mut s).await;
                reply(
                    &mut s,
                    status,
                    "last-native-body",
                    "Location: http://127.0.0.1:1/never\r\n",
                )
                .await;
            }
            assert!(
                timeout(Duration::from_millis(60), listener.accept())
                    .await
                    .is_err()
            );
        });
        let p = hl::HyperliquidClient::with_config(
            hl::Hyperliquid::custom(format!("http://{addr}"), "ws://unused"),
            config(),
        )
        .unwrap();
        assert!(
            matches!(p.meta().await,Err(Error::Exchange(hl::Error::Http{status:s,body})) if s==status&&body=="last-native-body")
        );
        server.await.unwrap();
    }
}
#[tokio::test]
async fn retry_after_is_honored_and_not_clamped_to_an_earlier_retry() {
    for value in [
        "1",
        "9999999999999999999999999999999",
        "Fri, 01 Jan 2100 00:00:00 GMT",
        "Tuesday, 31-Dec-69 23:59:59 GMT",
        "Tue Dec 31 23:59:59 2069",
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let value = value.to_owned();
        let server = tokio::spawn(async move {
            let (mut s, _) = listener.accept().await.unwrap();
            request(&mut s).await;
            reply(
                &mut s,
                429,
                "native rate limit",
                &format!("Retry-After: {value}\r\n"),
            )
            .await;
            assert!(
                timeout(Duration::from_millis(80), listener.accept())
                    .await
                    .is_err()
            );
        });
        let p = hl::HyperliquidClient::with_config(
            hl::Hyperliquid::custom(format!("http://{addr}"), "ws://unused"),
            config(),
        )
        .unwrap();
        assert!(matches!(
            p.meta().await,
            Err(Error::Exchange(hl::Error::Http { status: 429, .. }))
        ));
        server.await.unwrap();
    }
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut s, _) = listener.accept().await.unwrap();
        request(&mut s).await;
        reply(&mut s, 429, "limit", "Retry-After: 1\r\n").await;
        let began = Instant::now();
        let (mut s, _) = listener.accept().await.unwrap();
        assert!(began.elapsed() >= Duration::from_millis(950));
        request(&mut s).await;
        reply(&mut s, 200, META, "").await;
    });
    let mut cfg = config();
    cfg.read_retry.max_delay = Duration::from_secs(2);
    hl::HyperliquidClient::with_config(
        hl::Hyperliquid::custom(format!("http://{addr}"), "ws://unused"),
        cfg,
    )
    .unwrap()
    .meta()
    .await
    .unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn connection_losses_retry_reads_but_limits_and_decode_failures_do_not() {
    for mode in [0, 1, 2] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut s, _) = listener.accept().await.unwrap();
            request(&mut s).await;
            if mode == 0 {
                drop(s);
                let (mut s, _) = listener.accept().await.unwrap();
                request(&mut s).await;
                reply(&mut s, 200, META, "").await;
            } else {
                reply(&mut s, 200, if mode == 1 { "not JSON" } else { META }, "").await;
                assert!(
                    timeout(Duration::from_millis(60), listener.accept())
                        .await
                        .is_err()
                );
            }
        });
        let mut cfg = config();
        if mode == 2 {
            cfg.max_message_bytes = 8;
        }
        let p = hl::HyperliquidClient::with_config(
            hl::Hyperliquid::custom(format!("http://{addr}"), "ws://unused"),
            cfg,
        )
        .unwrap();
        match mode {
            0 => {
                p.meta().await.unwrap();
            }
            1 => assert!(matches!(
                p.meta().await,
                Err(Error::Exchange(hl::Error::Decode(_)))
            )),
            _ => assert!(
                matches!(p.meta().await,Err(Error::Transport(e)) if e.kind==TransportErrorKind::ResponseTooLarge)
            ),
        };
        server.await.unwrap();
    }
}
#[tokio::test]
async fn trading_submissions_are_single_attempt_even_with_retry_policy_and_ambiguous_failures() {
    for lighter in [false, true] {
        for failure in [0, 1, 2] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                if lighter {
                    let (mut s, _) = listener.accept().await.unwrap();
                    let (head, _) = request(&mut s).await;
                    assert!(head.contains("nextNonce"));
                    reply(&mut s, 200, r#"{"code":200,"nonce":42}"#, "").await;
                }
                let (mut s, _) = listener.accept().await.unwrap();
                let (head, body) = request(&mut s).await;
                assert!(head.starts_with("POST "));
                assert!(!body.is_empty());
                match failure {
                    0 => reply(&mut s, 503, "submission unknown", "").await,
                    1 => {
                        drop(s);
                    }
                    _ => {
                        let mut byte = [0];
                        let _ = s.read(&mut byte).await;
                    }
                }
                assert!(
                    timeout(Duration::from_millis(100), listener.accept())
                        .await
                        .is_err()
                );
            });
            if lighter {
                static ACC: AtomicUsize = AtomicUsize::new(10_000_000);
                let account = ACC.fetch_add(1, Ordering::Relaxed) as i64;
                let mut key = [0; 40];
                key[0] = 1;
                let signer = Arc::new(lt::LocalSigner::from_bytes(Zeroizing::new(key)).unwrap());
                let p = lt::LighterClient::with_config(
                    lt::Lighter::custom(format!("http://{addr}"), "ws://unused", 300),
                    config(),
                )
                .unwrap()
                .with_auth(lt::LighterAuth::new(signer, 300, account, 3).unwrap());
                assert!(
                    p.cancel_order(
                        lt::CancelOrder {
                            market_index: 4096,
                            index: 123
                        },
                        lt::TxOptions::new(1800000000000)
                    )
                    .await
                    .is_err()
                );
                assert!(matches!(
                    p.cancel_order(
                        lt::CancelOrder {
                            market_index: 4096,
                            index: 123
                        },
                        lt::TxOptions::new(1800000000000)
                    )
                    .await,
                    Err(Error::Exchange(lt::Error::NonceUncertain))
                ));
            } else {
                let signer: hl::LocalSigner =
                    "0123456789012345678901234567890123456789012345678901234567890123"
                        .parse()
                        .unwrap();
                let p = HyperliquidClient::with_config(
                    hl::Hyperliquid::custom(format!("http://{addr}"), "ws://unused")
                        .with_network(hl::Network::Testnet),
                    config(),
                )
                .unwrap()
                .with_auth(hl::HyperliquidAuth::direct(signer));
                assert!(
                    p.cancel(hl::CancelRequest {
                        cancels: vec![hl::Cancel { asset: 0, oid: 123 }]
                    })
                    .await
                    .is_err()
                );
            }
            server.await.unwrap();
        }
    }
}
#[tokio::test]
async fn authenticated_read_retries_keep_native_authorization_and_do_not_consume_nonces() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let mut auth = None;
        for i in 0..2 {
            let (mut s, _) = listener.accept().await.unwrap();
            let (head, _) = request(&mut s).await;
            assert!(head.contains("accountActiveOrders"));
            let value = head
                .lines()
                .find(|l| l.starts_with("authorization:"))
                .unwrap()
                .to_owned();
            if let Some(previous) = &auth {
                assert_eq!(previous, &value);
            }
            auth = Some(value);
            reply(
                &mut s,
                if i == 0 { 503 } else { 200 },
                if i == 0 {
                    "busy"
                } else {
                    r#"{"code":200,"orders":[]}"#
                },
                "",
            )
            .await;
        }
    });
    let mut key = [0; 40];
    key[0] = 1;
    let signer = Arc::new(lt::LocalSigner::from_bytes(Zeroizing::new(key)).unwrap());
    let p = lt::LighterClient::with_config(
        lt::Lighter::custom(format!("http://{addr}"), "ws://unused", 300),
        config(),
    )
    .unwrap()
    .with_auth(lt::LighterAuth::new(signer, 300, 37, 3).unwrap());
    assert!(p.active_orders(Some(4096)).await.unwrap().orders.is_empty());
    server.await.unwrap();
}
#[tokio::test]
async fn shutdown_cancels_inflight_http_and_backoff_and_closes_all_clones() {
    for waiting in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (seen, started) = oneshot::channel();
        let server = tokio::spawn(async move {
            let (mut s, _) = listener.accept().await.unwrap();
            request(&mut s).await;
            if waiting {
                reply(&mut s, 503, "busy", "").await;
            }
            seen.send(()).unwrap();
            if !waiting {
                let mut b = [0];
                let _ = s.read(&mut b).await;
            }
            assert!(
                timeout(Duration::from_millis(100), listener.accept())
                    .await
                    .is_err()
            );
        });
        let mut cfg = config();
        cfg.read_retry.initial_delay = Duration::from_secs(2);
        cfg.read_retry.max_delay = Duration::from_secs(2);
        let p = hl::HyperliquidClient::with_config(
            hl::Hyperliquid::custom(format!("http://{addr}"), "ws://unused"),
            cfg,
        )
        .unwrap();
        let copy = p.clone();
        let task = tokio::spawn(async move { copy.meta().await });
        started.await.unwrap();
        timeout(Duration::from_millis(200), p.transport().shutdown())
            .await
            .unwrap()
            .unwrap();
        assert!(
            matches!(task.await.unwrap(),Err(Error::Transport(e)) if e.kind==TransportErrorKind::Closed)
        );
        assert!(
            matches!(p.meta().await,Err(Error::Transport(e)) if e.kind==TransportErrorKind::Closed&&e.outcome==RequestOutcome::NotSent)
        );
        server.await.unwrap();
    }
}
#[test]
fn configuration_and_native_error_formatting_do_not_expose_credentials() {
    let cfg = TransportConfig {
        read_retry: BackoffConfig {
            max_retries: 17,
            ..BackoffConfig::disabled()
        },
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
    for error in [
        hl::Error::Api {
            message: "secret echoed token".into(),
        },
        hl::Error::Http {
            status: 400,
            body: "secret echoed token".into(),
        },
    ] {
        assert!(!format!("{error} {error:?}").contains("secret"));
    }
    let inner = serde_json::from_value::<u8>(serde_json::json!("secret echoed token")).unwrap_err();
    let error = hl::Error::Decode(inner);
    assert!(!format!("{error:?}").contains("secret"));
    assert!(std::error::Error::source(&error).is_none());
}

struct PublicMutation;
impl Endpoint<hl::Hyperliquid> for PublicMutation {
    type Access = Public;
    type Response = u16;
    fn request(&self, exchange: &hl::Hyperliquid) -> Result<HttpRequest, hl::Error> {
        Ok(HttpRequest {
            method: HttpMethod::Get,
            url: format!("{}/mutation", exchange.http_url()),
            headers: vec![],
            body: vec![],
        })
    }
    fn decode(response: HttpResponse) -> Result<u16, hl::Error> {
        Ok(response.status)
    }
}
#[tokio::test]
async fn unmarked_public_endpoints_are_not_retried_even_if_the_method_is_get() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut s, _) = listener.accept().await.unwrap();
        request(&mut s).await;
        reply(&mut s, 503, "failed", "").await;
        assert!(
            timeout(Duration::from_millis(80), listener.accept())
                .await
                .is_err()
        );
    });
    let p = hl::HyperliquidClient::with_config(
        hl::Hyperliquid::custom(format!("http://{addr}"), "ws://unused"),
        config(),
    )
    .unwrap();
    assert_eq!(p.request(PublicMutation).await.unwrap(), 503);
    server.await.unwrap();
}

#[tokio::test]
async fn closed_transports_reject_new_trading_before_signing_or_nonce_allocation() {
    struct DenyHl;
    impl hl::Signer for DenyHl {
        fn address(&self) -> hl::Address {
            hl::Address::from_bytes([1; 20])
        }
        async fn sign(&self, _: hl::SigningPayload) -> Result<hl::Signature, hl::SignerError> {
            panic!("closed transport must not invoke signer")
        }
    }
    struct DenyLt;
    impl lt::Signer for DenyLt {
        fn public_key(&self) -> lt::PublicKey {
            let mut key = [0; 40];
            key[0] = 4;
            lt::PublicKey::from_bytes(key).unwrap()
        }
        async fn sign(&self, _: &lt::SigningPayload) -> Result<lt::Signature, lt::Error> {
            panic!("closed transport must not invoke signer")
        }
    }
    let p = HyperliquidClient::new(
        hl::Hyperliquid::custom("http://127.0.0.1:1", "ws://unused")
            .with_network(hl::Network::Testnet),
    )
    .unwrap()
    .with_auth(hl::HyperliquidAuth::direct(DenyHl));
    p.transport().shutdown().await.unwrap();
    assert!(
        matches!(p.cancel(hl::CancelRequest{cancels:vec![hl::Cancel{asset:0,oid:1}]}).await,Err(Error::Transport(e)) if e.kind==TransportErrorKind::Closed&&e.outcome==RequestOutcome::NotSent)
    );
    let p = lt::LighterClient::new(lt::Lighter::custom(
        "http://127.0.0.1:1",
        "ws://unused",
        300,
    ))
    .unwrap()
    .with_auth(lt::LighterAuth::new(DenyLt, 300, 37, 3).unwrap());
    p.transport().shutdown().await.unwrap();
    assert!(
        matches!(p.cancel_order(lt::CancelOrder{market_index:4096,index:1},lt::TxOptions::new(1800000000000)).await,Err(Error::Transport(e)) if e.kind==TransportErrorKind::Closed&&e.outcome==RequestOutcome::NotSent)
    );
}
