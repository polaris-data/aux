use aux_rs_extended::*;
use serde_json::json;
#[tokio::test]
async fn official_order_hash_and_signature_domain_separation() {
    let order = settlement::Order {
        position_id: settlement::PositionId { value: 100 },
        base_asset_id: settlement::AssetId {
            value: felt("2").unwrap(),
        },
        base_amount: 100,
        quote_asset_id: settlement::AssetId {
            value: felt("1").unwrap(),
        },
        quote_amount: -156,
        fee_asset_id: settlement::AssetId {
            value: felt("1").unwrap(),
        },
        fee_amount: 74,
        expiration: settlement::Timestamp { seconds: 100 },
        salt: felt("123").unwrap(),
    };
    let public = "0x5d05989e9302dcebc74e241001e3e3ac3f4402ccf2f8e6f74b034b07ad6a904";
    let hash = order_hash(&Extended::testnet(), public, &order).unwrap();
    assert_eq!(
        hash,
        "0x4de4c009e0d0c5a70a7da0e2039fb2b99f376d53496f89d9f437e736add6b48"
    );
    assert_ne!(
        hash,
        order_hash(&Extended::mainnet(), public, &order).unwrap()
    );
    let signer = LocalStarkSigner::from_hex(zeroize::Zeroizing::new("0x1".into())).unwrap();
    let payload = stark_payload(&hash).unwrap();
    let signature = signer.sign(&payload).await.unwrap();
    verify_signature(&signer, &payload, &signature).unwrap();
    let mut altered = payload.clone();
    altered.digest[31] ^= 1;
    assert!(verify_signature(&signer, &altered, &signature).is_err());
}
#[test]
fn exact_order_amounts_use_native_side_and_fee_rounding() {
    let context = OrderContext {
        synthetic_asset_id: "0x4254432d3600000000000000000000".into(),
        collateral_asset_id: "0x31857064564ed0ff978e687456963cba09c2c6985d8f9300a1de4962fafa054"
            .into(),
        synthetic_resolution: 1000000,
        collateral_resolution: 1000000,
        collateral_position: 10002,
        max_position_value: None,
        quantity_precision: None,
    };
    let public = "0x61c5e7e8339b7d56f197f54ea91b776776690e3232313de0f2ecbd0ef76f466";
    let order = json!({"market":"BTC-USD","side":"BUY","qty":"0.00100000","price":"43445.11680000","fee":"0.0005","nonce":"1473459052","expiryEpochMillis":1705626537000u64});
    let prepared = prepare_order(&Extended::testnet(), public, order.clone(), &context).unwrap();
    assert_eq!(
        prepared.hash,
        "0x58454e78c25644cbcab59444736d573f169fb0996dafe1900a05e2ac50567f0"
    );
    assert_eq!(
        prepared.body["debuggingAmounts"]["collateralAmount"],
        "-43445117"
    );
    assert_eq!(prepared.body["debuggingAmounts"]["feeAmount"], "21723");
    let mut invalid = order;
    invalid["qty"] = json!("0.0000001");
    assert!(prepare_order(&Extended::testnet(), public, invalid, &context).is_err());
    assert!(scale_decimal("1.001", 2).is_err());
    assert_eq!(scale_decimal("-1.01", 2).unwrap(), -101);
}
#[test]
fn official_wallet_signature_key_derivation() {
    let signature = "0x9ef64d5936681edf44b4a7ad713f3bc24065d4039562af03fccf6a08d6996eab367df11439169b417b6a6d8ce81d409edb022597ce193916757c7d5d9cbf97301c";
    let signer =
        LocalStarkSigner::from_wallet_signature(zeroize::Zeroizing::new(signature.into())).unwrap();
    assert_eq!(
        signer.public_key(),
        "0x78298687996aff29a0bbcb994e1305db082d084f85ec38bb78c41e6787740ec"
    );
}
#[tokio::test]
async fn attached_and_position_tpsl_legs_are_signed_independently() {
    let signer = LocalStarkSigner::from_hex(zeroize::Zeroizing::new("0x1".into())).unwrap();
    let context = OrderContext {
        synthetic_asset_id: "0x2".into(),
        collateral_asset_id: "0x1".into(),
        synthetic_resolution: 1000000,
        collateral_resolution: 1000000,
        collateral_position: 10002,
        max_position_value: Some("100000".into()),
        quantity_precision: Some(6),
    };
    let order = json!({"type":"LIMIT","side":"BUY","qty":"0.001","price":"100","fee":"0.0005","nonce":"7","expiryEpochMillis":1705626537000u64,"tpSlType":"ORDER","takeProfit":{"price":"110","triggerPrice":"109","priceType":"LIMIT"},"stopLoss":{"price":"90","triggerPrice":"91","priceType":"MARKET"}});
    let signed = sign_order(&Extended::testnet(), &signer, order.clone(), &context)
        .await
        .unwrap();
    assert_ne!(
        signed["settlement"]["signature"],
        signed["takeProfit"]["settlement"]["signature"]
    );
    assert_ne!(
        signed["takeProfit"]["settlement"]["signature"],
        signed["stopLoss"]["settlement"]["signature"]
    );
    assert_eq!(
        signed["takeProfit"]["debuggingAmounts"]["syntheticAmount"],
        "-1000"
    );
    let mut position = order;
    position["type"] = json!("TPSL");
    position["tpSlType"] = json!("POSITION");
    position["qty"] = json!("0");
    position["price"] = json!("0");
    let signed = sign_order(&Extended::testnet(), &signer, position, &context)
        .await
        .unwrap();
    assert!(signed["settlement"].is_null());
    assert!(signed["takeProfit"]["settlement"]["signature"].is_object());
}

// Compatibility vectors from the pinned official SDK's authentication tests.
#[tokio::test]
async fn official_wallet_onboarding_and_api_signatures() {
    let signer = LocalEvmSigner::from_hex(zeroize::Zeroizing::new(
        "50c8e358cc974aaaa6e460641e53f78bdc550fd372984aa78ef8fd27c751e6f4".into(),
    ))
    .unwrap();
    let payload = api_request_payload("/action", "2024-01-05T01:08:56Z").unwrap();
    assert_eq!(
        payload.personal_message.as_deref(),
        Some("/action@2024-01-05T01:08:56Z")
    );
    assert_eq!(
        signer.sign(&payload).await.unwrap()[0],
        "0xf4e4e9aaf2014a3651dfafec63854e4dfd486dcc10e77f56b330e9942630fde03588e43d6c022f8513c1e4cf211e670c3134d3cfdf1bd61b570d2588bfb9fc921b"
    );
    let exchange = Extended::custom("https://example.com", "wss://example.com", "SN_MAIN").unwrap();
    let registration = json!({"signingDomain":"x10.exchange","accountIndex":0,"wallet":"0x2c12f074766f5eF9c5300ca8C85d06fBa605C59f","tosAccepted":true,"time":"2024-07-30T16:01:02Z","action":"REGISTER","host":"host"});
    let payload = onboarding_payload(&exchange, &registration).unwrap();
    assert_eq!(
        signer.sign(&payload).await.unwrap()[0],
        "0x9a59eb699eb58f2ec975455f33dd7205c8a569f7b6d7647c25b71e7ab7eec3d30f2b8c9038f06f077167eb90e0c002602e4ecbab180fad4b2c91d2259883e6571c"
    );
    let creation = key_derivation_payload(&exchange, &registration).unwrap();
    assert_eq!(
        signer.sign(&creation).await.unwrap()[0],
        "0x9ef64d5936681edf44b4a7ad713f3bc24065d4039562af03fccf6a08d6996eab367df11439169b417b6a6d8ce81d409edb022597ce193916757c7d5d9cbf97301c"
    );
    let stark = LocalStarkSigner::from_hex(zeroize::Zeroizing::new(
        "0x7dbb2c8651cc40e1d0d60b45eb52039f317a8aa82798bda52eee272136c0c44".into(),
    ))
    .unwrap();
    let hash = onboarding_l2_hash(
        registration["wallet"].as_str().unwrap(),
        &stark.public_key(),
    )
    .unwrap();
    let signature = stark.sign(&stark_payload(&hash).unwrap()).await.unwrap();
    assert_eq!(
        signature,
        vec![
            felt("0x70881694c59c7212b1a47fbbc07df4d32678f0326f778861ec3a2a5dbc09157")
                .unwrap()
                .to_string(),
            felt("0x558805193faa5d780719cba5f699ae1c888eec1fee23da4215fdd94a744d2cb")
                .unwrap()
                .to_string()
        ]
    );
    assert_ne!(
        key_derivation_payload(&Extended::mainnet(), &registration)
            .unwrap()
            .digest,
        key_derivation_payload(&Extended::testnet(), &registration)
            .unwrap()
            .digest
    );
    assert!(api_request_payload("/action\r\nInjected: x", "now").is_err());
    assert!(onboarding_payload(&exchange, &json!({"accountIndex":0})).is_err());
}
