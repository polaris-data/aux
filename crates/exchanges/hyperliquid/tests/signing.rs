use aux_rs_hyperliquid::{
    Action, Address, LocalSigner, Network, Signature, Signer,
    signing::{REFERENCE_REVISION, UserSignedAction, action_hash, l1_signing_payload},
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Vectors {
    reference: Value,
    public_test_private_key: String,
    signer_address: Address,
    l1: Vec<L1Vector>,
    user_signed: Vec<UserVector>,
}
#[derive(Deserialize)]
struct L1Vector {
    name: String,
    action: Action,
    network: String,
    vault_address: Option<Address>,
    nonce: u64,
    expires_after: Option<u64>,
    msgpack: String,
    action_hash: String,
    digest: String,
    signature: Signature,
}
#[derive(Deserialize)]
struct UserVector {
    name: String,
    action: Value,
    network: String,
    signature_chain_id: u64,
    digest: String,
    signature: Signature,
}
fn fixtures() -> Vectors {
    serde_json::from_str(include_str!("fixtures/signing_vectors.json")).unwrap()
}
fn network(value: &str) -> Network {
    match value {
        "Mainnet" => Network::Mainnet,
        "Testnet" => Network::Testnet,
        _ => panic!("unknown fixture network"),
    }
}
fn encoded(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

#[tokio::test]
async fn typed_l1_actions_match_the_pinned_python_reference_byte_for_byte() {
    let vectors = fixtures();
    assert_eq!(vectors.reference["revision"], REFERENCE_REVISION);
    assert_eq!(vectors.l1.len(), 36);
    let signer: LocalSigner = vectors.public_test_private_key.parse().unwrap();
    assert_eq!(signer.address(), vectors.signer_address);
    for vector in vectors.l1 {
        assert_eq!(
            hex::encode(rmp_serde::to_vec_named(&vector.action).unwrap()),
            vector.msgpack,
            "{} MessagePack",
            vector.name
        );
        let hash = action_hash(
            &vector.action,
            vector.vault_address,
            vector.nonce,
            vector.expires_after,
        )
        .unwrap();
        assert_eq!(
            encoded(&hash),
            vector.action_hash,
            "{} action hash",
            vector.name
        );
        let payload = l1_signing_payload(
            &vector.action,
            vector.vault_address,
            vector.nonce,
            vector.expires_after,
            network(&vector.network),
        )
        .unwrap();
        assert_eq!(
            encoded(payload.digest()),
            vector.digest,
            "{} EIP-712 digest",
            vector.name
        );
        let signature = signer.sign(payload.clone()).await.unwrap();
        assert_eq!(signature, vector.signature, "{} signature", vector.name);
        assert_eq!(
            signature.recover_address(&payload).unwrap(),
            vectors.signer_address
        );
    }
}

#[tokio::test]
async fn user_signed_scheme_matches_reference_and_wallet_domain_separation() {
    let vectors = fixtures();
    let signer: LocalSigner = vectors.public_test_private_key.parse().unwrap();
    assert_eq!(vectors.user_signed.len(), 8);
    for vector in vectors.user_signed {
        let action = if vector.name == "usd_send" {
            UserSignedAction::UsdSend {
                destination: vector.action["destination"]
                    .as_str()
                    .unwrap()
                    .parse()
                    .unwrap(),
                amount: vector.action["amount"].as_str().unwrap().into(),
                time: vector.action["time"].as_u64().unwrap(),
            }
        } else {
            UserSignedAction::ApproveAgent {
                agent_address: vector.action["agentAddress"]
                    .as_str()
                    .unwrap()
                    .parse()
                    .unwrap(),
                agent_name: vector.action["agentName"].as_str().unwrap().into(),
                nonce: vector.action["nonce"].as_u64().unwrap(),
            }
        };
        let payload = action
            .signing_payload(network(&vector.network), vector.signature_chain_id)
            .unwrap();
        assert_eq!(encoded(payload.digest()), vector.digest);
        let signature = signer.sign(payload.clone()).await.unwrap();
        assert_eq!(signature, vector.signature);
        assert_eq!(
            signature.recover_address(&payload).unwrap(),
            vectors.signer_address
        );
    }
}

#[tokio::test]
async fn changing_signed_context_or_field_order_invalidates_a_signature() {
    let vectors = fixtures();
    let vector = &vectors.l1[0];
    let payload =
        l1_signing_payload(&vector.action, None, vector.nonce, None, Network::Mainnet).unwrap();
    let signer: LocalSigner = vectors.public_test_private_key.parse().unwrap();
    let signature = signer.sign(payload).await.unwrap();
    for altered in [
        l1_signing_payload(&vector.action, None, vector.nonce, None, Network::Testnet).unwrap(),
        l1_signing_payload(
            &vector.action,
            None,
            vector.nonce + 1,
            None,
            Network::Mainnet,
        )
        .unwrap(),
        l1_signing_payload(
            &vector.action,
            None,
            vector.nonce,
            Some(vector.nonce + 100),
            Network::Mainnet,
        )
        .unwrap(),
        l1_signing_payload(
            &vector.action,
            Some(Address::from_bytes([1; 20])),
            vector.nonce,
            None,
            Network::Mainnet,
        )
        .unwrap(),
    ] {
        assert_ne!(
            signature.recover_address(&altered).unwrap(),
            signer.address()
        );
    }
    // JSON Value sorts fields. This must differ from canonical typed signing.
    let sorted = serde_json::to_value(&vector.action).unwrap();
    assert_ne!(
        action_hash(&sorted, None, vector.nonce, None).unwrap(),
        action_hash(&vector.action, None, vector.nonce, None).unwrap()
    );
    let mut bad = signature.clone();
    bad.v = 29;
    let payload =
        l1_signing_payload(&vector.action, None, vector.nonce, None, Network::Mainnet).unwrap();
    assert!(bad.recover_address(&payload).is_err());
    bad = signature;
    bad.r = "0x0".into();
    assert!(bad.recover_address(&payload).is_err());
}

#[test]
fn keys_are_validated_and_never_formatted() {
    let vectors = fixtures();
    let signer: LocalSigner = vectors.public_test_private_key.parse().unwrap();
    assert!(!format!("{signer:?}").contains(&vectors.public_test_private_key[2..]));
    for invalid in ["", "not a key", "0x00", &"0".repeat(64), &"f".repeat(64)] {
        let error = invalid.parse::<LocalSigner>().unwrap_err();
        assert!(!format!("{error:?} {error}").contains(invalid) || invalid.is_empty());
    }
}
