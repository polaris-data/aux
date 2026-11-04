use aux_rs_pascal::*;
use serde_json::{Value, json};
use zeroize::Zeroizing;
#[tokio::test]
async fn permits_match_published_vectors() {
    let rows: Vec<Value> = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    for row in rows {
        let wallet = row["signer_role"] == "owner_wallet";
        let signer = LocalSigner::from_hex(Zeroizing::new(
            row["private_key_hex"].as_str().unwrap().into(),
        ))
        .unwrap();
        let context = PermitContext {
            deployment_id: 3,
            owner: "GmaDrppBC7P5ARKV8g3djiwP89vz1jLK23V2GBjuAEGB".into(),
            signer: row["public_key"].as_str().unwrap().into(),
            client_ts_ms: 1731536000000,
            recv_window_ms: 5000,
        };
        let (name, request) = match row["permit"].as_str().unwrap() {
            "PlaceOrderPermit" => (
                "place_orders",
                json!({"size":"10","price":"0.550000","client_order_id":"42","replace_client_order_id":"43","expires_ts_ms":"1731539600000","symbol":"SIM_EVENT_1.MARKET_1","side":"BID","tif":"GTT","post_only":true,"reduce_only":true,"allow_missing_replace":true}),
            ),
            "CancelOrderPermit" => ("cancel_orders", json!({"client_order_id":"42"})),
            "MassCancelPermit" => ("mass_cancel", json!({"symbol":"SIM_EVENT_1.MARKET_1"})),
            "CreateTradingKeyPermit" => (
                "create_trading_key",
                json!({"name":"api-doc-key","trading_key":"2KW2XRd9kwqet15Aha2oK3tYvd3nWbTFH1MBiRAv1BE1","expiration_ts_ms":"1732140800000"}),
            ),
            "RevokeTradingKeyPermit" => (
                "revoke_trading_key",
                json!({"trading_key":"2KW2XRd9kwqet15Aha2oK3tYvd3nWbTFH1MBiRAv1BE1"}),
            ),
            "SignedWithdrawalPermit" => (
                "withdrawal",
                json!({"amount":"25.000000","destination_authority":"29d2S7vB453rNYFdR5Ycwt7y9haRT5fwVwL9zTmBhfV2","destination_token_account":"3JF3sEqM796hk5WFqA6EtmEwJQ9quALszsfJyvXNQKy3"}),
            ),
            "RegisterDepositAddressReferralCodePermit" => (
                "register_deposit_address",
                json!({"referral_code":"k7wq2m4x"}),
            ),
            "GenerateReferralCodePermit" => ("generate_referral_code", json!({})),
            "ClaimCustomReferralCodePermit" => (
                "claim_custom_referral_code",
                json!({"referral_code":"trader1"}),
            ),
            "ApplyReferralCodePermit" => {
                ("apply_referral_code", json!({"referral_code":"k7wq2m4x"}))
            }
            "SetUsernamePermit" => ("set_username", json!({"username":"Trader"})),
            "SetAccountBioPermit" => ("set_account_bio", json!({"account_bio":r#"verify "🙂""#})),
            _ => panic!("unmapped published permit"),
        };
        let expected = if let Some(hex) = row["message_hex"].as_str() {
            aux_rs_core::hex::decode(hex).unwrap()
        } else {
            row["message_utf8"].as_str().unwrap().as_bytes().to_vec()
        };
        let actual = permit_message(name, &request, &context).unwrap();
        assert_eq!(actual, expected, "{} wallet={wallet}", row["permit"]);
        let signature = signer.sign(&actual).await.unwrap();
        assert_eq!(bs58::encode(signature).into_string(), row["signature"]);
        let mut other = context;
        other.deployment_id = 2;
        assert_ne!(permit_message(name, &request, &other).unwrap(), actual);
    }
}
#[tokio::test]
async fn solana_transaction_matches_official_sdk() {
    let v: Value = serde_json::from_str(include_str!("fixtures/solana.json")).unwrap();
    let text = |k: &str| v[k].as_str().unwrap();
    let p = PreparedDeposit::new(
        text("owner"),
        text("source"),
        text("destination"),
        text("mint"),
        25_000_000,
        6,
        text("blockhash"),
        999,
    )
    .unwrap();
    assert_eq!(aux_rs_core::hex::encode(&p.message), text("message_hex"));
    let signer = LocalSigner::new(Zeroizing::new([7; 32]));
    let signed = p.sign(&signer).await.unwrap();
    assert_eq!(signed.transaction_base64, text("transaction_base64"));
    assert_eq!(signed.signature, text("signature"));
    assert_eq!(
        associated_token_address(text("owner"), text("mint")).unwrap(),
        text("associated_token_address")
    );
}
#[test]
fn exact_decimal_and_replacement_validation() {
    assert_eq!(micro_usd("0.000001").unwrap(), 1);
    assert!(micro_usd("0.0000001").is_err());
    assert!(micro_usd("1e-6").is_err());
    assert!(micro_usd("18446744073709551616").is_err());
}
