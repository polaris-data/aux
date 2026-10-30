// Adapted under the MIT license. Upstream notice:
// Copyright (c) 2016 Pyfisch
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
// Implementation: httpdate 1.0.3, commit 638da761065df8b67282b0a1d139c0a7e4a02429.
// parse_http_date, HttpDate parsing/conversions/validation, and parser helpers:
// https://github.com/pyfisch/httpdate/blob/638da761065df8b67282b0a1d139c0a7e4a02429/src/lib.rs
// https://github.com/pyfisch/httpdate/blob/638da761065df8b67282b0a1d139c0a7e4a02429/src/date.rs
// Retains all three date formats, 1970..9999 range, weekday/calendar validation,
// and the fixed 1970..2069 mapping for two-digit years. Formatting, ordering,
// and public error APIs are omitted; the private entry point returns Option.
// Leap-year divisibility checks use Rust's equivalent is_multiple_of method.
//
// Protocol reference: Retry-After accepts an HTTP-date; HTTP recipients accept
// IMF-fixdate, obsolete RFC 850, and asctime formats. These specify behavior,
// not a particular Rust implementation. Keep the pinned parser's year mapping
// for compatibility rather than introducing a new moving-century policy.
// https://www.rfc-editor.org/rfc/rfc9110.html#section-10.2.3
// https://www.rfc-editor.org/rfc/rfc9110.html#section-5.6.7
// Compatibility: calendar/format vectors below and the transport Retry-After tests.

use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Error(());

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct HttpDate {
    /// 0...59
    sec: u8,
    /// 0...59
    min: u8,
    /// 0...23
    hour: u8,
    /// 1...31
    day: u8,
    /// 1...12
    mon: u8,
    /// 1970...9999
    year: u16,
    /// 1...7
    wday: u8,
}

impl HttpDate {
    fn is_valid(&self) -> bool {
        self.sec < 60
            && self.min < 60
            && self.hour < 24
            && self.day > 0
            && self.day < 32
            && self.mon > 0
            && self.mon <= 12
            && self.year >= 1970
            && self.year <= 9999
            && &HttpDate::from(SystemTime::from(*self)) == self
    }
}

impl From<SystemTime> for HttpDate {
    fn from(v: SystemTime) -> HttpDate {
        let dur = v
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        let secs_since_epoch = dur.as_secs();

        if secs_since_epoch >= 253402300800 {
            // year 9999
            panic!("date must be before year 9999");
        }

        /* 2000-03-01 (mod 400 year, immediately after feb29 */
        const LEAPOCH: i64 = 11017;
        const DAYS_PER_400Y: i64 = 365 * 400 + 97;
        const DAYS_PER_100Y: i64 = 365 * 100 + 24;
        const DAYS_PER_4Y: i64 = 365 * 4 + 1;

        let days = (secs_since_epoch / 86400) as i64 - LEAPOCH;
        let secs_of_day = secs_since_epoch % 86400;

        let mut qc_cycles = days / DAYS_PER_400Y;
        let mut remdays = days % DAYS_PER_400Y;

        if remdays < 0 {
            remdays += DAYS_PER_400Y;
            qc_cycles -= 1;
        }

        let mut c_cycles = remdays / DAYS_PER_100Y;
        if c_cycles == 4 {
            c_cycles -= 1;
        }
        remdays -= c_cycles * DAYS_PER_100Y;

        let mut q_cycles = remdays / DAYS_PER_4Y;
        if q_cycles == 25 {
            q_cycles -= 1;
        }
        remdays -= q_cycles * DAYS_PER_4Y;

        let mut remyears = remdays / 365;
        if remyears == 4 {
            remyears -= 1;
        }
        remdays -= remyears * 365;

        let mut year = 2000 + remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;

        let months = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
        let mut mon = 0;
        for mon_len in months.iter() {
            mon += 1;
            if remdays < *mon_len {
                break;
            }
            remdays -= *mon_len;
        }
        let mday = remdays + 1;
        let mon = if mon + 2 > 12 {
            year += 1;
            mon - 10
        } else {
            mon + 2
        };

        let mut wday = (3 + days) % 7;
        if wday <= 0 {
            wday += 7
        };

        HttpDate {
            sec: (secs_of_day % 60) as u8,
            min: ((secs_of_day % 3600) / 60) as u8,
            hour: (secs_of_day / 3600) as u8,
            day: mday as u8,
            mon: mon as u8,
            year: year as u16,
            wday: wday as u8,
        }
    }
}

impl From<HttpDate> for SystemTime {
    fn from(v: HttpDate) -> SystemTime {
        let leap_years =
            ((v.year - 1) - 1968) / 4 - ((v.year - 1) - 1900) / 100 + ((v.year - 1) - 1600) / 400;
        let mut ydays = match v.mon {
            1 => 0,
            2 => 31,
            3 => 59,
            4 => 90,
            5 => 120,
            6 => 151,
            7 => 181,
            8 => 212,
            9 => 243,
            10 => 273,
            11 => 304,
            12 => 334,
            _ => unreachable!(),
        } + v.day as u64
            - 1;
        if is_leap_year(v.year) && v.mon > 2 {
            ydays += 1;
        }
        let days = (v.year as u64 - 1970) * 365 + leap_years as u64 + ydays;
        UNIX_EPOCH
            + Duration::from_secs(
                v.sec as u64 + v.min as u64 * 60 + v.hour as u64 * 3600 + days * 86400,
            )
    }
}

impl FromStr for HttpDate {
    type Err = Error;

    fn from_str(s: &str) -> Result<HttpDate, Error> {
        if !s.is_ascii() {
            return Err(Error(()));
        }
        let x = s.trim().as_bytes();
        let date = parse_imf_fixdate(x)
            .or_else(|_| parse_rfc850_date(x))
            .or_else(|_| parse_asctime(x))?;
        if !date.is_valid() {
            return Err(Error(()));
        }
        Ok(date)
    }
}

fn toint_1(x: u8) -> Result<u8, Error> {
    let result = x.wrapping_sub(b'0');
    if result < 10 {
        Ok(result)
    } else {
        Err(Error(()))
    }
}

fn toint_2(s: &[u8]) -> Result<u8, Error> {
    let high = s[0].wrapping_sub(b'0');
    let low = s[1].wrapping_sub(b'0');

    if high < 10 && low < 10 {
        Ok(high * 10 + low)
    } else {
        Err(Error(()))
    }
}

#[allow(clippy::many_single_char_names)]
fn toint_4(s: &[u8]) -> Result<u16, Error> {
    let a = u16::from(s[0].wrapping_sub(b'0'));
    let b = u16::from(s[1].wrapping_sub(b'0'));
    let c = u16::from(s[2].wrapping_sub(b'0'));
    let d = u16::from(s[3].wrapping_sub(b'0'));

    if a < 10 && b < 10 && c < 10 && d < 10 {
        Ok(a * 1000 + b * 100 + c * 10 + d)
    } else {
        Err(Error(()))
    }
}

fn parse_imf_fixdate(s: &[u8]) -> Result<HttpDate, Error> {
    // Example: `Sun, 06 Nov 1994 08:49:37 GMT`
    if s.len() != 29 || &s[25..] != b" GMT" || s[16] != b' ' || s[19] != b':' || s[22] != b':' {
        return Err(Error(()));
    }
    Ok(HttpDate {
        sec: toint_2(&s[23..25])?,
        min: toint_2(&s[20..22])?,
        hour: toint_2(&s[17..19])?,
        day: toint_2(&s[5..7])?,
        mon: match &s[7..12] {
            b" Jan " => 1,
            b" Feb " => 2,
            b" Mar " => 3,
            b" Apr " => 4,
            b" May " => 5,
            b" Jun " => 6,
            b" Jul " => 7,
            b" Aug " => 8,
            b" Sep " => 9,
            b" Oct " => 10,
            b" Nov " => 11,
            b" Dec " => 12,
            _ => return Err(Error(())),
        },
        year: toint_4(&s[12..16])?,
        wday: match &s[..5] {
            b"Mon, " => 1,
            b"Tue, " => 2,
            b"Wed, " => 3,
            b"Thu, " => 4,
            b"Fri, " => 5,
            b"Sat, " => 6,
            b"Sun, " => 7,
            _ => return Err(Error(())),
        },
    })
}

fn parse_rfc850_date(s: &[u8]) -> Result<HttpDate, Error> {
    // Example: `Sunday, 06-Nov-94 08:49:37 GMT`
    if s.len() < 23 {
        return Err(Error(()));
    }

    fn wday<'a>(s: &'a [u8], wday: u8, name: &'static [u8]) -> Option<(u8, &'a [u8])> {
        if &s[0..name.len()] == name {
            return Some((wday, &s[name.len()..]));
        }
        None
    }
    let (wday, s) = wday(s, 1, b"Monday, ")
        .or_else(|| wday(s, 2, b"Tuesday, "))
        .or_else(|| wday(s, 3, b"Wednesday, "))
        .or_else(|| wday(s, 4, b"Thursday, "))
        .or_else(|| wday(s, 5, b"Friday, "))
        .or_else(|| wday(s, 6, b"Saturday, "))
        .or_else(|| wday(s, 7, b"Sunday, "))
        .ok_or(Error(()))?;
    if s.len() != 22 || s[12] != b':' || s[15] != b':' || &s[18..22] != b" GMT" {
        return Err(Error(()));
    }
    let mut year = u16::from(toint_2(&s[7..9])?);
    if year < 70 {
        year += 2000;
    } else {
        year += 1900;
    }
    Ok(HttpDate {
        sec: toint_2(&s[16..18])?,
        min: toint_2(&s[13..15])?,
        hour: toint_2(&s[10..12])?,
        day: toint_2(&s[0..2])?,
        mon: match &s[2..7] {
            b"-Jan-" => 1,
            b"-Feb-" => 2,
            b"-Mar-" => 3,
            b"-Apr-" => 4,
            b"-May-" => 5,
            b"-Jun-" => 6,
            b"-Jul-" => 7,
            b"-Aug-" => 8,
            b"-Sep-" => 9,
            b"-Oct-" => 10,
            b"-Nov-" => 11,
            b"-Dec-" => 12,
            _ => return Err(Error(())),
        },
        year,
        wday,
    })
}

fn parse_asctime(s: &[u8]) -> Result<HttpDate, Error> {
    // Example: `Sun Nov  6 08:49:37 1994`
    if s.len() != 24 || s[10] != b' ' || s[13] != b':' || s[16] != b':' || s[19] != b' ' {
        return Err(Error(()));
    }
    Ok(HttpDate {
        sec: toint_2(&s[17..19])?,
        min: toint_2(&s[14..16])?,
        hour: toint_2(&s[11..13])?,
        day: {
            let x = &s[8..10];
            {
                if x[0] == b' ' {
                    toint_1(x[1])
                } else {
                    toint_2(x)
                }
            }?
        },
        mon: match &s[4..8] {
            b"Jan " => 1,
            b"Feb " => 2,
            b"Mar " => 3,
            b"Apr " => 4,
            b"May " => 5,
            b"Jun " => 6,
            b"Jul " => 7,
            b"Aug " => 8,
            b"Sep " => 9,
            b"Oct " => 10,
            b"Nov " => 11,
            b"Dec " => 12,
            _ => return Err(Error(())),
        },
        year: toint_4(&s[20..24])?,
        wday: match &s[0..4] {
            b"Mon " => 1,
            b"Tue " => 2,
            b"Wed " => 3,
            b"Thu " => 4,
            b"Fri " => 5,
            b"Sat " => 6,
            b"Sun " => 7,
            _ => return Err(Error(())),
        },
    })
}

fn is_leap_year(y: u16) -> bool {
    y.is_multiple_of(4) && (!y.is_multiple_of(100) || y.is_multiple_of(400))
}

pub(super) fn parse_http_date(s: &str) -> Option<SystemTime> {
    s.parse::<HttpDate>().ok().map(SystemTime::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_three_http_date_formats_match_the_rfc_example() {
        let expected = Some(UNIX_EPOCH + Duration::from_secs(784_111_777));
        for value in [
            "Sun, 06 Nov 1994 08:49:37 GMT",
            "Sunday, 06-Nov-94 08:49:37 GMT",
            "Sun Nov  6 08:49:37 1994",
            "Sun Nov 06 08:49:37 1994",
            " \tSun, 06 Nov 1994 08:49:37 GMT\r\n",
        ] {
            assert_eq!(parse_http_date(value), expected, "{value}");
        }
    }

    // Epoch values independently calculated with Python's datetime in UTC.
    #[test]
    fn calendar_boundaries_and_leap_centuries() {
        for (value, seconds) in [
            ("Thu, 01 Jan 1970 00:00:00 GMT", 0),
            ("Tue, 29 Feb 2000 12:34:56 GMT", 951_827_696),
            ("Tue, 31 Dec 2069 23:59:59 GMT", 3_155_759_999),
            ("Wed, 01 Jan 2070 00:00:00 GMT", 3_155_760_000),
            ("Mon, 01 Mar 2100 00:00:00 GMT", 4_107_542_400),
            ("Tue, 29 Feb 2400 00:00:00 GMT", 13_574_563_200),
            ("Fri, 31 Dec 9999 23:59:59 GMT", 253_402_300_799),
        ] {
            assert_eq!(
                parse_http_date(value),
                Some(UNIX_EPOCH + Duration::from_secs(seconds)),
                "{value}"
            );
        }
    }

    #[test]
    fn obsolete_two_digit_year_mapping_is_preserved() {
        assert_eq!(
            parse_http_date("Thursday, 01-Jan-70 00:00:00 GMT"),
            Some(UNIX_EPOCH)
        );
        assert_eq!(
            parse_http_date("Tuesday, 31-Dec-69 23:59:59 GMT"),
            parse_http_date("Tue, 31 Dec 2069 23:59:59 GMT")
        );
        assert_eq!(
            parse_http_date("Tuesday, 29-Feb-00 12:34:56 GMT"),
            parse_http_date("Tue, 29 Feb 2000 12:34:56 GMT")
        );
    }

    #[test]
    fn invalid_dates_and_truncated_input_are_rejected_without_panics() {
        for value in [
            "",
            "GMT",
            "Sun,",
            "Sunday, ",
            "Sun, 07 Nov 1994 08:49:37 GMT", // Wrong weekday.
            "Wed, 31 Apr 2024 00:00:00 GMT", // Invalid day for month.
            "Mon, 29 Feb 2100 00:00:00 GMT", // Non-leap century.
            "Sun, 06 Nov 1994 24:49:37 GMT",
            "Sun, 06 Nov 1994 08:60:37 GMT",
            "Sun, 06 Nov 1994 08:49:60 GMT",
            "Sun, 00 Nov 1994 08:49:37 GMT",
            "Sun, 06 Xxx 1994 08:49:37 GMT",
            "Sun, 06 Nov 1994 08:49:37 UTC",
            "Sun, 06 Nov 1994 08:49:37 gmt",
            "Wed, 31 Dec 1969 23:59:59 GMT",
            "Fri, 31 Dec 10000 23:59:59 GMT",
            "Sun Nov 10 08:00:00 1000",
            "Sun Nov 10 08*00:00 2000",
            "Sunday, 06-Nov-94 08+49:37 GMT",
            "Sunday, 31-Feb-94 08:49:37 GMT",
            "Sun Feb 31 08:49:37 1994",
            "Sun, 06 Nov 1994 08:49:37 GMT extra",
            "éSun, 06 Nov 1994 08:49:37 GMT",
            "\u{a0}Sun, 06 Nov 1994 08:49:37 GMT",
        ] {
            assert_eq!(parse_http_date(value), None, "{value}");
        }
        for value in [
            "Sun, 06 Nov 1994 08:49:37 GMT",
            "Sunday, 06-Nov-94 08:49:37 GMT",
            "Sun Nov  6 08:49:37 1994",
        ] {
            for end in 0..value.len() {
                assert_eq!(parse_http_date(&value[..end]), None);
            }
        }
    }
}
