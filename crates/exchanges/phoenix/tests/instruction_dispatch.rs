use aux_rs_phoenix::build_instruction;
use serde_json::Value;
#[test]
fn every_instruction_dispatch_fixture() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/instruction_dispatch.json");
    let mut fixtures: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let update = std::env::var_os("AUX_UPDATE_INSTRUCTION_FIXTURES").is_some();
    let mut failures = Vec::new();
    for (name, fixture) in fixtures.as_object_mut().unwrap() {
        match build_instruction(name, fixture["request"].clone()) {
            Ok(actual) if update => fixture["instruction"] = actual,
            Ok(actual) => assert_eq!(fixture["instruction"], actual, "{name}"),
            Err(error) => failures.push(format!("{name}: {error}")),
        }
    }
    if update {
        std::fs::write(
            path,
            serde_json::to_string_pretty(&fixtures).unwrap() + "\n",
        )
        .unwrap();
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
