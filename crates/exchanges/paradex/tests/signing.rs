use aux_rs_paradex::*;
use serde_json::{Value, json};
use zeroize::Zeroizing;
#[tokio::test]
async fn official_siwe_and_evm_address_vectors() {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/siwe.json")).unwrap();
    let signer =
        LocalEvmSigner::from_hex(Zeroizing::new(fixture["key"].as_str().unwrap().into())).unwrap();
    assert_eq!(
        evm_account_address(
            fixture["address"].as_str().unwrap(),
            fixture["class_hash"].as_str().unwrap()
        )
        .unwrap(),
        fixture["account"]
    );
    for vector in fixture["vectors"].as_array().unwrap() {
        let e = if vector["network"] == "mainnet" {
            Paradex::mainnet()
        } else {
            Paradex::testnet()
        };
        let message = siwe_message(
            &e,
            fixture["address"].as_str().unwrap(),
            vector["purpose"].as_str().unwrap(),
            vector["nonce"].as_str().unwrap(),
            vector["issued_at"].as_str().unwrap(),
            vector["expiration"].as_str(),
        )
        .unwrap();
        assert_eq!(message, vector["message"]);
        let digest = personal_hash(message.as_bytes());
        assert_eq!(
            format!("0x{}", aux_rs_core::hex::encode(digest)),
            vector["digest"]
        );
        let payload = SigningPayload {
            digest,
            scheme: SigningScheme::EvmPersonal,
            typed_data: None,
            personal_message: Some(message),
        };
        let signature = signer.sign(&payload).await.unwrap();
        assert_eq!(signature[0], vector["signature"]);
        verify_signature(&signer, &payload, &signature).unwrap();
        let mut other = e.clone();
        other.evm_chain_id += 1;
        let auth = ParadexAuth::new(&e, "0x1234", signer.clone(), None).unwrap();
        assert!(auth.authentication_request(&other).await.is_err());
        other = e.clone();
        other.siwe_domain = "different.example".into();
        assert!(auth.authentication_request(&other).await.is_err());
    }
}
#[tokio::test]
async fn official_starknet_and_evm_signature_vectors() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    let stark =
        LocalStarkSigner::from_hex(Zeroizing::new(fixtures["key"].as_str().unwrap().into()))
            .unwrap();
    let evm =
        LocalEvmSigner::from_hex(Zeroizing::new(fixtures["evm_key"].as_str().unwrap().into()))
            .unwrap();
    assert_eq!(stark.public_key(), fixtures["public_key"]);
    for vector in fixtures["vectors"].as_array().unwrap() {
        let mut e = Paradex::testnet();
        e.chain_id = vector["network"].as_str().unwrap().into();
        let payload = typed_payload(
            fixtures["account"].as_str().unwrap(),
            vector["typed_data"].clone(),
        )
        .unwrap();
        assert_eq!(
            format!(
                "{:#x}",
                starknet_core::types::Felt::from_bytes_be(&payload.digest)
            ),
            vector["digest"],
            "{}",
            vector["name"]
        );
        let sig = stark.sign(&payload).await.unwrap();
        assert_eq!(json!(sig), vector["signature"], "{}", vector["name"]);
        verify_signature(&stark, &payload, &sig).unwrap();
        let sig = evm.sign(&payload).await.unwrap();
        assert_eq!(json!(sig), vector["evm_signature"]);
        verify_signature(&evm, &payload, &sig).unwrap();
        let mut changed = payload.clone();
        changed.digest[31] ^= 1;
        assert!(
            verify_signature(
                &stark,
                &changed,
                &serde_json::from_value::<Vec<String>>(vector["signature"].clone()).unwrap()
            )
            .is_err()
        );
        let native = match vector["name"].as_str().unwrap() {
            "auth" => Some(auth_typed_data(&e, 1793710000, 1793710300).unwrap()),
            "onboarding" => Some(onboarding_typed_data(&e).unwrap()),
            "order_LIMIT" | "order_MARKET" => {
                Some(order_typed_data(&e, &vector["inputs"], None).unwrap())
            }
            "modify" => Some(order_typed_data(&e, &vector["inputs"], Some("123456")).unwrap()),
            _ => None,
        };
        if let Some(native) = native {
            assert_eq!(native, vector["typed_data"], "{}", vector["name"]);
        }
    }
}
#[test]
fn block_merkle_payloads_match_native_reference() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    for vector in fixtures["vectors"].as_array().unwrap() {
        let name = vector["name"].as_str().unwrap();
        if name != "block" && name != "offer" {
            continue;
        }
        let mut e = Paradex::testnet();
        e.chain_id = vector["network"].as_str().unwrap().into();
        let order =
            json!({"account":"0x1234","side":"BUY","type":"LIMIT","size":"1","price":"3100"});
        let trades = if name == "block" {
            json!({"ETH-USD-PERP":{"price":"3100","size":"1","maker_order":order,"taker_order":{"account":"0x5678","side":"SELL","type":"LIMIT","size":"1","price":"3100"}},"BTC-USD-PERP":{"min_size":"0.01","max_size":"1","min_price":"60000","max_price":"70000"}})
        } else {
            json!({"ETH-USD-PERP":{"price":"3100","size":"1","offerer_order":order}})
        };
        assert_eq!(
            block_typed_data(
                &e,
                "123456",
                1793710300000,
                &trades,
                if name == "offer" {
                    Some("654321")
                } else {
                    None
                }
            )
            .unwrap(),
            vector["typed_data"]
        );
        if name == "block" {
            let mut nested = trades.clone();
            let btc = nested["BTC-USD-PERP"].take();
            nested["BTC-USD-PERP"] = json!({"trade_constraints":btc});
            assert_eq!(
                block_typed_data(&e, "123456", 1793710300000, &nested, None).unwrap(),
                vector["typed_data"]
            );
        }
    }
}
#[test]
fn reject_invalid_values_and_keys() {
    for value in ["1e-8", "-1", "0.000000001", "NaN", "", "1..2"] {
        assert!(chain_decimal(value).is_err(), "{value}");
    }
    assert_eq!(
        chain_decimal("12345678901234567890.12345678").unwrap(),
        "1234567890123456789012345678"
    );
    assert!(LocalStarkSigner::from_hex(Zeroizing::new("0x0".into())).is_err());
    assert!(LocalEvmSigner::from_hex(Zeroizing::new("invalid".into())).is_err());
}
#[test]
fn official_administrative_typed_data_vectors() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/administrative.json")).unwrap();
    for vector in fixture["vectors"].as_array().unwrap() {
        let payload = typed_payload(
            vector["account"].as_str().unwrap(),
            vector["typed_data"].clone(),
        )
        .unwrap();
        assert_eq!(
            format!(
                "{:#x}",
                starknet_core::types::Felt::from_bytes_be(&payload.digest)
            ),
            vector["digest"],
            "{}",
            vector["name"]
        );
    }
}
#[tokio::test]
async fn token_permissions_expiry_and_shared_state() {
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
    let state = TokenState::default();
    let clone = state.clone();
    let expiration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 300;
    let token = |exp| {
        Zeroizing::new(format!(
            "e30.{}.fixture",
            URL_SAFE_NO_PAD.encode(format!("{{\"exp\":{exp}}}"))
        ))
    };
    state.set(token(expiration), true).unwrap();
    assert!(clone.bearer(false).is_ok());
    assert!(clone.bearer(true).is_err());
    clone.set(token(expiration), false).unwrap();
    assert!(state.bearer(true).is_ok());
    assert!(state.set(token(1), false).is_err());
    clone.clear();
    assert!(state.bearer(false).is_err());
}
