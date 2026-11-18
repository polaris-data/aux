use crate::{Error, Result};
use alloy_primitives::{B256, Bytes, I256, U256, hex};
use alloy_sol_types::SolValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
/// Decoded native fields. This is structural decoding, never signature verification.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DecodedReport {
    Known {
        version: u16,
        full_report: String,
        report_body: String,
        fields: BTreeMap<String, Value>,
    },
    Unknown {
        version: u16,
        full_report: String,
        report_body: String,
    },
}
// Common ABI header followed by each schema's native fields, in wire order.
const COMMON: &[(&str, &str)] = &[
    ("feedId", "bytes32"),
    ("validFromTimestamp", "uint32"),
    ("observationsTimestamp", "uint32"),
    ("nativeFee", "uint192"),
    ("linkFee", "uint192"),
    ("expiresAt", "uint32"),
];
fn fields(version: u16) -> Option<&'static [(&'static str, &'static str)]> {
    Some(match version {
        2 => &[("price", "int192")],
        3 => &[("price", "int192"), ("bid", "int192"), ("ask", "int192")],
        7 => &[("exchangeRate", "int192")],
        8 => &[
            ("lastUpdateTimestamp", "uint64"),
            ("midPrice", "int192"),
            ("marketStatus", "uint32"),
        ],
        9 => &[
            ("navPerShare", "int192"),
            ("navDate", "uint64"),
            ("aum", "int192"),
            ("ripcord", "uint32"),
        ],
        10 => &[
            ("lastUpdateTimestamp", "uint64"),
            ("price", "int192"),
            ("marketStatus", "uint32"),
            ("currentMultiplier", "int192"),
            ("newMultiplier", "int192"),
            ("activationDateTime", "uint32"),
            ("tokenizedPrice", "int192"),
        ],
        11 => &[
            ("mid", "int192"),
            ("lastSeenTimestampNs", "uint64"),
            ("bid", "int192"),
            ("bidVolume", "int192"),
            ("ask", "int192"),
            ("askVolume", "int192"),
            ("lastTradedPrice", "int192"),
            ("marketStatus", "uint32"),
        ],
        14 => &[
            ("midPrice", "int192"),
            ("bidPrice", "int192"),
            ("askPrice", "int192"),
            ("expiryTime", "uint64"),
            ("firstDayOfNotice", "uint64"),
            ("lastSeenTimestampNs", "uint64"),
            ("marketStatus", "uint32"),
            ("contractMonth", "uint32"),
            ("goldmanRollPrice", "int192"),
            ("currentBusinessDay", "uint32"),
            ("interpolatedGoldmanRollPrice", "int192"),
        ],
        _ => return None,
    })
}
/// Decode ABI `(bytes32[3],bytes,bytes32[],bytes32[],bytes32)` report envelope.
/// The original hex is retained verbatim. Unsupported versions remain inspectable.
pub fn decode_report(full_report: &str) -> Result<DecodedReport> {
    if full_report.len() > 2 * aux_rs_data::MAX_RECORD_BYTES + 2 {
        return Err(Error::Decode("report exceeds configured limit"));
    }
    let bytes = hex::decode(full_report).map_err(|_| Error::Decode("invalid report hex"))?;
    type Envelope = ([B256; 3], Bytes, Vec<B256>, Vec<B256>, B256);
    let envelope = Envelope::abi_decode_params(&bytes)
        .map_err(|_| Error::Decode("invalid report ABI envelope"))?;
    if envelope.abi_encode_params() != bytes
        || envelope.2.len() != envelope.3.len()
        || envelope.2.len() > 32
    {
        return Err(Error::Decode("noncanonical report envelope"));
    }
    let body = envelope.1.as_ref();
    if body.len() < 32 {
        return Err(Error::Decode("report body has no feed ID"));
    }
    let version = u16::from_be_bytes([body[0], body[1]]);
    let report_body = hex::encode_prefixed(body);
    let Some(tail) = fields(version) else {
        return Ok(DecodedReport::Unknown {
            version,
            full_report: full_report.into(),
            report_body,
        });
    };
    if body.len() != (COMMON.len() + tail.len()) * 32 {
        return Err(Error::Decode("report body does not match schema length"));
    }
    let mut decoded = BTreeMap::new();
    for ((name, kind), word) in COMMON.iter().chain(tail).zip(body.chunks_exact(32)) {
        let value = match *kind {
            "bytes32" => Value::String(hex::encode_prefixed(word)),
            "int192" => {
                let extension = if word[8] & 0x80 == 0 { 0 } else { 0xff };
                if word[..8].iter().any(|b| *b != extension) {
                    return Err(Error::Decode("invalid int192 sign extension"));
                }
                Value::String(I256::from_raw(U256::from_be_slice(word)).to_string())
            }
            kind => {
                let width = kind
                    .strip_prefix("uint")
                    .and_then(|n| n.parse::<usize>().ok())
                    .ok_or(Error::Decode("unknown ABI type"))?;
                if word[..32 - width / 8].iter().any(|b| *b != 0) {
                    return Err(Error::Decode("noncanonical ABI integer"));
                }
                let n = U256::from_be_slice(word);
                if width <= 32 {
                    Value::from(n.to::<u32>())
                } else {
                    Value::String(n.to_string())
                }
            }
        };
        decoded.insert((*name).into(), value);
    }
    Ok(DecodedReport::Known {
        version,
        full_report: full_report.into(),
        report_body,
        fields: decoded,
    })
}
