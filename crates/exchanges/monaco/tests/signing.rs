use aux_rs_core::{HttpMethod, HttpRequest, hex};
use aux_rs_monaco::*;
use serde_json::Value;
use zeroize::Zeroizing;
fn vectors() -> Value {
    serde_json::from_str(include_str!("fixtures/signing.json")).unwrap()
}
fn auth(protocol: RequestSigning) -> MonacoAuth<LocalSessionSigner> {
    let v = vectors();
    let signer =
        LocalSessionSigner::from_hex(Zeroizing::new(v["seed"].as_str().unwrap().into())).unwrap();
    MonacoAuth::restore(
        Monaco::mainnet(),
        signer,
        SessionInfo {
            address: v["wallet"]["address"].as_str().unwrap().into(),
            client_id: "fixture-client".into(),
            session_public_key: v["publicKey"].as_str().unwrap().into(),
            expires_at: 4_102_444_800,
            chain_id: 1329,
        },
        protocol,
    )
    .unwrap()
}
#[tokio::test]
async fn official_session_vectors_cover_legacy_rfc9421_empty_queries_and_exact_body_bytes() {
    let v = vectors();
    for case in v["http"].as_array().unwrap() {
        for protocol in [RequestSigning::Legacy, RequestSigning::Rfc9421] {
            let a = auth(protocol);
            let mut request = HttpRequest {
                method: if case["method"] == "GET" {
                    HttpMethod::Get
                } else {
                    HttpMethod::Post
                },
                url: format!("https://apimonaco.xyz{}", case["target"].as_str().unwrap()),
                headers: vec![],
                body: case["body"].as_str().unwrap().as_bytes().to_vec(),
            };
            a.sign_request_at(&mut request, v["timestampMs"].as_u64().unwrap())
                .await
                .unwrap();
            let header = |name: &str| {
                request
                    .headers
                    .iter()
                    .find(|(n, _)| n == name)
                    .unwrap()
                    .1
                    .clone()
            };
            match protocol {
                RequestSigning::Legacy => {
                    assert_eq!(header("X-Monaco-Signature"), case["legacySignature"]);
                    assert_eq!(header("X-Monaco-PublicKey"), v["publicKey"]);
                    assert_eq!(header("X-Monaco-Timestamp"), v["timestampMs"].to_string());
                }
                RequestSigning::Rfc9421 => {
                    for name in ["Signature", "Signature-Input", "Content-Digest"] {
                        assert_eq!(header(name), case["rfc9421"][name]);
                    }
                }
            }
        }
    }
    let a = auth(RequestSigning::Legacy);
    assert_eq!(
        hex::encode(
            a.sign_checked(v["websocket"]["message"].as_str().unwrap().as_bytes())
                .await
                .unwrap()
        ),
        v["websocket"]["signature"]
    );
}
#[tokio::test]
async fn wallet_matches_viem_eip191_including_unicode_byte_length() {
    let v = vectors();
    let signer = LocalWalletSigner::from_hex(Zeroizing::new(
        v["wallet"]["privateKey"].as_str().unwrap().into(),
    ))
    .unwrap();
    assert_eq!(
        signer.address(),
        v["wallet"]["address"]
            .as_str()
            .unwrap()
            .to_ascii_lowercase()
    );
    for m in v["wallet"]["messages"].as_array().unwrap() {
        assert_eq!(
            signer
                .sign_message(m["message"].as_str().unwrap())
                .await
                .unwrap(),
            m["signature"]
        );
        let bytes =
            hex::decode(m["signature"].as_str().unwrap().strip_prefix("0x").unwrap()).unwrap();
        let sig = k256::ecdsa::Signature::from_slice(&bytes[..64]).unwrap();
        let rec = k256::ecdsa::RecoveryId::from_byte(bytes[64] - 27).unwrap();
        let key = k256::ecdsa::VerifyingKey::recover_from_prehash(
            &wallet_message_digest(m["message"].as_str().unwrap()),
            &sig,
            rec,
        )
        .unwrap();
        let altered = k256::ecdsa::VerifyingKey::recover_from_prehash(
            &wallet_message_digest("altered challenge"),
            &sig,
            rec,
        )
        .unwrap();
        assert_ne!(key, altered);
    }
}
#[tokio::test]
async fn signatures_fail_if_payload_or_key_changes_and_credentials_are_redacted() {
    let v = vectors();
    let a = auth(RequestSigning::Legacy);
    let msg = v["http"][2]["legacyBase"].as_str().unwrap();
    let sig = a.sign_checked(msg.as_bytes()).await.unwrap();
    let key = ed25519_dalek::VerifyingKey::from_bytes(&a.signer().public_key()).unwrap();
    assert!(
        key.verify_strict(
            msg.replace("POST", "PUT").as_bytes(),
            &ed25519_dalek::Signature::from_bytes(&sig)
        )
        .is_err()
    );
    let other = LocalSessionSigner::from_bytes(Zeroizing::new([7; 32]));
    assert!(
        ed25519_dalek::VerifyingKey::from_bytes(&other.public_key())
            .unwrap()
            .verify_strict(msg.as_bytes(), &ed25519_dalek::Signature::from_bytes(&sig))
            .is_err()
    );
    let mut r = HttpRequest {
        method: HttpMethod::Get,
        url: "https://staging.apimonaco.xyz/api/v1/orders".into(),
        headers: vec![],
        body: vec![],
    };
    assert!(matches!(
        a.sign_request_at(&mut r, 0).await,
        Err(Error::Authentication(_))
    ));
    r.url = "https://apimonaco.xyz/api/v1/orders".into();
    r.headers.push(("signature".into(), "override".into()));
    assert!(matches!(
        a.sign_request_at(&mut r, 0).await,
        Err(Error::InvalidRequest(_))
    ));
    let formatted = format!("{a:?} {:?}", a.signer());
    assert!(!formatted.contains(v["seed"].as_str().unwrap()));
    let clone = a.clone();
    a.invalidate();
    assert!(!clone.is_active());
    assert!(clone.sign_checked(b"request").await.is_err());
}
struct BadSigner;
impl SessionSigner for BadSigner {
    fn public_key(&self) -> [u8; 32] {
        LocalSessionSigner::from_bytes(Zeroizing::new([1; 32])).public_key()
    }
    async fn sign(&self, _: &[u8]) -> Result<[u8; 64], Error> {
        Ok([0; 64])
    }
}
#[tokio::test]
async fn external_signatures_are_verified_before_submission_and_expired_restore_is_rejected() {
    let mut info = auth(RequestSigning::Legacy).session_info();
    let bad = BadSigner;
    info.session_public_key = hex::encode(bad.public_key());
    let auth =
        MonacoAuth::restore(Monaco::mainnet(), bad, info.clone(), RequestSigning::Legacy).unwrap();
    assert!(matches!(
        auth.sign_checked(b"test").await,
        Err(Error::Signing(_))
    ));
    info.expires_at = 0;
    assert!(
        MonacoAuth::restore(
            Monaco::mainnet(),
            BadSigner,
            info.clone(),
            RequestSigning::Legacy
        )
        .is_err()
    );
    info.expires_at = 4_102_444_800;
    info.chain_id = 1328;
    assert!(
        MonacoAuth::restore(Monaco::mainnet(), BadSigner, info, RequestSigning::Legacy).is_err()
    );
}

#[tokio::test]
async fn published_rust_sdk_rfc9421_vectors_match_including_an_explicit_empty_query() {
    let vectors: Vec<Value> = serde_json::from_str(include_str!("fixtures/rfc9421.json")).unwrap();
    for v in vectors {
        let signer =
            LocalSessionSigner::from_hex(Zeroizing::new(v["privateKey"].as_str().unwrap().into()))
                .unwrap();
        let info = SessionInfo {
            address: format!("0x{}", "11".repeat(20)),
            client_id: "fixture".into(),
            session_public_key: v["publicKey"].as_str().unwrap().into(),
            expires_at: 4_102_444_800,
            chain_id: 1329,
        };
        let a =
            MonacoAuth::restore(Monaco::mainnet(), signer, info, RequestSigning::Rfc9421).unwrap();
        let mut r = HttpRequest {
            method: if v["method"] == "GET" {
                HttpMethod::Get
            } else {
                HttpMethod::Post
            },
            url: format!("https://apimonaco.xyz{}", v["target"].as_str().unwrap()),
            headers: vec![],
            body: v["body"].as_str().unwrap().as_bytes().to_vec(),
        };
        a.sign_request_at(&mut r, v["created"].as_u64().unwrap() * 1000)
            .await
            .unwrap();
        for (header, key) in [
            ("Signature", "signature"),
            ("Signature-Input", "signatureInput"),
            ("Content-Digest", "contentDigest"),
        ] {
            assert_eq!(
                r.headers.iter().find(|(k, _)| k == header).unwrap().1,
                v[key]
            );
        }
    }
}
