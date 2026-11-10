/// Native numeric tokens and string-encoded integers retain their original representation.
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[derive(Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NativeNumber {
    Number(
        #[cfg_attr(feature = "typescript", ts(type = "number | bigint | string"))]
        serde_json::Number,
    ),
    String(String),
}
impl Default for NativeNumber {
    fn default() -> Self {
        Self::Number(0.into())
    }
}
impl<'de> serde::Deserialize<'de> for NativeNumber {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        match <serde_json::Value as serde::Deserialize>::deserialize(d)? {
            serde_json::Value::Number(n) => Ok(Self::Number(n)),
            serde_json::Value::String(s) => Ok(Self::String(s)),
            _ => Err(serde::de::Error::custom(
                "native number or numeric string required",
            )),
        }
    }
}
