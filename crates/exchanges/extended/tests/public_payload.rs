use aux_rs_extended::*;
#[test]
fn public_market_payload_compatibility() {
    let _: GetMarketsResult =
        serde_json::from_str(include_str!("fixtures/public_markets.json")).unwrap();
}
