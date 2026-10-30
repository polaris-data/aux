// Adapted under the MIT license. Upstream notice:
// The MIT License (MIT)
//
// Copyright (c) 2015 Alice Maz
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//
// Implementation: base64 0.22.1, commit e14400697453bcc85997119b874bc03d9601d0af.
// Naive::internal_encode and encode::add_padding:
// https://github.com/marshallpierce/rust-base64/blob/e14400697453bcc85997119b874bc03d9601d0af/src/engine/naive.rs
// https://github.com/marshallpierce/rust-base64/blob/e14400697453bcc85997119b874bc03d9601d0af/src/encode.rs
// Retains the three-byte/six-bit encoding and zero-filled trailing bits, using
// String output and fixed standard alphabet/padding instead of engine traits.
// No decoding, configurable alphabet, or optimized block processing is included.
//
// Protocol reference (not an endorsement of the Rust crate): the pinned official
// Lighter Go SDK serializes Sig []byte through encoding/json, which emits padded
// standard Base64. See L2CreateOrderTxInfo, L2CancelOrderTxInfo, and getTxInfo:
// https://github.com/elliottech/lighter-go/blob/ebae8edecfee9986b0a5fcde3cb9a06bd4af3a3a/types/txtypes/create_order.go
// https://github.com/elliottech/lighter-go/blob/ebae8edecfee9986b0a5fcde3cb9a06bd4af3a3a/types/txtypes/cancel_order.go
// https://pkg.go.dev/encoding/json#Marshal
// Encoding rules: https://www.rfc-editor.org/rfc/rfc4648.html#section-4
// Compatibility: RFC vectors below and tests/signing.rs's pinned Go wire fixtures.

pub(crate) fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::new();
    for chunk in input.chunks(3) {
        let bits = (u32::from(chunk[0]) << 16)
            | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8)
            | u32::from(*chunk.get(2).unwrap_or(&0));
        output.push(char::from(ALPHABET[(bits >> 18) as usize]));
        output.push(char::from(ALPHABET[((bits >> 12) & 0x3f) as usize]));
        output.push(if chunk.len() > 1 {
            char::from(ALPHABET[((bits >> 6) & 0x3f) as usize])
        } else {
            '='
        });
        output.push(if chunk.len() > 2 {
            char::from(ALPHABET[(bits & 0x3f) as usize])
        } else {
            '='
        });
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_4648_vectors_cover_each_padding_case() {
        for (input, expected) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(base64_encode(input.as_bytes()), expected);
        }
    }

    #[test]
    fn standard_alphabet_and_trailing_bits() {
        assert_eq!(base64_encode(&[0, 0, 0]), "AAAA");
        assert_eq!(base64_encode(&[0xfb, 0xef, 0xff]), "++//");
        assert_eq!(base64_encode(&[0xff]), "/w==");
        assert_eq!(base64_encode(&[0xff, 0xff]), "//8=");
        assert_eq!(base64_encode(&[0xff, 0xff, 0xff]), "////");
    }
}
