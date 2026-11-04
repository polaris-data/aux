use aux_rs_polymarket::*;
use serde_json::Value;
#[test]
fn every_published_contract_call_matches_independent_abi_encoder() {
    let rows: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/contracts-abi.json")).unwrap();
    for row in rows {
        let encoded = encode_contract_call(
            row["interface"].as_str().unwrap(),
            row["function"].as_str().unwrap(),
            &row["args"],
        )
        .unwrap_or_else(|e| {
            if let Error::Abi(message) = e {
                panic!("{}: {message}", row["function"]);
            }
            panic!("{}: {e}", row["function"]);
        });
        assert_eq!(encoded.to_string(), row["calldata"].as_str().unwrap());
    }
}
#[test]
fn session_wallet_envelope_matches_independent_abi_encoder() {
    let row: Value = serde_json::from_str(include_str!("fixtures/session-signature.json")).unwrap();
    let signature: Bytes = row["signature"].as_str().unwrap().parse().unwrap();
    let encoded =
        wrap_session_signature(row["signer"].as_str().unwrap().parse().unwrap(), &signature)
            .unwrap();
    assert_eq!(encoded.to_string(), row["wrapped"].as_str().unwrap());
}
