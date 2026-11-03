use aux_rs_metric::{self as m, creation::*, stop_loss::*, workflows::*};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/helpers.json")).unwrap()
}
fn cases<T: DeserializeOwned, R: Serialize>(key: &str, f: impl Fn(&T) -> Result<R, m::Error>) {
    for (i, row) in fixture()[key].as_array().unwrap().iter().enumerate() {
        let request = serde_json::from_value(row["request"].clone()).unwrap();
        let result = f(&request).unwrap_or_else(|e| panic!("{key}[{i}]: {e}"));
        assert_eq!(
            serde_json::to_value(result).unwrap(),
            row["expected"],
            "{key}[{i}]"
        );
    }
}
#[test]
fn pool_creation_and_uniform_bins_match_sdk_defaults_and_calldata() {
    cases("poolCreation", prepare_create_pool);
    cases("uniformBins", create_uniform_bins);
    for row in fixture()["poolCreation"].as_array().unwrap() {
        let params = serde_json::from_value(row["request"]["params"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(prepare_factory_create_pool_params(&params).unwrap()).unwrap(),
            row["expected"]["params"]
        );
    }
}
#[test]
fn exact_weighted_self_owner_native_liquidity_match_sdk() {
    cases("liquidity", prepare_liquidity_addition);
    for row in fixture()["liquidity"].as_array().unwrap() {
        if row["request"].get("nativeValue").is_none() {
            continue;
        }
        let base: m::PreparedCall = serde_json::from_value(row["baseCall"].clone()).unwrap();
        let native = liquidity_native_payment(&base, m::U256::from(50)).unwrap();
        assert_eq!(serde_json::to_value(native).unwrap(), row["expected"]);
    }
}
#[test]
fn all_twelve_fallback_modes_and_external_payments_match_sdk() {
    cases("fallbackSwaps", prepare_swap_with_fallback);
    cases("externalSwaps", prepare_external_swap);
    for row in fixture()["fallbackSwaps"].as_array().unwrap() {
        let call: m::PreparedCall = serde_json::from_value(row["expected"].clone()).unwrap();
        let steps =
            m::decode_contract_input("MetricOmmSimpleRouter", "sequence", &call.data).unwrap();
        assert_eq!(steps[0][0][1], 1); // STOP on primary success.
        assert_eq!(steps[0][0][2], 0); // CONTINUE after primary failure.
        assert_eq!(steps[0][1][1], 1); // STOP on fallback success.
        assert_eq!(steps[0][1][2], 2); // REVERT if both fail.
    }
}
#[test]
fn per_bin_stop_loss_and_aggregate_resume_times_match_sdk() {
    cases("stopLoss", evaluate_oracle_stop_loss);
}
#[test]
fn creation_rejects_invalid_precision_bounds_and_extension_configuration() {
    let mut request: PoolCreationRequest =
        serde_json::from_value(fixture()["poolCreation"][0]["request"].clone()).unwrap();
    for distance in ["838.8608", "-838.8609", "0.00001", "NaN", "1e2"] {
        request.params.cur_bin_dist_from_provided_price_percent = distance.into();
        assert!(prepare_create_pool(&request).is_err(), "{distance}");
    }
    request.params.cur_bin_dist_from_provided_price_percent = "0".into();
    request.params.bid_ask_ref_asymmetry_x8 = 0;
    assert!(prepare_create_pool(&request).is_err());
    request.params.bid_ask_ref_asymmetry_x8 = 65535;
    request.params.minimal_operational_liquidity = m::U256::ZERO;
    assert!(prepare_create_pool(&request).is_err());
    request.params.minimal_operational_liquidity = m::U256::from(1_000_000);
    request.params.extension_orders.before_swap = m::U256::from(1);
    assert!(prepare_create_pool(&request).is_err());
    request.params.extension_orders.before_swap = m::U256::ZERO;
    request.params.admin_spread_fee_e6 = 1 << 24;
    assert!(prepare_create_pool(&request).is_err());
    request.params.admin_spread_fee_e6 = 0;
    request.helper_asymmetry_override = Some(0);
    assert!(prepare_create_pool(&request).is_err());
}
#[test]
fn generated_salt_is_inspectable_and_preserved_in_the_prepared_call() {
    let mut request: PoolCreationRequest =
        serde_json::from_value(fixture()["poolCreation"][0]["request"].clone()).unwrap();
    request.params.salt = None;
    let first = prepare_create_pool(&request).unwrap();
    let second = prepare_create_pool(&request).unwrap();
    assert_ne!(first.params.salt, second.params.salt);
    let decoded =
        m::decode_contract_input("MetricOmmPoolFactory", "createPool", &first.call.data).unwrap();
    assert_eq!(decoded[0][19], json!(first.params.salt));
    request.params.salt = Some(first.params.salt);
    assert_eq!(
        prepare_create_pool(&request).unwrap().call.data,
        first.call.data
    );
}
#[test]
fn invalid_native_payment_modes_and_fallback_amount_widths_fail_locally() {
    let mut request: FallbackSwapRequest =
        serde_json::from_value(fixture()["fallbackSwaps"][0]["request"].clone()).unwrap();
    request.payment = SwapPayment::NativeInput {
        wrapped_native: m::Address::ZERO,
    };
    assert!(prepare_swap_with_fallback(&request).is_err());
    request.payment = SwapPayment::Wrapped;
    request.call.value = m::U256::from(1);
    assert!(prepare_swap_with_fallback(&request).is_err());
    request.call.value = m::U256::ZERO;
    request.call.data = "0x1234".parse().unwrap();
    assert!(prepare_swap_with_fallback(&request).is_err());
    assert!(liquidity_native_payment(&request.call, m::U256::from(1)).is_err());
    let mut external: ExternalSwapRequest =
        serde_json::from_value(fixture()["externalSwaps"][0]["request"].clone()).unwrap();
    external.params.amount_in_maximum = m::U256::from(1) << 128usize;
    assert!(prepare_external_swap(&external).is_err());
    let mut addition: LiquidityAdditionRequest =
        serde_json::from_value(fixture()["liquidity"][0]["request"].clone()).unwrap();
    addition.kind = LiquidityAdditionKind::Weighted;
    assert!(prepare_liquidity_addition(&addition).is_err());
    addition.kind = LiquidityAdditionKind::ExactShares;
    addition.salt = m::U256::from(1) << 80usize;
    assert!(prepare_liquidity_addition(&addition).is_err());
}
#[test]
fn disabled_stop_loss_skips_price_math_and_empty_bins_default_to_skipped() {
    let mut request: StopLossEvaluationRequest =
        serde_json::from_value(fixture()["stopLoss"][0]["request"].clone()).unwrap();
    request.bid_price_x64 = m::U256::ZERO;
    request.ask_price_x64 = m::U256::ZERO;
    let result = evaluate_oracle_stop_loss(&request).unwrap();
    assert!(!result.enabled && !result.is_any_bin_stop_loss_breached);
    assert_eq!(result.seconds_until_zero_for_one_resumes, Some(0));
    request.drawdown_e6 = 100_000;
    assert!(evaluate_oracle_stop_loss(&request).is_err());
    request.drawdown_e6 = 1_000_001;
    assert!(evaluate_oracle_stop_loss(&request).is_err());
    let mut raw = fixture()["stopLoss"][0]["request"].clone();
    raw.as_object_mut().unwrap().remove("skipEmptyBins");
    let request: StopLossEvaluationRequest = serde_json::from_value(raw).unwrap();
    assert_eq!(evaluate_oracle_stop_loss(&request).unwrap().bins.len(), 2);
}

#[derive(Clone, Default)]
struct StatusRpc {
    calls: std::sync::Arc<std::sync::Mutex<Vec<Value>>>,
    fail_watermark: bool,
}
impl aux_rs_core::HttpTransport for StatusRpc {
    async fn execute(
        &self,
        _request: aux_rs_core::HttpRequest,
    ) -> Result<aux_rs_core::HttpResponse, aux_rs_core::TransportError> {
        panic!("stop-loss status must only use read transport");
    }
    async fn execute_read(
        &self,
        request: aux_rs_core::HttpRequest,
    ) -> Result<aux_rs_core::HttpResponse, aux_rs_core::TransportError> {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        self.calls.lock().unwrap().push(body.clone());
        let row = &fixture()["stopLossStatus"][0];
        let response = match body["method"].as_str().unwrap() {
            "eth_blockNumber" => json!({"result":row["blockNumber"]}),
            "eth_call" => {
                assert_eq!(
                    body["params"][1], row["blockNumber"],
                    "all reads must share one resolved block"
                );
                let matched = row["calls"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| {
                        r["to"]
                            .as_str()
                            .unwrap()
                            .eq_ignore_ascii_case(body["params"][0]["to"].as_str().unwrap())
                            && r["data"] == body["params"][0]["data"]
                    })
                    .unwrap();
                if self.fail_watermark && matched["functionName"] == "currentHighWatermarks" {
                    json!({"error":{"code":-32000,"message":"native watermark read failed"}})
                } else {
                    json!({"result":matched["result"]})
                }
            }
            other => panic!("unexpected status request {other}"),
        };
        let mut response = response;
        response["id"] = body["id"].clone();
        response["jsonrpc"] = json!("2.0");
        Ok(aux_rs_core::HttpResponse {
            status: 200,
            headers: vec![],
            body: serde_json::to_vec(&response).unwrap(),
        })
    }
}
#[tokio::test]
async fn aggregate_stop_loss_reads_one_block_and_matches_sdk_status() {
    let row = &fixture()["stopLossStatus"][0];
    let rpc = StatusRpc::default();
    let client = m::MetricChainClient::with_transport(
        m::ChainConfig::new(1, "http://read.invalid", "http://write.invalid").unwrap(),
        rpc.clone(),
    );
    let request: StopLossStatusRequest = serde_json::from_value(row["request"].clone()).unwrap();
    let result = client
        .oracle_stop_loss_status(request.clone())
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(result).unwrap(), row["expected"]);
    {
        let calls = rpc.calls.lock().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|r| r["method"] == "eth_blockNumber")
                .count(),
            1
        );
        assert_eq!(calls.len(), 8); // block + bootstrap(3) + bin arrays(2) + funded watermarks(2).
    }
    let before = rpc.calls.lock().unwrap().len();
    let mut bounded = request.clone();
    bounded.max_bins = 2;
    assert!(client.oracle_stop_loss_status(bounded).await.is_err());
    assert_eq!(rpc.calls.lock().unwrap().len() - before, 4);
    let mut invalid = request.clone();
    invalid.bid_price_x64 = invalid.ask_price_x64;
    let before = rpc.calls.lock().unwrap().len();
    assert!(client.oracle_stop_loss_status(invalid).await.is_err());
    assert_eq!(rpc.calls.lock().unwrap().len(), before);
    let client = m::MetricChainClient::with_transport(
        m::ChainConfig::new(1, "http://read.invalid", "http://write.invalid").unwrap(),
        StatusRpc {
            fail_watermark: true,
            ..Default::default()
        },
    );
    assert!(client.oracle_stop_loss_status(request).await.is_err());
}
