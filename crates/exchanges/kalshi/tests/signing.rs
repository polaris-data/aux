use aux_rs_kalshi::*;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::Value;
use zeroize::Zeroizing;
#[tokio::test]
async fn native_algorithms_match_independent_references() {
    let rows: Vec<Value> = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    for r in rows {
        let signer =
            LocalSigner::from_pem(Zeroizing::new(r["private_pem"].as_str().unwrap().into()))
                .unwrap();
        let message = r["message"].as_str().unwrap().as_bytes();
        let expected = STANDARD
            .decode(r["signature_base64"].as_str().unwrap())
            .unwrap();
        verify_request_signature(r["public_pem"].as_str().unwrap(), message, &expected).unwrap();
        let auth = KalshiAuth::new("fixture-key", std::sync::Arc::new(signer)).unwrap();
        let headers = auth
            .headers_at(
                1731536000000,
                "GET",
                "/trade-api/v2/portfolio/balance?ignored=1",
            )
            .await
            .unwrap();
        let signature = STANDARD.decode(&headers[2].1).unwrap();
        verify_request_signature(r["public_pem"].as_str().unwrap(), message, &signature).unwrap();
        if r["kind"] == "ed25519" {
            assert_eq!(signature, expected);
        }
        assert!(
            verify_request_signature(r["public_pem"].as_str().unwrap(), b"altered", &signature)
                .is_err()
        );
    }
}
#[test]
fn environments_preserve_native_roots() {
    let d = Kalshi::demo();
    assert_eq!(
        d.endpoints["perps"],
        "https://external-api.demo.kalshi.co/trade-api/v2"
    );
    assert!(d.websocket["perps"].ends_with("/trade-api/ws/v2/margin"));
    assert_eq!(
        d.endpoints["clearing"],
        "https://demo-api.kalshi.co/klear-api/v1"
    );
}
