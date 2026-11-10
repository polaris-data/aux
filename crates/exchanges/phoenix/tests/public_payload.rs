use aux_rs_phoenix::*;
#[test]
fn public_market_payload_compatibility() {
    let _: Vec<ExchangeMarketConfig> =
        serde_json::from_str(include_str!("fixtures/public_markets.json")).unwrap();
}
