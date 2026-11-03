use aux_rs_paradex::ResponsesSystemConfigResponse;
#[test]
fn preserve_exact_unknown_numbers() {
    let fixtures: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let mut text = serde_json::to_string(&fixtures["get_system_config"]["response"]).unwrap();
    text.pop();
    text.push_str(",\"exact\":1.00000000000000000001,\"id\":1234567890123456789012345678}");
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    let parsed: ResponsesSystemConfigResponse = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), value);
    let envelope = format!("{{\"result\":{text}}}");
    let event = aux_rs_paradex::Event::decode(&envelope).unwrap();
    assert_eq!(
        event.fields["result"]["id"].to_string(),
        "1234567890123456789012345678"
    );
}
