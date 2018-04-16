/*
 * The MIT License (MIT)
 * Copyright (c) 2018 Joern Huxhorn
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the “Software”), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

/*
 * Copyright 2018 Joern Huxhorn
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![deny(warnings, missing_docs)]
use std::fmt;
use std::error::Error;

#[cfg_attr(rustfmt, rustfmt_skip)]
static ENCODING_DIGITS: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
    'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X',
    'Y', 'Z',
];

#[cfg_attr(rustfmt, rustfmt_skip)]
static DECODING_DIGITS: [Option<u8>; 123] = [
    // 0
    None, None, None, None, None, None, None, None,
    // 8
    None, None, None, None, None, None, None, None,
    // 16
    None, None, None, None, None, None, None, None,
    // 24
    None, None, None, None, None, None, None, None,
    // 32
    None, None, None, None, None, None, None, None,
    // 40
    None, None, None, None, None, None, None, None,
    // 48
    Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7),
    // 56
    Some(8), Some(9), None, None, None, None, None, None,
    // 64
    None, Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16),
    // 72
    Some(17), Some(1), Some(18), Some(19), Some(1), Some(20), Some(21), Some(0),
    // 80
    Some(22), Some(23), Some(24), Some(25), Some(26), None, Some(27), Some(28),
    // 88
    Some(29), Some(30), Some(31), None, None, None, None, None,
    // 96
    None, Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16),
    // 104
    Some(17), Some(1), Some(18), Some(19), Some(1), Some(20), Some(21), Some(0),
    // 112
    Some(22), Some(23), Some(24), Some(25), Some(26), None, Some(27), Some(28),
    // 120
    Some(29), Some(30), Some(31),
];

#[derive(Debug, PartialEq)]
/// Error that can occur while decoding a [crockford Base32][crockford] string.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
pub enum DecodingError {
    /// The length of the parsed string does not conform to requirements.
    InvalidLength,

    /// The parsed string contains a character that is not allowed in a
    /// [crockford Base32][crockford] string.
    ///
    /// [crockford]: https://crockford.com/wrmg/base32.html
    InvalidChar(Option<char>),

    /// Parsing the string overflowed the result value bits.
    DataTypeOverflow,
}

impl Error for DecodingError {
    fn description(&self) -> &str {
        let result;
        match *self {
            DecodingError::InvalidLength => result = "invalid length",
            DecodingError::InvalidChar(_) => result = "invalid character",
            DecodingError::DataTypeOverflow => result = "data type overflow",
        }
        result
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            DecodingError::InvalidChar(Some(c)) => write!(f, "{} '{}'", self.description(), c),
            _ => write!(f, "{}", self.description()),
        }
    }
}

const MASK_BITS: u64 = 5;
const MASK: u64 = 0b11111;

/// Appends `count` number of [crockford Base32][crockford] digits to `to_append_to`.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
///
/// Only `count` values up to 13 make sense since that will exhaust all the bits
/// of the given `value`. Higher `count` values will simply prepend additional `0`s.
///
/// # Examples
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford(1, 1, &mut a_string);
/// assert_eq!(a_string, "1");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford(0xFF, 4, &mut a_string);
/// assert_eq!(a_string, "007Z");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford(0xFFFF_FFFF_FFFF_FFFF, 13, &mut a_string);
/// assert_eq!(a_string, "FZZZZZZZZZZZZ");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford(0xFFFF_FFFF_FFFF_FFFF, 14, &mut a_string);
/// assert_eq!(a_string, "0FZZZZZZZZZZZZ");
/// ```
pub fn append_crockford(value: u64, count: u8, to_append_to: &mut String) {
    let u64_count = u64::from(count);

    for i in 0..u64_count {
        let shift_bits = (u64_count - i - 1) * MASK_BITS;

        let index = if shift_bits < 64 {
            ((value >> shift_bits) & MASK) as usize
        } else {
            0
        };

        to_append_to.push(ENCODING_DIGITS[index]);
    }
}

/// Parses the given [crockford Base32][crockford] string into a `u64`.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
///
/// # Examples
/// ```
/// # use rusty_ulid::crockford::*;
/// let parsed = parse_crockford("7Z");
///
/// assert_eq!(Ok(0xFF), parsed);
/// ```
///
/// When decoding, upper and lower case letters are accepted,
/// `i` and `l` will be treated as `1` and `o` will be treated as `0`.
///
/// ```
/// # use std::error::Error;
/// # use rusty_ulid::crockford::*;
/// #
/// # fn try_main() -> Result<(), Box<Error>> {
/// let parsed = parse_crockford("x1iIlLoO0");
///
/// let mut string_representation = String::new();
/// append_crockford(parsed?, 9, &mut string_representation);
///
/// assert_eq!(string_representation, "X11111000");
/// #
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
///
/// # Errors
/// Parsing a string longer than 13 characters would cause an `u64` overflow.
/// Trying to do so results in `InvalidLength`.
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let nope = parse_crockford("12345678901234");
///
/// assert_eq!(Err(DecodingError::InvalidLength), nope);
/// ```
///
/// Parsing a 13 character works if the `u64` does not overflow.
/// Overflowing the `u64` results in `DataTypeOverflow`.
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let yep = parse_crockford("FZZZZZZZZZZZZ");
///
/// assert_eq!(Ok(0xFFFF_FFFF_FFFF_FFFF), yep);
///
/// let nope = parse_crockford("G000000000000");
///
/// assert_eq!(Err(DecodingError::DataTypeOverflow), nope);
/// ```
//
/// Parsing a string containing an invalid character results in `InvalidChar`.
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let nope = parse_crockford("U");
///
/// assert_eq!(Err(DecodingError::InvalidChar(Some('U'))), nope);
/// ```
pub fn parse_crockford(input: &str) -> Result<u64, DecodingError> {
    let length = input.len() as u64;
    if length > 13 {
        // more than 13 characters would exceed u64
        return Err(DecodingError::InvalidLength);
    }

    let mut result: u64 = 0;

    for (i, current_char) in input.chars().enumerate() {
        let index = current_char as usize;
        if index >= DECODING_DIGITS.len() {
            return Err(DecodingError::InvalidChar(Some(current_char)));
        }
        if let Some(value) = DECODING_DIGITS[index] {
            if length == 13 && i == 0 && (value & 0b10000) != 0 {
                return Err(DecodingError::DataTypeOverflow);
            }
            result |= (u64::from(value)) << ((length - (i as u64) - 1) * MASK_BITS);
        } else {
            return Err(DecodingError::InvalidChar(Some(current_char)));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAST_TIMESTAMP: u64 = 1481195424879;
    const PAST_TIMESTAMP_PART: &str = "01B3F2133F";

    const MAX_TIMESTAMP: u64 = 0xFFFF_FFFF_FFFF;
    const MAX_TIMESTAMP_PART: &str = "7ZZZZZZZZZ";

    #[test]
    fn append_crockford_test_cases() {
        single_append_crockford(0, 1, "0");
        single_append_crockford(1, 1, "1");
        single_append_crockford(2, 1, "2");
        single_append_crockford(3, 1, "3");
        single_append_crockford(4, 1, "4");
        single_append_crockford(5, 1, "5");
        single_append_crockford(6, 1, "6");
        single_append_crockford(7, 1, "7");
        single_append_crockford(8, 1, "8");
        single_append_crockford(9, 1, "9");
        single_append_crockford(10, 1, "A");
        single_append_crockford(11, 1, "B");
        single_append_crockford(12, 1, "C");
        single_append_crockford(13, 1, "D");
        single_append_crockford(14, 1, "E");
        single_append_crockford(15, 1, "F");
        single_append_crockford(16, 1, "G");
        single_append_crockford(17, 1, "H");
        single_append_crockford(18, 1, "J");
        single_append_crockford(19, 1, "K");
        single_append_crockford(20, 1, "M");
        single_append_crockford(21, 1, "N");
        single_append_crockford(22, 1, "P");
        single_append_crockford(23, 1, "Q");
        single_append_crockford(24, 1, "R");
        single_append_crockford(25, 1, "S");
        single_append_crockford(26, 1, "T");
        single_append_crockford(27, 1, "V");
        single_append_crockford(28, 1, "W");
        single_append_crockford(29, 1, "X");
        single_append_crockford(30, 1, "Y");
        single_append_crockford(31, 1, "Z");
        single_append_crockford(32, 1, "0");
        single_append_crockford(32, 2, "10");
        single_append_crockford(0, 0, "");
        single_append_crockford(0, 13, "0000000000000");
        single_append_crockford(194, 2, "62");
        single_append_crockford(45_678, 4, "1CKE");
        single_append_crockford(393_619, 4, "C0CK");
        single_append_crockford(398_373, 4, "C515");
        single_append_crockford(421_562, 4, "CVNT");
        single_append_crockford(456_789, 4, "DY2N");
        single_append_crockford(519_571, 4, "FVCK");
        single_append_crockford(3_838_385_658_376_483, 11, "3D2ZQ6TVC93");
        single_append_crockford(0x1F, 1, "Z");
        single_append_crockford(0x1F << 5, 1, "0");
        single_append_crockford(0x1F << 5, 2, "Z0");
        single_append_crockford(0x1F << 10, 1, "0");
        single_append_crockford(0x1F << 10, 2, "00");
        single_append_crockford(0x1F << 10, 3, "Z00");
        single_append_crockford(0x1F << 15, 3, "000");
        single_append_crockford(0x1F << 15, 4, "Z000");
        single_append_crockford(0x1F << 55, 13, "0Z00000000000");
        single_append_crockford(0x1F << 60, 13, "F000000000000");
        single_append_crockford(0xFFFF_FFFF_FFFF_FFFF, 13, "FZZZZZZZZZZZZ");
        single_append_crockford(PAST_TIMESTAMP, 10, PAST_TIMESTAMP_PART);
        single_append_crockford(MAX_TIMESTAMP, 10, MAX_TIMESTAMP_PART);
        single_append_crockford(0xFFFF_FFFF_FFFF_FFFF, 0xFF, "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000FZZZZZZZZZZZZ");
    }

    fn single_append_crockford(value: u64, count: u8, expected_result: &str) {
        let mut a_string = String::new();
        append_crockford(value, count, &mut a_string);
        println!("{}", a_string);
        assert_eq!(expected_result, a_string);
    }

    #[test]
    fn parse_crockford_test_cases() {
        single_parse_crockford("0", Ok(0));
        single_parse_crockford("1", Ok(1));
        single_parse_crockford("2", Ok(2));
        single_parse_crockford("3", Ok(3));
        single_parse_crockford("4", Ok(4));
        single_parse_crockford("5", Ok(5));
        single_parse_crockford("6", Ok(6));
        single_parse_crockford("7", Ok(7));
        single_parse_crockford("8", Ok(8));
        single_parse_crockford("9", Ok(9));

        single_parse_crockford("A", Ok(10));
        single_parse_crockford("B", Ok(11));
        single_parse_crockford("C", Ok(12));
        single_parse_crockford("D", Ok(13));
        single_parse_crockford("E", Ok(14));
        single_parse_crockford("F", Ok(15));
        single_parse_crockford("G", Ok(16));
        single_parse_crockford("H", Ok(17));
        single_parse_crockford("J", Ok(18));
        single_parse_crockford("K", Ok(19));
        single_parse_crockford("M", Ok(20));
        single_parse_crockford("N", Ok(21));
        single_parse_crockford("P", Ok(22));
        single_parse_crockford("Q", Ok(23));
        single_parse_crockford("R", Ok(24));
        single_parse_crockford("S", Ok(25));
        single_parse_crockford("T", Ok(26));
        single_parse_crockford("V", Ok(27));
        single_parse_crockford("W", Ok(28));
        single_parse_crockford("X", Ok(29));
        single_parse_crockford("Y", Ok(30));
        single_parse_crockford("Z", Ok(31));

        single_parse_crockford("a", Ok(10));
        single_parse_crockford("b", Ok(11));
        single_parse_crockford("c", Ok(12));
        single_parse_crockford("d", Ok(13));
        single_parse_crockford("e", Ok(14));
        single_parse_crockford("f", Ok(15));
        single_parse_crockford("g", Ok(16));
        single_parse_crockford("h", Ok(17));
        single_parse_crockford("j", Ok(18));
        single_parse_crockford("k", Ok(19));
        single_parse_crockford("m", Ok(20));
        single_parse_crockford("n", Ok(21));
        single_parse_crockford("p", Ok(22));
        single_parse_crockford("q", Ok(23));
        single_parse_crockford("r", Ok(24));
        single_parse_crockford("s", Ok(25));
        single_parse_crockford("t", Ok(26));
        single_parse_crockford("v", Ok(27));
        single_parse_crockford("w", Ok(28));
        single_parse_crockford("x", Ok(29));
        single_parse_crockford("y", Ok(30));
        single_parse_crockford("z", Ok(31));

        single_parse_crockford("10", Ok(32));

        // special characters
        single_parse_crockford("o", Ok(0));
        single_parse_crockford("O", Ok(0));
        single_parse_crockford("i", Ok(1));
        single_parse_crockford("I", Ok(1));
        single_parse_crockford("l", Ok(1));
        single_parse_crockford("L", Ok(1));

        single_parse_crockford(PAST_TIMESTAMP_PART, Ok(PAST_TIMESTAMP));
        single_parse_crockford(MAX_TIMESTAMP_PART, Ok(MAX_TIMESTAMP));

        single_parse_crockford("ZZZZZZZZZZZZ", Ok(0xFFF_FFFF_FFFF_FFFF));
        single_parse_crockford("FZZZZZZZZZZZZ", Ok(0xFFFF_FFFF_FFFF_FFFF));
        single_parse_crockford("G000000000000", Err(DecodingError::DataTypeOverflow));

        single_parse_crockford("U", Err(DecodingError::InvalidChar(Some('U'))));

        single_parse_crockford("12345678901234", Err(DecodingError::InvalidLength));
    }

    fn single_parse_crockford(value: &str, expected_result: Result<u64, DecodingError>) {
        let result = parse_crockford(value);
        println!("{:?}", result);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn decoding_error_display_trait() {
        single_decoding_error_display_trait(DecodingError::InvalidLength, "invalid length");
        single_decoding_error_display_trait(DecodingError::InvalidChar(None), "invalid character");
        single_decoding_error_display_trait(
            DecodingError::InvalidChar(Some('U')),
            "invalid character 'U'",
        );
        single_decoding_error_display_trait(DecodingError::DataTypeOverflow, "data type overflow");
    }

    fn single_decoding_error_display_trait(error: DecodingError, expected_result: &str) {
        let result = format!("{}", error);
        assert_eq!(result, expected_result)
    }
}
