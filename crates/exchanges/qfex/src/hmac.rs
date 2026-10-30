// Adapted under the MIT license. Upstream notice:
// Copyright (c) 2017 Artyom Pavlov
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Implementation: hmac 0.12.1, commit 46797e3b44973a30edb9d7f3a3ebb41810061d90.
// get_der_key and SimpleHmac::{new_from_slice, update, finalize_into}:
// https://github.com/RustCrypto/MACs/blob/46797e3b44973a30edb9d7f3a3ebb41810061d90/hmac/src/lib.rs
// https://github.com/RustCrypto/MACs/blob/46797e3b44973a30edb9d7f3a3ebb41810061d90/hmac/src/simple.rs
// Specialized to one-shot SHA-256 signing with a 64-byte block and 32-byte
// output. Retains long-key hashing, zero padding, and inner/outer hash order;
// reuses one Zeroizing pad buffer. No generic MAC, verification, or reset API.
// SHA-256 itself remains supplied by sha2.
//
// Protocol reference: QFEX documents HMAC-SHA256(secret, "nonce:unix_ts") with
// hex output. This specifies the algorithm, not a particular Rust crate.
// https://docs.qfex.com/websocket/channels/trade/authenticate (reviewed 2026-10-30)
// Construction: https://www.rfc-editor.org/rfc/rfc2104.html#section-2
// Compatibility: RFC 4231 vectors below and tests/fixtures/auth.json, generated
// independently with Python's standard-library hmac module.

use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

pub(crate) fn sha256(key: &[u8], message: &[u8]) -> [u8; 32] {
    let mut pad = Zeroizing::new([0u8; 64]);
    if key.len() > pad.len() {
        let hashed = Zeroizing::new(<[u8; 32]>::from(Sha256::digest(key)));
        pad[..32].copy_from_slice(hashed.as_ref());
    } else {
        pad[..key.len()].copy_from_slice(key);
    }
    for byte in pad.iter_mut() {
        *byte ^= 0x36;
    }
    let mut inner = Sha256::new();
    inner.update(pad.as_ref());
    inner.update(message);
    let inner_hash = Zeroizing::new(<[u8; 32]>::from(inner.finalize()));
    for byte in pad.iter_mut() {
        *byte ^= 0x36 ^ 0x5c;
    }
    let mut outer = Sha256::new();
    outer.update(pad.as_ref());
    outer.update(inner_hash.as_ref());
    outer.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aux_rs_core::hex;

    // https://www.rfc-editor.org/rfc/rfc4231.html#section-4
    #[test]
    fn rfc_4231_sha256_vectors() {
        let cases = [
            (vec![0x0b; 20], b"Hi There".to_vec(),
             "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"),
            (b"Jefe".to_vec(), b"what do ya want for nothing?".to_vec(),
             "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"),
            (vec![0xaa; 20], vec![0xdd; 50],
             "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe"),
            ((1..=25).collect(), vec![0xcd; 50],
             "82558a389a443c0ea4cc819899f2083a85f0faa3e578f8077a2e3ff46729665b"),
            (vec![0xaa; 131], b"Test Using Larger Than Block-Size Key - Hash Key First".to_vec(),
             "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"),
            (vec![0xaa; 131], b"This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm.".to_vec(),
             "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2"),
        ];
        for (key, message, expected) in cases {
            assert_eq!(hex::encode(sha256(&key, &message)), expected);
        }
        // RFC case 5 publishes only the first 128 bits.
        let truncated = sha256(&[0x0c; 20], b"Test With Truncation");
        assert_eq!(
            hex::encode(&truncated[..16]),
            "a3b6167473100ee06e0c796c2955552b"
        );
    }

    // Independent Python standard-library HMAC-SHA256 vectors, key bytes 0..len.
    #[test]
    fn key_lengths_at_sha256_block_boundary() {
        for (len, expected) in [
            (
                0,
                "2610988f9611cad5125199d517643a6b7b52a5a2ace57eb3659dba9104079a64",
            ),
            (
                1,
                "2610988f9611cad5125199d517643a6b7b52a5a2ace57eb3659dba9104079a64",
            ),
            (
                63,
                "e455b37150d3082319e3cb0a43c73a57727c2bd50f64157cb5704aa76d3032b9",
            ),
            (
                64,
                "e46c64041bf1c4933639d0b9f0ce971774af1915f5554037902cb516aad726b0",
            ),
            (
                65,
                "a1cfeea3c6c966f3308e14e263c5a5a0c10ded992d5077b9232ad84e04b66e3c",
            ),
            (
                128,
                "fc3632e3b6087ca3582f802f2a86ddb3106beea7535450e6a3497c267ed00e1f",
            ),
            (
                131,
                "5b09e642136d143583b161ee59062c3f4f3196988616929a78acb4db342f07db",
            ),
        ] {
            let key: Vec<u8> = (0..len).collect();
            assert_eq!(hex::encode(sha256(&key, b"nonce:1760545414")), expected);
        }
        assert_eq!(
            hex::encode(sha256(b"", b"")),
            "b613679a0814d9ec772f95d778c35fc5ff1697c493715653c6c712144292c5ad"
        );
    }
}
