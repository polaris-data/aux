#![cfg(feature = "rustls")]
use aux_zig::{ChainlinkCandlestickClient, PolymarketClient, PythHermesClient};
use rustls::{
    ClientConfig, RootCertStore,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};
use serde_json::json;
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::Arc,
};
const TLS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/loopback-tls");

struct Peer(Child);
impl Drop for Peer {
    fn drop(&mut self) {
        self.0.stdin.take();
        let _ = self.0.wait();
    }
}
fn config(trust: bool, identity: bool) -> ClientConfig {
    let mut roots = RootCertStore::empty();
    if trust {
        roots
            .add(CertificateDer::from_pem_file(format!("{TLS}/ca.pem")).unwrap())
            .unwrap();
    }
    let builder =
        ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_root_certificates(roots);
    if identity {
        let cert = CertificateDer::from_pem_file(format!("{TLS}/client.pem")).unwrap();
        let key = PrivateKeyDer::from_pem_file(format!("{TLS}/client-key.pem")).unwrap();
        builder.with_client_auth_cert(vec![cert], key).unwrap()
    } else {
        builder.with_no_client_auth()
    }
}
#[test]
fn custom_ca_and_client_identity_apply_to_http_and_websockets() {
    let mut peer = Peer(
        Command::new("python3")
            .arg(format!("{TLS}/peer.py"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    let mut port = String::new();
    BufReader::new(peer.0.stdout.take().unwrap())
        .read_line(&mut port)
        .unwrap();
    let port = port.trim().parse::<u16>().unwrap();
    let origin = format!("https://127.0.0.1:{port}");
    let socket = format!("wss://127.0.0.1:{port}/poly-ws/sports");
    let sports = json!({"service":"sports"});
    for (trust, identity) in [(false, true), (true, false)] {
        let tls = config(trust, identity);
        assert!(
            PythHermesClient::new()
                .with_origin(&origin)
                .with_tls(tls.clone())
                .live(&json!({}))
                .is_err()
        );
        assert!(
            PolymarketClient::new()
                .with_tls(tls)
                .subscribe(&sports, Some(&socket))
                .is_err()
        );
    }
    let tls = config(true, true);
    let health = PythHermesClient::new()
        .with_origin(&origin)
        .with_tls(tls.clone())
        .live(&json!({}))
        .unwrap();
    assert_eq!(health["data"], "OK");
    let http = ChainlinkCandlestickClient::new()
        .with_tls(tls.clone())
        .with_origin(&origin)
        .with_api_key("test-key")
        .stream_prices(&json!({"symbol":["ETHUSD","BTCUSD"]}))
        .unwrap();
    assert_eq!(http.next_event().unwrap().unwrap()["heartbeat"], 1748525528);
    http.close();
    let ws = PolymarketClient::new()
        .with_tls(tls)
        .subscribe(&sports, Some(&socket))
        .unwrap();
    assert_eq!(ws.next_event().unwrap().unwrap()["score"], "1-0");
    assert!(ws.next_event().unwrap().is_none());
    ws.close();
}
