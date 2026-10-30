/// Encode one URL path segment or query value without adding delimiters.
pub fn url_component(value: &str) -> String {
    let mut result = String::new();
    for b in value.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
            result.push(char::from(b));
        } else {
            use std::fmt::Write;
            let _ = write!(result, "%{b:02X}");
        }
    }
    result
}

/// Convert a positive decimal value into an exact number of native increments.
/// Exponents, signs, rounding, and values beyond signed 64-bit units are rejected.
pub fn decimal_units(value: &str, increment: &str) -> Result<u64, &'static str> {
    fn parse(s: &str) -> Result<(u128, u32), &'static str> {
        if s.is_empty() {
            return Err("empty decimal");
        }
        let mut dot = false;
        let mut scale = 0;
        let mut n = 0u128;
        let mut digits = 0;
        for b in s.bytes() {
            if b == b'.' && !dot {
                dot = true;
                continue;
            }
            if !b.is_ascii_digit() {
                return Err("expected a plain positive decimal");
            }
            n = n
                .checked_mul(10)
                .and_then(|n| n.checked_add(u128::from(b - b'0')))
                .ok_or("decimal overflow")?;
            if dot {
                scale += 1;
            }
            digits += 1;
        }
        if digits == 0 || s.starts_with('.') || s.ends_with('.') {
            return Err("invalid decimal");
        }
        Ok((n, scale))
    }
    let (mut n, ns) = parse(value)?;
    let (mut step, ss) = parse(increment)?;
    if n == 0 || step == 0 {
        return Err("value and increment must be positive");
    }
    if ns < ss {
        n = n
            .checked_mul(10u128.checked_pow(ss - ns).ok_or("decimal overflow")?)
            .ok_or("decimal overflow")?;
    }
    if ss < ns {
        step = step
            .checked_mul(10u128.checked_pow(ns - ss).ok_or("decimal overflow")?)
            .ok_or("decimal overflow")?;
    }
    if n % step != 0 {
        return Err("value is not an exact multiple of the native increment");
    }
    let units = u64::try_from(n / step).map_err(|_| "native units overflow")?;
    if units > i64::MAX as u64 {
        return Err("native units overflow");
    }
    Ok(units)
}

/// Validate a positive, plain decimal without converting through floating point.
pub fn positive_decimal(value: &str) -> Result<(), &'static str> {
    let mut dot = false;
    let mut nonzero = false;
    if value.is_empty() || value.starts_with('.') || value.ends_with('.') {
        return Err("invalid decimal");
    }
    for b in value.bytes() {
        if b == b'.' && !dot {
            dot = true;
            continue;
        }
        if !b.is_ascii_digit() {
            return Err("expected a plain positive decimal");
        }
        nonzero |= b != b'0';
    }
    if nonzero {
        Ok(())
    } else {
        Err("decimal must be positive")
    }
}
