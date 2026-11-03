use aux_rs_paradex::sbe;
use serde_json::Value;
fn fixtures() -> Value {
    serde_json::from_str(include_str!("fixtures/sbe.json")).unwrap()
}
fn bytes(value: &Value) -> Vec<u8> {
    let text = value["hex"].as_str().unwrap();
    let mut bytes = vec![0; text.len() / 2];
    aux_rs_core::hex::decode_to_slice(text, &mut bytes).unwrap();
    bytes
}
#[test]
fn all_templates_preserve_native_wire_values() {
    let fixtures = fixtures();
    let frames = fixtures["frames"].as_array().unwrap();
    assert_eq!(frames.len(), 12);
    for frame in frames {
        let decoded = sbe::decode(&bytes(frame)).unwrap();
        assert_eq!(
            decoded.template_id(),
            frame["template"].as_u64().unwrap() as u16
        );
        let native = decoded.data().unwrap();
        assert_eq!(native, frame["native"], "{}", frame["name"]);
        if let Some(timestamp) = native["ts"].as_i64()
            && let Some(reference) = frame["reference"]["timestamp"].as_i64()
        {
            assert_eq!(timestamp / 1000, reference);
        }
        if frame["template"] == 1 {
            assert_eq!(native["tradeIdStr"], frame["reference"]["trade_id_str"]);
            assert_eq!(native["tradeIdStr"], "1234567890123456789012345678");
            assert_eq!(native["price"], frame["reference"]["price"]);
        }
        if frame["template"] == 21 {
            assert!(native["seq"].is_null());
        }
        if frame["template"] == 5 {
            assert_eq!(native["fundingRate"], frame["reference"]["funding_rate"]);
        }
    }
}
#[test]
fn truncated_malformed_and_unsupported_frames_are_errors() {
    for frame in fixtures()["frames"].as_array().unwrap() {
        let bytes = bytes(frame);
        for end in 0..bytes.len() {
            assert!(
                sbe::decode(&bytes[..end]).is_err(),
                "{} truncated at {end}",
                frame["name"]
            );
        }
        for (offset, value) in [(2, 99u8), (4, 99), (6, 99)] {
            let mut corrupted = bytes.clone();
            corrupted[offset] = value;
            assert!(sbe::decode(&corrupted).is_err());
        }
        let mut corrupted = bytes.clone();
        corrupted[..2].copy_from_slice(&0u16.to_le_bytes());
        assert!(sbe::decode(&corrupted).is_err());
    }
}
#[test]
fn block_length_controls_variable_data_offsets() {
    for frame in fixtures()["frames"].as_array().unwrap() {
        let mut bytes = bytes(frame);
        let block = u16::from_le_bytes(bytes[..2].try_into().unwrap());
        bytes.splice(8 + usize::from(block)..8 + usize::from(block), [0; 8]);
        bytes[..2].copy_from_slice(&(block + 8).to_le_bytes());
        assert_eq!(
            sbe::decode(&bytes).unwrap().data().unwrap(),
            frame["native"]
        );
    }
}
#[test]
fn rejects_corrupt_group_counts_and_utf8() {
    let fixtures = fixtures();
    let frames = fixtures["frames"].as_array().unwrap();
    let frame = frames.iter().find(|f| f["template"] == 3).unwrap();
    let mut bytes = bytes(frame);
    let block = u16::from_le_bytes(bytes[..2].try_into().unwrap());
    let offset = 8 + usize::from(block);
    bytes[offset + 2..offset + 4].copy_from_slice(&u16::MAX.to_le_bytes());
    assert!(sbe::decode(&bytes).is_err());
    let mut bytes = super_bytes(frame);
    *bytes.last_mut().unwrap() = 0xff;
    assert!(sbe::decode(&bytes).is_err());
}
fn super_bytes(frame: &Value) -> Vec<u8> {
    bytes(frame)
}
