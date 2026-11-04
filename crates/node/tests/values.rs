#[path = "../src/value.rs"]
mod value;
use serde::Serialize;
use serde_value::Value;
#[test]
fn integer_widths_and_128_bit_limits_survive_serialization() {
    #[derive(Serialize)]
    struct Numbers {
        small: u32,
        wide: u64,
        negative: i64,
        max: u128,
        min: i128,
    }
    let result = value::to_value(Numbers {
        small: 7,
        wide: u64::MAX,
        negative: i64::MIN,
        max: u128::MAX,
        min: i128::MIN,
    })
    .unwrap();
    let Value::Map(fields) = result.clone() else {
        panic!("expected map")
    };
    assert_eq!(fields[&Value::String("small".into())], Value::U32(7));
    assert_eq!(fields[&Value::String("wide".into())], Value::U64(u64::MAX));
    assert_eq!(
        fields[&Value::String("negative".into())],
        Value::I64(i64::MIN)
    );
    assert_eq!(value::to_value(result.clone()).unwrap(), result);
}
#[test]
fn arbitrary_json_numbers_are_distinct_from_user_objects() {
    let raw: serde_json::Value = serde_json::from_str(
        r#"{"a":1,"b":18446744073709551615,"c":0.1000000000000000001,"$aux::wide":"123"}"#,
    )
    .unwrap();
    let Value::Map(fields) = value::to_value(raw).unwrap() else {
        panic!("expected map")
    };
    assert!(matches!(
        fields[&Value::String("a".into())],
        Value::Newtype(_)
    ));
    assert_eq!(
        fields[&Value::String("$aux::wide".into())],
        Value::String("123".into())
    );
    let Value::Newtype(ref number) = fields[&Value::String("c".into())] else {
        panic!("expected number")
    };
    assert_eq!(
        **number,
        Value::Seq(vec![
            Value::Bool(false),
            Value::String("0.1000000000000000001".into())
        ])
    );
}
