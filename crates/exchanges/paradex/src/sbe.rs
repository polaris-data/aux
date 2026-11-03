//! Bounds-checked decoding of the pinned Paradex schema 1:3. No floating point.
use crate::Error;
use serde_json::{Map, Value, json};
#[path = "sbe_layout.rs"]
mod layout;
#[path = "sbe_models.rs"]
mod models;
pub use models::*;
#[derive(Clone, Copy)]
enum Kind {
    Integer { bytes: usize, signed: bool },
    OptionalI64,
    Timestamp,
    Address,
    Decimal { places: usize, nullable: bool },
    Enum(&'static [(u64, &'static str)]),
    Flags(&'static [(u64, &'static str)]),
}
struct Field {
    name: &'static str,
    offset: usize,
    size: usize,
    kind: Kind,
}
type Variable = (&'static str, Option<(usize, &'static [Field])>);
struct Layout {
    template: u16,
    minimum: usize,
    fields: &'static [Field],
    variable: &'static [Variable],
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DecodedFrame {
    pub schema_id: u16,
    pub schema_version: u16,
    pub block_length: u16,
    #[serde(flatten)]
    pub message: SbeMessage,
}
impl DecodedFrame {
    pub fn template_id(&self) -> u16 {
        match &self.message {
            SbeMessage::TradeEvent(_) => 1,
            SbeMessage::BboEvent(_) => 2,
            SbeMessage::BookEvent(_) => 3,
            SbeMessage::MarketSummaryEvent(_) => 4,
            SbeMessage::FundingDataEvent(_) => 5,
            SbeMessage::FundingRateComparisonEvent(_) => 6,
            SbeMessage::OrderEvent(_) => 20,
            SbeMessage::FillEvent(_) => 21,
            SbeMessage::PositionEvent(_) => 22,
            SbeMessage::AccountEvent(_) => 23,
            SbeMessage::HeartbeatEvent(_) => 40,
            SbeMessage::SubscribedEvent(_) => 41,
        }
    }
    pub fn data(&self) -> Result<Value, Error> {
        Ok(serde_json::to_value(&self.message)?["data"].take())
    }
}
fn malformed() -> Error {
    Error::InvalidRequest("malformed Paradex SBE frame")
}
fn take(bytes: &[u8], offset: usize, len: usize) -> Result<&[u8], Error> {
    let end = offset.checked_add(len).ok_or_else(malformed)?;
    bytes.get(offset..end).ok_or_else(malformed)
}
fn u16_at(bytes: &[u8], offset: usize) -> Result<u16, Error> {
    Ok(u16::from_le_bytes(
        take(bytes, offset, 2)?
            .try_into()
            .map_err(|_| malformed())?,
    ))
}
fn fixed(fields: &[Field], bytes: &[u8]) -> Result<Map<String, Value>, Error> {
    let mut out = Map::new();
    for f in fields {
        let raw = take(bytes, f.offset, f.size)?;
        let mut buf = [0; 8];
        if raw.len() <= 8 {
            buf[..raw.len()].copy_from_slice(raw);
        }
        let u = u64::from_le_bytes(buf);
        let value = match f.kind {
            Kind::Address => {
                let hex = aux_rs_core::hex::encode(raw);
                let value = hex.trim_start_matches('0');
                json!(format!("0x{}", if value.is_empty() { "0" } else { value }))
            }
            Kind::OptionalI64 => {
                if u as i64 == i64::MIN {
                    Value::Null
                } else {
                    json!(u as i64)
                }
            }
            Kind::Integer { bytes, signed } => {
                if signed {
                    let shift = 64 - bytes * 8;
                    json!(((u << shift) as i64) >> shift)
                } else {
                    json!(u)
                }
            }
            // Preserve the SBE microsecond timestamp exactly, rather than losing
            // sub-millisecond precision to match the JSON channel's unit.
            Kind::Timestamp => json!(u as i64),
            Kind::Decimal { places, nullable } => {
                let n = u as i64;
                if n == i64::MIN {
                    if !nullable {
                        return Err(malformed());
                    }
                    Value::Null
                } else {
                    let n = i128::from(n);
                    let scale = 10i128.pow(places as u32);
                    let abs = n.abs();
                    json!(format!(
                        "{}{}.{:0width$}",
                        if n < 0 { "-" } else { "" },
                        abs / scale,
                        abs % scale,
                        width = places
                    ))
                }
            }
            Kind::Enum(values) => {
                if u == 254 {
                    Value::Null
                } else {
                    json!(
                        values
                            .iter()
                            .find(|(n, _)| *n == u)
                            .ok_or_else(malformed)?
                            .1
                    )
                }
            }
            Kind::Flags(values) => {
                let mask = values.iter().fold(0u64, |m, (b, _)| m | (1u64 << b));
                if u & !mask != 0 {
                    return Err(malformed());
                }
                json!(
                    values
                        .iter()
                        .filter(|(bit, _)| u & (1u64 << bit) != 0)
                        .map(|(_, name)| *name)
                        .collect::<Vec<_>>()
                )
            }
        };
        out.insert(f.name.into(), value);
    }
    Ok(out)
}
pub fn decode(bytes: &[u8]) -> Result<DecodedFrame, Error> {
    let block = u16_at(bytes, 0)?;
    let template = u16_at(bytes, 2)?;
    let schema = u16_at(bytes, 4)?;
    let version = u16_at(bytes, 6)?;
    if schema != 1 || version != 3 {
        return Err(Error::InvalidRequest(
            "unsupported Paradex SBE schema; expected 1:3",
        ));
    }
    let layout = layout::layout(template)
        .ok_or(Error::InvalidRequest("unsupported Paradex SBE template"))?;
    if usize::from(block) < layout.minimum {
        return Err(malformed());
    }
    let mut data = fixed(layout.fields, take(bytes, 8, usize::from(block))?)?;
    let mut cursor = 8 + usize::from(block);
    for (name, group) in layout.variable {
        let value = if let Some((minimum, fields)) = group {
            let block = usize::from(u16_at(bytes, cursor)?);
            let count = usize::from(u16_at(bytes, cursor + 2)?);
            cursor += 4;
            if block < *minimum {
                return Err(malformed());
            }
            take(
                bytes,
                cursor,
                block.checked_mul(count).ok_or_else(malformed)?,
            )?;
            let mut values = Vec::with_capacity(count);
            for _ in 0..count {
                values.push(Value::Object(fixed(fields, take(bytes, cursor, block)?)?));
                cursor += block;
            }
            Value::Array(values)
        } else {
            let len = usize::from(*take(bytes, cursor, 1)?.first().ok_or_else(malformed)?);
            cursor += 1;
            let s = std::str::from_utf8(take(bytes, cursor, len)?).map_err(|_| malformed())?;
            cursor += len;
            json!(s)
        };
        data.insert((*name).into(), value);
    }
    if cursor != bytes.len() {
        return Err(malformed());
    }
    // Never expose the old truncated integer as the authoritative trade id.
    if template == 1
        && data
            .get("tradeIdStr")
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
    {
        return Err(Error::InvalidRequest(
            "schema 1:3 trade lacks full trade id",
        ));
    }
    Ok(DecodedFrame {
        schema_id: schema,
        schema_version: version,
        block_length: block,
        message: serde_json::from_value(
            json!({"template":layout.template.to_string(),"data":data}),
        )?,
    })
}
