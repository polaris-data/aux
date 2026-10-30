use crate::Error;
use aux_rs_core::hex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

macro_rules! identifier {
    ($name:ident, $length:expr, $message:literal) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name([u8; $length]);

        impl $name {
            pub const fn from_bytes(bytes: [u8; $length]) -> Self {
                Self(bytes)
            }
            pub const fn as_bytes(&self) -> &[u8; $length] {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = Error;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let value = value
                    .strip_prefix("0x")
                    .or_else(|| value.strip_prefix("0X"))
                    .unwrap_or(value);
                let mut bytes = [0; $length];
                hex::decode_to_slice(value, &mut bytes)
                    .map_err(|_| Error::InvalidRequest($message))?;
                Ok(Self(bytes))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "0x{}", hex::encode(self.0))
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self, f)
            }
        }
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                String::deserialize(deserializer)?
                    .parse()
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

identifier!(
    Address,
    20,
    "address must contain exactly 20 hexadecimal bytes"
);
identifier!(
    Cloid,
    16,
    "client order ID must contain exactly 16 hexadecimal bytes"
);
