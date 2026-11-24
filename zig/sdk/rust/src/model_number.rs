/// Preserve the wire representation of a number that may be quoted by the venue.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum NativeNumber {
    Number(serde_json::Number),
    String(String),
}

impl Default for NativeNumber {
    fn default() -> Self {
        Self::Number(0.into())
    }
}

impl<'de> serde::Deserialize<'de> for NativeNumber {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        match serde_json::Value::deserialize(d)? {
            serde_json::Value::Number(n) => Ok(Self::Number(n)),
            serde_json::Value::String(s) => Ok(Self::String(s)),
            _ => Err(serde::de::Error::custom(
                "expected a number or numeric string",
            )),
        }
    }
}
