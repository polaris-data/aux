use aux_rs_metric::{self as m, planning::*};
use serde_json::Value;
#[test]
fn sdk_liquidity_plans_match_for_empty_funded_and_cursor_boundary_bins() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("fixtures/liquidity.json")).unwrap();
    for row in cases {
        let request: AdditionRequest = serde_json::from_value(row["request"].clone()).unwrap();
        let plan = addition_plan(&request).unwrap();
        assert_eq!(serde_json::to_value(plan).unwrap(), row["expected"]);
        let request: WeightedRequest =
            serde_json::from_value(row["weightedRequest"].clone()).unwrap();
        let plan = weighted_plan(&request).unwrap();
        assert_eq!(serde_json::to_value(plan).unwrap(), row["weightedExpected"]);
    }
}
#[test]
fn invalid_targets_caps_and_duplicate_bins_fail_explicitly() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("fixtures/liquidity.json")).unwrap();
    let mut request: AdditionRequest = serde_json::from_value(cases[0]["request"].clone()).unwrap();
    request.targets.push(request.targets[0].clone());
    assert!(addition_plan(&request).is_err());
    let mut request: WeightedRequest =
        serde_json::from_value(cases[0]["weightedRequest"].clone()).unwrap();
    request.max_amount_token0 = m::U256::ZERO;
    assert!(weighted_plan(&request).is_err());
}
#[test]
fn removals_preserve_percentages_cap_absolute_shares_and_skip_empty_positions() {
    let positions = vec![
        PositionShares {
            bin: 0,
            shares: m::U256::from(101),
        },
        PositionShares {
            bin: 1,
            shares: m::U256::from(20),
        },
    ];
    let specs = vec![
        RemovalSpec {
            bin: 0,
            shares_to_remove: None,
            bps: Some(5000),
        },
        RemovalSpec {
            bin: 1,
            shares_to_remove: Some(m::U256::from(100)),
            bps: None,
        },
        RemovalSpec {
            bin: 2,
            shares_to_remove: None,
            bps: Some(10000),
        },
    ];
    let deltas = removal_plan(&positions, &specs).unwrap();
    assert_eq!(deltas.bin_idxs, [0, 1]);
    assert_eq!(deltas.shares, [m::U256::from(50), m::U256::from(20)]);
    assert!(removal_plan(&positions, &[specs[0].clone(), specs[0].clone()]).is_err());
}
