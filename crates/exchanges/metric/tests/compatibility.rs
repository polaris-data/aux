use aux_rs_metric::{self as m, math::*};
use serde_json::{Value, json};
use zeroize::Zeroizing;
fn fixtures() -> Vec<Value> {
    serde_json::from_str(include_str!("fixtures/abi.json")).unwrap()
}
fn normalize(value: Value) -> Value {
    match value {
        Value::Number(n) => Value::String(n.to_string()),
        Value::Array(v) => Value::Array(v.into_iter().map(normalize).collect()),
        Value::Object(o) => Value::Object(o.into_iter().map(|(k, v)| (k, normalize(v))).collect()),
        v => v,
    }
}
#[test]
fn every_pinned_abi_matches_independent_viem_vectors() {
    let rows = fixtures();
    assert!(rows.len() > 900);
    for row in rows {
        let c = row["contract"].as_str().unwrap();
        let s = row["signature"].as_str().unwrap();
        match row["kind"].as_str().unwrap() {
            "function" => {
                let actual = m::encode_contract_call(c, s, &row["args"])
                    .unwrap_or_else(|e| panic!("{c}.{s}: {e}"));
                assert_eq!(
                    actual.to_string(),
                    row["input"].as_str().unwrap(),
                    "{c}.{s}"
                );
                let data: m::Bytes = row["output"].as_str().unwrap().parse().unwrap();
                let decoded = m::decode_contract_output(c, s, &data)
                    .unwrap_or_else(|e| panic!("{c}.{s}: {e}"));
                assert_eq!(normalize(decoded), row["outputArgs"], "{c}.{s}");
            }
            "error" => {
                let data: m::Bytes = row["input"].as_str().unwrap().parse().unwrap();
                let decoded =
                    m::decode_contract_error(c, &data).unwrap_or_else(|e| panic!("{c}.{s}: {e}"));
                assert_eq!(
                    normalize(decoded["arguments"].clone()),
                    row["args"],
                    "{c}.{s}"
                );
            }
            "event" => {
                let topics: Vec<m::B256> = serde_json::from_value(row["topics"].clone()).unwrap();
                let data: m::Bytes = row["data"].as_str().unwrap().parse().unwrap();
                let decoded = m::decode_contract_event(c, s, &topics, &data)
                    .unwrap_or_else(|e| panic!("{c}.{s}: {e}"));
                assert_eq!(
                    normalize(decoded["indexed"].clone()),
                    row["indexed"],
                    "{c}.{s}"
                );
                assert_eq!(normalize(decoded["body"].clone()), row["body"], "{c}.{s}");
            }
            _ => unreachable!(),
        }
    }
}
#[tokio::test]
async fn legacy_and_eip1559_match_ethers_on_each_public_chain() {
    use m::EvmSigner;
    let rows: Vec<Value> = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    for row in rows {
        let signer =
            m::LocalEvmSigner::from_hex(Zeroizing::new(row["key"].as_str().unwrap().into()))
                .unwrap();
        assert_eq!(format!("{:#x}", signer.address()), row["address"]);
        let request: m::PreparedTransaction =
            serde_json::from_value(row["prepared"].clone()).unwrap();
        let signed = request.sign(&signer).await.unwrap();
        assert_eq!(signed.bytes.to_string(), row["signed"]);
        assert_eq!(format!("{:#x}", signed.hash), row["hash"]);
    }
}
#[test]
fn sdk_math_vectors_and_precision_edges() {
    let v: Value = serde_json::from_str(include_str!("fixtures/math.json")).unwrap();
    let bins: Vec<BinData> = serde_json::from_value(v["bins"].clone()).unwrap();
    let packed = pack_bin_data_array(&bins);
    let expected: Vec<m::U256> = v["packed"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().parse().unwrap())
        .collect();
    assert_eq!(packed, expected);
    assert_eq!(unpack_bin_data_array(&packed), bins);
    for row in v["stoploss"].as_array().unwrap() {
        let price = row["priceE18"].as_str().unwrap().parse().unwrap();
        let (a, b) = m::liquidity::recommended_initial_scaled_amounts(price).unwrap();
        assert_eq!(a.to_string(), row["initialScaledAmount0PerE18Shares"]);
        assert_eq!(b.to_string(), row["initialScaledAmount1PerE18Shares"]);
    }
    for row in v["cursor"].as_array().unwrap() {
        let input = &row["input"];
        let bounds = m::liquidity::liquidity_cursor_bounds(&m::liquidity::CursorInput {
            cur_bin_idx: input["curBinIdx"].as_i64().unwrap() as i16,
            cur_pos_in_bin: input["curPosInBin"].as_str().unwrap().parse().unwrap(),
            lowest_bin: -1,
            highest_bin: 1,
            bin_lengths_e6: vec![10000, 5000, 1000],
            distance_e12: row["distanceE12"].as_str().unwrap().parse().unwrap(),
        })
        .unwrap();
        let out = &row["output"];
        assert_eq!(
            bounds.minimal_cur_bin,
            i16::try_from(out["minimalCurBin"].as_i64().unwrap()).unwrap()
        );
        assert_eq!(
            bounds.maximal_cur_bin,
            i16::try_from(out["maximalCurBin"].as_i64().unwrap()).unwrap()
        );
        assert_eq!(bounds.minimal_position.to_string(), out["minimalPosition"]);
        assert_eq!(bounds.maximal_position.to_string(), out["maximalPosition"]);
    }
    assert!(decimal_to_raw("1.0000001", 6).is_err());
    assert_eq!(
        decimal_to_raw("1.230000", 6).unwrap(),
        m::U256::from(1_230_000)
    );
    assert!(mul_div(m::U256::MAX, m::U256::MAX, m::U256::from(1), Rounding::Down).is_err());
    assert!(
        mul_div(
            m::U256::from(1),
            m::U256::from(1),
            m::U256::from(3),
            Rounding::Exact
        )
        .is_err()
    );
    assert_eq!(unpack_two_int128(m::U256::MAX), (-1, -1));
    let state = BinState {
        token0_balance_scaled: m::U256::from(17),
        token1_balance_scaled: m::U256::from(23),
        length_e6: 15,
        add_fee_buy_e6: 65535,
        add_fee_sell_e6: 7,
    };
    assert_eq!(decode_bin_state(encode_bin_state(&state).unwrap()), state);
}
#[test]
fn abi_inputs_reject_rounding_overloads_and_trailing_type_mistakes() {
    assert!(
        m::encode_contract_call(
            "MetricOmmPool",
            "setBinAdditionalFees",
            &json!([0, 65536, 0])
        )
        .is_err()
    );
    assert!(m::encode_contract_call("MetricOmmPool", "extsload", &json!([])).is_err());
    assert!(
        m::encode_contract_call(
            "MetricOmmSimpleRouter",
            "exactInputSingle",
            &json!({"params":{"amountIn":"1.2"}})
        )
        .is_err()
    );
    assert!(m::decode_contract_event("MetricOmmPool", "Swap", &[], &[]).is_err());
}

#[tokio::test]
async fn oracle_packets_and_signatures_match_pinned_pusher() {
    use m::oracle::*;
    let row: Value = serde_json::from_str(include_str!("fixtures/oracle.json")).unwrap();
    let lanes = serde_json::from_value(row["lanes"].clone()).unwrap();
    let word = pack_oracle_slot(
        &lanes,
        row["timestampMs"].as_u64().unwrap(),
        row["slotId"].as_u64().unwrap() as u8,
        Rounding::Down,
    )
    .unwrap();
    assert_eq!(
        word,
        row["word"].as_str().unwrap().parse::<m::U256>().unwrap()
    );
    let creator = row["creator"].as_str().unwrap().parse().unwrap();
    let oracle = row["oracle"].as_str().unwrap().parse().unwrap();
    let id = feed_id(creator, m::U256::from(1), 255, 3).unwrap();
    assert_eq!(format!("{id:#x}"), row["feedId"]);
    let digest = slot_update_digest(m::U256::from(1), oracle, creator, word);
    assert_eq!(format!("{digest:#x}"), row["updateDigest"]);
    let consent = pusher_consent_digest(
        m::U256::from(1),
        oracle,
        m::U256::from(row["deadline"].as_u64().unwrap()),
        row["pusher"].as_str().unwrap().parse().unwrap(),
        creator,
    );
    assert_eq!(format!("{consent:#x}"), row["consentDigest"]);
    let signer = m::LocalEvmSigner::from_hex(Zeroizing::new(format!("{:064x}", 1))).unwrap();
    let signature = sign_personal_digest(&signer, digest).await.unwrap();
    assert_eq!(format!("{signature}"), row["signature"]);
    assert_eq!(oracle_push_calldata(&[word]).unwrap().len(), 33);
    assert!(pack_oracle_slot(&lanes, 1, 0, Rounding::Exact).is_err());
}
