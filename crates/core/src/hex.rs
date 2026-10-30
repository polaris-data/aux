// Copyright (c) 2013-2014 The Rust Project Developers.
// Copyright (c) 2015-2020 The rust-hex Developers.
// Adapted under the MIT license; see ../LICENSE-hex-MIT.
//
// Implementation: hex 0.4.3, commit b2b4370b5bf021b98ee7adc92233e8de3f2de792.
// https://github.com/KokaKiwi/rust-hex/blob/b2b4370b5bf021b98ee7adc92233e8de3f2de792/src/lib.rs
// https://github.com/KokaKiwi/rust-hex/blob/b2b4370b5bf021b98ee7adc92233e8de3f2de792/src/error.rs
// Retains val, Vec::from_hex, decode_to_slice, and FromHexError behavior.
// Encoding simplifies BytesToHexChars to a flat iterator over lowercase digits;
// the generic conversion traits, uppercase encoder, and Serde support are omitted.
//
// Protocol reference (not an endorsement of the Rust crate): Hyperliquid's SDK
// uses bytes.fromhex, bytes.hex, and eth_utils.to_hex for native identifiers and
// signing data. Lighter's official signer also uses encoding/hex.
// https://github.com/hyperliquid-dex/hyperliquid-python-sdk/blob/2fdb18f9517675ea03695a0962bd19eece9c83f0/hyperliquid/utils/signing.py
// https://github.com/elliottech/lighter-go/blob/ebae8edecfee9986b0a5fcde3cb9a06bd4af3a3a/signer/key_manager.go
// Encoding rules: https://www.rfc-editor.org/rfc/rfc4648.html#section-8
// Compatibility: unit tests below and the exchanges' pinned signing fixtures.

//! Shared lowercase hex encoding and strict, case-insensitive decoding.
//! Prefix handling belongs to callers; these functions do not strip `0x`.

use std::fmt;

/// An invalid hex character, odd input length, or incorrect output buffer size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromHexError {
    InvalidHexCharacter { c: char, index: usize },
    OddLength,
    InvalidStringLength,
}

impl fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidHexCharacter { c, index } => {
                write!(f, "Invalid character {c:?} at position {index}")
            }
            Self::OddLength => write!(f, "Odd number of digits"),
            Self::InvalidStringLength => write!(f, "Invalid string length"),
        }
    }
}

impl std::error::Error for FromHexError {}

/// Encode bytes as lowercase hexadecimal without a prefix.
pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    data.as_ref()
        .iter()
        .flat_map(|byte| {
            [
                char::from(DIGITS[(byte >> 4) as usize]),
                char::from(DIGITS[(byte & 0x0f) as usize]),
            ]
        })
        .collect()
}

fn val(c: u8, index: usize) -> Result<u8, FromHexError> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(FromHexError::InvalidHexCharacter {
            c: c as char,
            index,
        }),
    }
}

/// Decode an even number of hex digits, accepting either letter case.
pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, FromHexError> {
    let data = data.as_ref();
    if data.len() % 2 != 0 {
        return Err(FromHexError::OddLength);
    }
    data.chunks_exact(2)
        .enumerate()
        .map(|(i, pair)| Ok(val(pair[0], 2 * i)? << 4 | val(pair[1], 2 * i + 1)?))
        .collect()
}

/// Decode into an exactly sized output buffer. On invalid input, bytes decoded
/// before the first invalid character may already have been written.
pub fn decode_to_slice<T: AsRef<[u8]>>(data: T, out: &mut [u8]) -> Result<(), FromHexError> {
    let data = data.as_ref();
    if data.len() % 2 != 0 {
        return Err(FromHexError::OddLength);
    }
    if data.len() / 2 != out.len() {
        return Err(FromHexError::InvalidStringLength);
    }
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = val(data[2 * i], 2 * i)? << 4 | val(data[2 * i + 1], 2 * i + 1)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_byte_values_match_standard_hex_formatting() {
        let bytes: Vec<u8> = (0..=255).collect();
        let expected: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(encode(&bytes), expected);
        assert_eq!(decode(&expected).unwrap(), bytes);
        assert_eq!(decode(expected.to_uppercase()).unwrap(), bytes);
        let mut output = [0; 256];
        decode_to_slice(&expected, &mut output).unwrap();
        assert_eq!(output.as_slice(), bytes);
    }

    #[test]
    fn empty_and_mixed_case_inputs() {
        assert_eq!(encode([]), "");
        assert_eq!(decode("").unwrap(), Vec::<u8>::new());
        assert_eq!(decode_to_slice("", &mut []), Ok(()));
        assert_eq!(decode(b"00aAfF09").unwrap(), [0, 170, 255, 9]);
        let mut output = [0; 4];
        decode_to_slice("00aAfF09", &mut output).unwrap();
        assert_eq!(output, [0, 170, 255, 9]);
    }

    #[test]
    fn invalid_input_preserves_error_details_and_precedence() {
        for (input, expected) in [
            ("0", FromHexError::OddLength),
            ("ggg", FromHexError::OddLength),
            ("€", FromHexError::OddLength),
            (
                "00g0",
                FromHexError::InvalidHexCharacter { c: 'g', index: 2 },
            ),
            ("0x", FromHexError::InvalidHexCharacter { c: 'x', index: 1 }),
            (" 0", FromHexError::InvalidHexCharacter { c: ' ', index: 0 }),
            ("é", FromHexError::InvalidHexCharacter { c: 'Ã', index: 0 }),
        ] {
            assert_eq!(decode(input), Err(expected));
            let mut out = vec![0; input.len() / 2];
            assert_eq!(decode_to_slice(input, &mut out), Err(expected));
        }
        assert_eq!(decode_to_slice("g", &mut []), Err(FromHexError::OddLength));
        for len in [0, 2] {
            let mut out = vec![0x55; len];
            assert_eq!(
                decode_to_slice("gg", &mut out),
                Err(FromHexError::InvalidStringLength)
            );
            assert_eq!(out, vec![0x55; len]);
        }
        let mut out = [0x55; 2];
        assert!(decode_to_slice("00gg", &mut out).is_err());
        assert_eq!(out, [0, 0x55]);
    }

    #[test]
    fn error_messages_match_hex_0_4_3() {
        assert_eq!(FromHexError::OddLength.to_string(), "Odd number of digits");
        assert_eq!(
            FromHexError::InvalidStringLength.to_string(),
            "Invalid string length"
        );
        assert_eq!(
            FromHexError::InvalidHexCharacter { c: '\n', index: 5 }.to_string(),
            "Invalid character '\\n' at position 5"
        );
    }
}
