use aux_rs_polymarket::*;
use serde_json::{Value, json};
use zeroize::Zeroizing;
#[tokio::test]
async fn eip712_and_signatures_match_independent_references() {
    let rows: Vec<Value> = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    let signer = LocalEvmSigner::from_hex(Zeroizing::new("07".repeat(32))).unwrap();
    for row in rows.iter().filter(|r| r["name"] != "hmac") {
        let td = &row["typed_data"];
        assert_eq!(
            typed_data_hash(td).unwrap().to_string(),
            row["digest"].as_str().unwrap()
        );
        assert_eq!(
            sign_typed_data(&signer, td).await.unwrap().to_string(),
            row["signature"].as_str().unwrap()
        );
        if let Some(op) = row.get("operation") {
            let actual = perps_op_typed_data(137, op, 42, 1731536000000).unwrap();
            assert_eq!(
                typed_data_hash(&actual).unwrap(),
                typed_data_hash(td).unwrap()
            );
        }
        if row["name"].as_str().unwrap().starts_with("order_") {
            let order = if td["primaryType"] == "TypedDataSign" {
                &td["message"]["contents"]
            } else {
                &td["message"]
            };
            let version = td["domain"]["version"].as_str().unwrap().parse().unwrap();
            let domain = OrderDomain {
                chain_id: 137,
                exchange: td["domain"]["verifyingContract"]
                    .as_str()
                    .unwrap()
                    .parse()
                    .unwrap(),
                version,
            };
            let native = order_typed_data(order, domain).unwrap();
            assert_eq!(
                sign_order(&signer, order, domain).await.unwrap(),
                row["order_signature"]
            );
            assert_eq!(
                typed_data_hash(&native).unwrap(),
                typed_data_hash(td).unwrap()
            );
        }
    }
}
#[test]
fn hmac_is_exact_and_padded_urlsafe() {
    let rows: Vec<Value> = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    let r = rows.iter().find(|r| r["name"] == "hmac").unwrap();
    let auth = ApiCredentials::new(
        "fixture".into(),
        Zeroizing::new(r["secret"].as_str().unwrap().into()),
        Zeroizing::new("public-passphrase".into()),
    )
    .unwrap();
    assert_eq!(
        auth.signature(
            1731536000,
            "POST",
            "/order",
            br#"{"price":"0.550000","size":"10"}"#
        )
        .unwrap(),
        r["signature"]
    );
}
#[test]
fn asset_ids_and_domains_are_versioned() {
    let market = json!({"version":"v2","outcomes":"[\"Yes\",\"No\"]","positionIds":["123456789012345678901234567890","2"],"clobTokenIds":"[\"3\",\"4\"]","negRisk":false});
    assert_eq!(
        market_asset_id(&market, "Yes").unwrap(),
        "123456789012345678901234567890"
    );
    assert_eq!(market_order_domain(&market).unwrap().version, 3);
    let mut unknown = market;
    unknown["version"] = json!("future");
    assert!(market_asset_id(&unknown, "Yes").is_err());
}
#[test]
fn native_perps_arrays_match_json_envelopes() {
    let raw = json!({"type":"createOrders","args":[{"iid":1,"buy":true,"p":"100.00","qty":"10.00","tif":"gtc","po":false}]});
    assert_eq!(
        perps_signable_operation(&raw).unwrap(),
        json!(["createOrders", [[1, true, "100.00", "10.00", "gtc", false]]])
    );
}
#[tokio::test]
async fn native_owner_proxy_authority_and_operation_bytes_match_references() {
    let signer =
        std::sync::Arc::new(LocalEvmSigner::from_hex(Zeroizing::new("07".repeat(32))).unwrap());
    let owner = PolymarketPerpsAuth {
        owner: Some(signer.clone()),
        proxy: None,
        credentials: None,
        withdrawal_contract: Some(
            "0xDCa4af75705dbB50f62437045afF9921947917d2"
                .parse()
                .unwrap(),
        ),
    };
    let proxy = PolymarketPerpsAuth {
        owner: None,
        proxy: Some(signer.clone()),
        credentials: Some(
            PerpsCredentials::new(
                signer.address(),
                Zeroizing::new("offline-secret".into()),
                None,
            )
            .unwrap(),
        ),
        withdrawal_contract: None,
    };
    let rows: Vec<Value> = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    for row in rows.iter().filter(|r| r.get("body").is_some()) {
        let mut body = row["body"].clone();
        let auth = if row["authority"] == "owner" {
            &owner
        } else {
            &proxy
        };
        auth.sign_body(&Polymarket::mainnet(), &mut body)
            .await
            .unwrap();
        assert_eq!(body["sig"], row["signature"]);
    }
    let mut builder = json!({"op":{"type":"updateBuilder","args":{"name":"test"}}});
    assert!(
        proxy
            .sign_body(&Polymarket::mainnet(), &mut builder)
            .await
            .is_err()
    );
    let mut trade = json!({"op":{"type":"createOrders","args":[]}});
    assert!(
        owner
            .sign_body(&Polymarket::mainnet(), &mut trade)
            .await
            .is_err()
    );
    let expired = PolymarketPerpsAuth {
        credentials: Some(
            PerpsCredentials::new(
                signer.address(),
                Zeroizing::new("offline-secret".into()),
                Some(0),
            )
            .unwrap(),
        ),
        ..proxy
    };
    assert!(
        expired
            .sign_body(&Polymarket::mainnet(), &mut trade)
            .await
            .is_err()
    );
}
#[tokio::test]
async fn wallet_auth_refreshes_owned_headers_and_honors_explicit_nonce() {
    use aux_rs_core::{HttpMethod, HttpRequest};
    let signer =
        std::sync::Arc::new(LocalEvmSigner::from_hex(Zeroizing::new("07".repeat(32))).unwrap());
    let auth = PolymarketAuth::new(signer.clone(), signer.address());
    let operation:Operation=serde_json::from_value(json!({"service":"clob","client":"PolymarketClient","method":"derive","verb":"GET","path":"/auth/derive-api-key","auth":"wallet","safe":false,"parameters":[],"has_body":false,"fixture":"wallet"})).unwrap();
    let mut request = HttpRequest {
        method: HttpMethod::Get,
        url: "https://clob.polymarket.com/auth/derive-api-key".into(),
        headers: vec![
            ("poly_nonce".into(), "7".into()),
            ("POLY_SIGNATURE".into(), "stale".into()),
            ("poly_timestamp".into(), "1".into()),
        ],
        body: vec![],
    };
    auth.prepare(&Polymarket::mainnet(), &operation, &mut request)
        .await
        .unwrap();
    let get = |name: &str| {
        request
            .headers
            .iter()
            .filter(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v)
            .collect::<Vec<_>>()
    };
    assert_eq!(get("POLY_NONCE"), ["7"]);
    assert_eq!(get("POLY_TIMESTAMP").len(), 1);
    assert_eq!(get("POLY_SIGNATURE").len(), 1);
    assert_ne!(get("POLY_SIGNATURE")[0], "stale");
}
