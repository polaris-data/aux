//! Caller-owned buffers keep protocol encoding independent of the host allocator.
unsafe extern "C" {
    fn aux_v1_hex_encode(input: *const u8, len: usize, out: *mut u8, capacity: usize) -> bool;
    fn aux_v1_hex_decode(
        input: *const u8,
        len: usize,
        out: *mut u8,
        capacity: usize,
        bad: *mut usize,
    ) -> u8;
    fn aux_v1_url_component(input: *const u8, len: usize, out: *mut u8, capacity: usize) -> usize;
}

pub fn hex_encode(input: &[u8]) -> String {
    let mut output = vec![0; input.len().checked_mul(2).expect("hex string too long")];
    // SAFETY: Both slices are live, disjoint, and the output fits exactly.
    let ok = unsafe {
        aux_v1_hex_encode(
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    };
    assert!(ok, "invalid encoded hex length");
    String::from_utf8(output).expect("hex digits are ASCII")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexError {
    OddLength,
    InvalidStringLength,
    InvalidHexCharacter { c: char, index: usize },
}

pub fn hex_decode(input: &[u8], output: &mut [u8]) -> Result<(), HexError> {
    let mut index = 0;
    // SAFETY: Zig checks output capacity before writing. Both slices and the
    // error index are valid for this synchronous call.
    match unsafe {
        aux_v1_hex_decode(
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
            output.len(),
            &mut index,
        )
    } {
        0 => Ok(()),
        1 => Err(HexError::OddLength),
        2 => Err(HexError::InvalidStringLength),
        3 => Err(HexError::InvalidHexCharacter {
            c: char::from(input[index]),
            index,
        }),
        _ => unreachable!("invalid hex decoder status"),
    }
}

pub fn url_component(value: &str) -> String {
    let mut output = vec![0; value.len().checked_mul(3).expect("URL component too long")];
    // SAFETY: Each byte produces at most three ASCII bytes in the output.
    let size = unsafe {
        aux_v1_url_component(
            value.as_ptr(),
            value.len(),
            output.as_mut_ptr(),
            output.len(),
        )
    };
    output.truncate(size);
    String::from_utf8(output).expect("URL components are ASCII")
}
