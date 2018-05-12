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
use std::error::Error;
use std::fmt;

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
    InvalidChar(char),

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
            DecodingError::InvalidChar(c) => write!(f, "{} '{}'", self.description(), c),
            _ => write!(f, "{}", self.description()),
        }
    }
}

const MASK_BITS: usize = 5;
const MASK_U64: u64 = 0b11111;
const MASK_U128: u128 = 0b11111;

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
/// append_crockford_u64(1, 1, &mut a_string);
/// assert_eq!(a_string, "1");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u64(0xFF, 4, &mut a_string);
/// assert_eq!(a_string, "007Z");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u64(0xFFFF_FFFF_FFFF_FFFF, 13, &mut a_string);
/// assert_eq!(a_string, "FZZZZZZZZZZZZ");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u64(0xFFFF_FFFF_FFFF_FFFF, 14, &mut a_string);
/// assert_eq!(a_string, "0FZZZZZZZZZZZZ");
/// ```
pub fn append_crockford_u64(value: u64, count: u8, to_append_to: &mut String) {
    let count = usize::from(count);

    for i in 0..count {
        let shift_bits = (count - i - 1) * MASK_BITS;

        let index = if shift_bits < 64 {
            ((value >> shift_bits) & MASK_U64) as usize
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
/// let parsed = parse_crockford_u64("7Z");
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
/// let parsed = parse_crockford_u64("x1iIlLoO0");
///
/// let mut string_representation = String::new();
/// append_crockford_u64(parsed?, 9, &mut string_representation);
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
/// let nope = parse_crockford_u64("12345678901234");
///
/// assert_eq!(Err(DecodingError::InvalidLength), nope);
/// ```
///
/// Parsing a 13 character works if the `u64` does not overflow.
/// Overflowing the `u64` results in `DataTypeOverflow`.
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let yeah = parse_crockford_u64("FZZZZZZZZZZZZ");
///
/// assert_eq!(Ok(0xFFFF_FFFF_FFFF_FFFF), yeah);
///
/// let nope = parse_crockford_u64("G000000000000");
///
/// assert_eq!(Err(DecodingError::DataTypeOverflow), nope);
/// ```
//
/// Parsing a string containing an invalid character results in `InvalidChar`.
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let nope = parse_crockford_u64("U");
///
/// assert_eq!(Err(DecodingError::InvalidChar('U')), nope);
/// ```
pub fn parse_crockford_u64(input: &str) -> Result<u64, DecodingError> {
    let length = input.len() as u64;
    if length > 13 {
        // more than 13 characters would exceed u64
        return Err(DecodingError::InvalidLength);
    }

    let mut result: u64 = 0;

    for (i, current_char) in input.chars().enumerate() {
        let index = current_char as usize;
        if index >= DECODING_DIGITS.len() {
            return Err(DecodingError::InvalidChar(current_char));
        }
        if let Some(u8_value) = DECODING_DIGITS[index] {
            let value = u64::from(u8_value);
            if i == 0 {
                if length == 13 && value > 15 {
                    return Err(DecodingError::DataTypeOverflow);
                }
                result = value;
            } else {
                result = (result << MASK_BITS) | value;
            };
        } else {
            return Err(DecodingError::InvalidChar(current_char));
        }
    }

    Ok(result)
}

/// Appends `count` number of [crockford Base32][crockford] digits to `to_append_to`.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
///
/// Only `count` values up to 26 make sense since that will exhaust all the bits
/// of the given `value`. Higher `count` values will simply prepend additional `0`s.
///
/// # Examples
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(1, 1, &mut a_string);
/// assert_eq!(a_string, "1");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(0xFF, 4, &mut a_string);
/// assert_eq!(a_string, "007Z");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF, 13, &mut a_string);
/// assert_eq!(a_string, "FZZZZZZZZZZZZ");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF, 14, &mut a_string);
/// assert_eq!(a_string, "0FZZZZZZZZZZZZ");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, 26, &mut a_string);
/// assert_eq!(a_string, "7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, 27, &mut a_string);
/// assert_eq!(a_string, "07ZZZZZZZZZZZZZZZZZZZZZZZZZ");
/// ```
pub fn append_crockford_u128(value: u128, count: u8, to_append_to: &mut String) {
    let count = usize::from(count);

    for i in 0..count {
        let shift_bits = (count - i - 1) * MASK_BITS;

        let index = if shift_bits < 128 {
            ((value >> shift_bits) & MASK_U128) as usize
        } else {
            0
        };

        to_append_to.push(ENCODING_DIGITS[index]);
    }
}

/// Parses the given [crockford Base32][crockford] string into a `u128`.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
///
/// # Examples
/// ```
/// # use rusty_ulid::crockford::*;
/// let parsed = parse_crockford_u128("7Z");
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
/// let parsed = parse_crockford_u128("x1iIlLoO0");
///
/// let mut string_representation = String::new();
/// append_crockford_u128(parsed?, 9, &mut string_representation);
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
/// Parsing a string longer than 26 characters would cause an `u128` overflow.
/// Trying to do so results in `InvalidLength`.
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let nope = parse_crockford_u128("123456789012345678901234567");
///
/// assert_eq!(Err(DecodingError::InvalidLength), nope);
/// ```
///
/// Parsing a 26 character works if the `u128` does not overflow.
/// Overflowing the `u128` results in `DataTypeOverflow`.
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let yeah = parse_crockford_u128("7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
///
/// assert_eq!(Ok(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF), yeah);
///
/// let nope = parse_crockford_u128("80000000000000000000000000");
///
/// assert_eq!(Err(DecodingError::DataTypeOverflow), nope);
/// ```
//
/// Parsing a string containing an invalid character results in `InvalidChar`.
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let nope = parse_crockford_u128("U");
///
/// assert_eq!(Err(DecodingError::InvalidChar('U')), nope);
/// ```
pub fn parse_crockford_u128(input: &str) -> Result<u128, DecodingError> {
    let length = input.len();
    if length > 26 {
        // more than 27 characters would exceed u128
        return Err(DecodingError::InvalidLength);
    }

    let mut result: u128 = 0;

    for (i, current_char) in input.chars().enumerate() {
        let index = current_char as usize;
        if index >= DECODING_DIGITS.len() {
            return Err(DecodingError::InvalidChar(current_char));
        }
        if let Some(u8_value) = DECODING_DIGITS[index] {
            let value = u128::from(u8_value);
            if i == 0 {
                if length == 26 && value > 7 {
                    return Err(DecodingError::DataTypeOverflow);
                }
                result = value;
            } else {
                result = (result << MASK_BITS) | value;
            };
        } else {
            return Err(DecodingError::InvalidChar(current_char));
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
    fn append_crockford_u64_test_cases() {
        single_append_crockford_u64(0, 1, "0");
        single_append_crockford_u64(1, 1, "1");
        single_append_crockford_u64(2, 1, "2");
        single_append_crockford_u64(3, 1, "3");
        single_append_crockford_u64(4, 1, "4");
        single_append_crockford_u64(5, 1, "5");
        single_append_crockford_u64(6, 1, "6");
        single_append_crockford_u64(7, 1, "7");
        single_append_crockford_u64(8, 1, "8");
        single_append_crockford_u64(9, 1, "9");
        single_append_crockford_u64(10, 1, "A");
        single_append_crockford_u64(11, 1, "B");
        single_append_crockford_u64(12, 1, "C");
        single_append_crockford_u64(13, 1, "D");
        single_append_crockford_u64(14, 1, "E");
        single_append_crockford_u64(15, 1, "F");
        single_append_crockford_u64(16, 1, "G");
        single_append_crockford_u64(17, 1, "H");
        single_append_crockford_u64(18, 1, "J");
        single_append_crockford_u64(19, 1, "K");
        single_append_crockford_u64(20, 1, "M");
        single_append_crockford_u64(21, 1, "N");
        single_append_crockford_u64(22, 1, "P");
        single_append_crockford_u64(23, 1, "Q");
        single_append_crockford_u64(24, 1, "R");
        single_append_crockford_u64(25, 1, "S");
        single_append_crockford_u64(26, 1, "T");
        single_append_crockford_u64(27, 1, "V");
        single_append_crockford_u64(28, 1, "W");
        single_append_crockford_u64(29, 1, "X");
        single_append_crockford_u64(30, 1, "Y");
        single_append_crockford_u64(31, 1, "Z");
        single_append_crockford_u64(32, 1, "0");
        single_append_crockford_u64(32, 2, "10");
        single_append_crockford_u64(0, 0, "");
        single_append_crockford_u64(0, 13, "0000000000000");
        single_append_crockford_u64(194, 2, "62");
        single_append_crockford_u64(45_678, 4, "1CKE");
        single_append_crockford_u64(393_619, 4, "C0CK");
        single_append_crockford_u64(398_373, 4, "C515");
        single_append_crockford_u64(421_562, 4, "CVNT");
        single_append_crockford_u64(456_789, 4, "DY2N");
        single_append_crockford_u64(519_571, 4, "FVCK");
        single_append_crockford_u64(3_838_385_658_376_483, 11, "3D2ZQ6TVC93");
        single_append_crockford_u64(0x1F, 1, "Z");
        single_append_crockford_u64(0x1F << 5, 1, "0");
        single_append_crockford_u64(0x1F << 5, 2, "Z0");
        single_append_crockford_u64(0x1F << 10, 1, "0");
        single_append_crockford_u64(0x1F << 10, 2, "00");
        single_append_crockford_u64(0x1F << 10, 3, "Z00");
        single_append_crockford_u64(0x1F << 15, 3, "000");
        single_append_crockford_u64(0x1F << 15, 4, "Z000");
        single_append_crockford_u64(0x1F << 55, 13, "0Z00000000000");
        single_append_crockford_u64(0x1F << 60, 13, "F000000000000");
        single_append_crockford_u64(0xFFFF_FFFF_FFFF_FFFF, 13, "FZZZZZZZZZZZZ");
        single_append_crockford_u64(PAST_TIMESTAMP, 10, PAST_TIMESTAMP_PART);
        single_append_crockford_u64(MAX_TIMESTAMP, 10, MAX_TIMESTAMP_PART);
        single_append_crockford_u64(0xFFFF_FFFF_FFFF_FFFF, 0xFF, "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000FZZZZZZZZZZZZ");
    }

    fn single_append_crockford_u64(value: u64, count: u8, expected_result: &str) {
        let mut a_string = String::new();
        append_crockford_u64(value, count, &mut a_string);
        println!("{}", a_string);
        assert_eq!(expected_result, a_string);
    }

    #[test]
    fn parse_crockford_u64_test_cases() {
        single_parse_crockford_u64("0", Ok(0));
        single_parse_crockford_u64("1", Ok(1));
        single_parse_crockford_u64("2", Ok(2));
        single_parse_crockford_u64("3", Ok(3));
        single_parse_crockford_u64("4", Ok(4));
        single_parse_crockford_u64("5", Ok(5));
        single_parse_crockford_u64("6", Ok(6));
        single_parse_crockford_u64("7", Ok(7));
        single_parse_crockford_u64("8", Ok(8));
        single_parse_crockford_u64("9", Ok(9));

        single_parse_crockford_u64("A", Ok(10));
        single_parse_crockford_u64("B", Ok(11));
        single_parse_crockford_u64("C", Ok(12));
        single_parse_crockford_u64("D", Ok(13));
        single_parse_crockford_u64("E", Ok(14));
        single_parse_crockford_u64("F", Ok(15));
        single_parse_crockford_u64("G", Ok(16));
        single_parse_crockford_u64("H", Ok(17));
        single_parse_crockford_u64("J", Ok(18));
        single_parse_crockford_u64("K", Ok(19));
        single_parse_crockford_u64("M", Ok(20));
        single_parse_crockford_u64("N", Ok(21));
        single_parse_crockford_u64("P", Ok(22));
        single_parse_crockford_u64("Q", Ok(23));
        single_parse_crockford_u64("R", Ok(24));
        single_parse_crockford_u64("S", Ok(25));
        single_parse_crockford_u64("T", Ok(26));
        single_parse_crockford_u64("V", Ok(27));
        single_parse_crockford_u64("W", Ok(28));
        single_parse_crockford_u64("X", Ok(29));
        single_parse_crockford_u64("Y", Ok(30));
        single_parse_crockford_u64("Z", Ok(31));

        single_parse_crockford_u64("a", Ok(10));
        single_parse_crockford_u64("b", Ok(11));
        single_parse_crockford_u64("c", Ok(12));
        single_parse_crockford_u64("d", Ok(13));
        single_parse_crockford_u64("e", Ok(14));
        single_parse_crockford_u64("f", Ok(15));
        single_parse_crockford_u64("g", Ok(16));
        single_parse_crockford_u64("h", Ok(17));
        single_parse_crockford_u64("j", Ok(18));
        single_parse_crockford_u64("k", Ok(19));
        single_parse_crockford_u64("m", Ok(20));
        single_parse_crockford_u64("n", Ok(21));
        single_parse_crockford_u64("p", Ok(22));
        single_parse_crockford_u64("q", Ok(23));
        single_parse_crockford_u64("r", Ok(24));
        single_parse_crockford_u64("s", Ok(25));
        single_parse_crockford_u64("t", Ok(26));
        single_parse_crockford_u64("v", Ok(27));
        single_parse_crockford_u64("w", Ok(28));
        single_parse_crockford_u64("x", Ok(29));
        single_parse_crockford_u64("y", Ok(30));
        single_parse_crockford_u64("z", Ok(31));

        single_parse_crockford_u64("10", Ok(32));

        // special characters
        single_parse_crockford_u64("o", Ok(0));
        single_parse_crockford_u64("O", Ok(0));
        single_parse_crockford_u64("i", Ok(1));
        single_parse_crockford_u64("I", Ok(1));
        single_parse_crockford_u64("l", Ok(1));
        single_parse_crockford_u64("L", Ok(1));

        single_parse_crockford_u64(PAST_TIMESTAMP_PART, Ok(PAST_TIMESTAMP));
        single_parse_crockford_u64(MAX_TIMESTAMP_PART, Ok(MAX_TIMESTAMP));

        single_parse_crockford_u64("ZZZZZZZZZZZZ", Ok(0xFFF_FFFF_FFFF_FFFF));
        single_parse_crockford_u64("FZZZZZZZZZZZZ", Ok(0xFFFF_FFFF_FFFF_FFFF));
        single_parse_crockford_u64("G000000000000", Err(DecodingError::DataTypeOverflow));

        single_parse_crockford_u64("U", Err(DecodingError::InvalidChar('U')));

        single_parse_crockford_u64("12345678901234", Err(DecodingError::InvalidLength));
    }

    fn single_parse_crockford_u64(value: &str, expected_result: Result<u64, DecodingError>) {
        let result = parse_crockford_u64(value);
        println!("parse_crockford_u64({}) => {:?}", value, result);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn append_crockford_u128_test_cases() {
        single_append_crockford_u128(0, 1, "0");
        single_append_crockford_u128(1, 1, "1");
        single_append_crockford_u128(2, 1, "2");
        single_append_crockford_u128(3, 1, "3");
        single_append_crockford_u128(4, 1, "4");
        single_append_crockford_u128(5, 1, "5");
        single_append_crockford_u128(6, 1, "6");
        single_append_crockford_u128(7, 1, "7");
        single_append_crockford_u128(8, 1, "8");
        single_append_crockford_u128(9, 1, "9");
        single_append_crockford_u128(10, 1, "A");
        single_append_crockford_u128(11, 1, "B");
        single_append_crockford_u128(12, 1, "C");
        single_append_crockford_u128(13, 1, "D");
        single_append_crockford_u128(14, 1, "E");
        single_append_crockford_u128(15, 1, "F");
        single_append_crockford_u128(16, 1, "G");
        single_append_crockford_u128(17, 1, "H");
        single_append_crockford_u128(18, 1, "J");
        single_append_crockford_u128(19, 1, "K");
        single_append_crockford_u128(20, 1, "M");
        single_append_crockford_u128(21, 1, "N");
        single_append_crockford_u128(22, 1, "P");
        single_append_crockford_u128(23, 1, "Q");
        single_append_crockford_u128(24, 1, "R");
        single_append_crockford_u128(25, 1, "S");
        single_append_crockford_u128(26, 1, "T");
        single_append_crockford_u128(27, 1, "V");
        single_append_crockford_u128(28, 1, "W");
        single_append_crockford_u128(29, 1, "X");
        single_append_crockford_u128(30, 1, "Y");
        single_append_crockford_u128(31, 1, "Z");
        single_append_crockford_u128(32, 1, "0");
        single_append_crockford_u128(32, 2, "10");
        single_append_crockford_u128(0, 0, "");
        single_append_crockford_u128(0, 13, "0000000000000");
        single_append_crockford_u128(194, 2, "62");
        single_append_crockford_u128(45_678, 4, "1CKE");
        single_append_crockford_u128(393_619, 4, "C0CK");
        single_append_crockford_u128(398_373, 4, "C515");
        single_append_crockford_u128(421_562, 4, "CVNT");
        single_append_crockford_u128(456_789, 4, "DY2N");
        single_append_crockford_u128(519_571, 4, "FVCK");
        single_append_crockford_u128(3_838_385_658_376_483, 11, "3D2ZQ6TVC93");
        single_append_crockford_u128(0x1F, 1, "Z");
        single_append_crockford_u128(0x1F << 5, 1, "0");
        single_append_crockford_u128(0x1F << 5, 2, "Z0");
        single_append_crockford_u128(0x1F << 10, 1, "0");
        single_append_crockford_u128(0x1F << 10, 2, "00");
        single_append_crockford_u128(0x1F << 10, 3, "Z00");
        single_append_crockford_u128(0x1F << 15, 3, "000");
        single_append_crockford_u128(0x1F << 15, 4, "Z000");
        single_append_crockford_u128(0x1F << 55, 13, "0Z00000000000");
        single_append_crockford_u128(0x1F << 60, 13, "Z000000000000");
        single_append_crockford_u128(0x1F << 120, 26, "0Z000000000000000000000000");
        single_append_crockford_u128(0x1F << 125, 26, "70000000000000000000000000");
        single_append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF, 13, "FZZZZZZZZZZZZ");
        single_append_crockford_u128(
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            26,
            "7ZZZZZZZZZZZZZZZZZZZZZZZZZ",
        );
        single_append_crockford_u128(PAST_TIMESTAMP.into(), 10, PAST_TIMESTAMP_PART);
        single_append_crockford_u128(MAX_TIMESTAMP.into(), 10, MAX_TIMESTAMP_PART);
        single_append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF, 0xFF, "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000FZZZZZZZZZZZZ");
        single_append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, 0xFF, "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007ZZZZZZZZZZZZZZZZZZZZZZZZZ");
    }

    fn single_append_crockford_u128(value: u128, count: u8, expected_result: &str) {
        let mut a_string = String::new();
        append_crockford_u128(value, count, &mut a_string);
        println!("{}", a_string);
        assert_eq!(expected_result, a_string);
    }

    #[test]
    fn parse_crockford_u128_test_cases() {
        single_parse_crockford_u128("0", Ok(0));
        single_parse_crockford_u128("1", Ok(1));
        single_parse_crockford_u128("2", Ok(2));
        single_parse_crockford_u128("3", Ok(3));
        single_parse_crockford_u128("4", Ok(4));
        single_parse_crockford_u128("5", Ok(5));
        single_parse_crockford_u128("6", Ok(6));
        single_parse_crockford_u128("7", Ok(7));
        single_parse_crockford_u128("8", Ok(8));
        single_parse_crockford_u128("9", Ok(9));

        single_parse_crockford_u128("A", Ok(10));
        single_parse_crockford_u128("B", Ok(11));
        single_parse_crockford_u128("C", Ok(12));
        single_parse_crockford_u128("D", Ok(13));
        single_parse_crockford_u128("E", Ok(14));
        single_parse_crockford_u128("F", Ok(15));
        single_parse_crockford_u128("G", Ok(16));
        single_parse_crockford_u128("H", Ok(17));
        single_parse_crockford_u128("J", Ok(18));
        single_parse_crockford_u128("K", Ok(19));
        single_parse_crockford_u128("M", Ok(20));
        single_parse_crockford_u128("N", Ok(21));
        single_parse_crockford_u128("P", Ok(22));
        single_parse_crockford_u128("Q", Ok(23));
        single_parse_crockford_u128("R", Ok(24));
        single_parse_crockford_u128("S", Ok(25));
        single_parse_crockford_u128("T", Ok(26));
        single_parse_crockford_u128("V", Ok(27));
        single_parse_crockford_u128("W", Ok(28));
        single_parse_crockford_u128("X", Ok(29));
        single_parse_crockford_u128("Y", Ok(30));
        single_parse_crockford_u128("Z", Ok(31));

        single_parse_crockford_u128("a", Ok(10));
        single_parse_crockford_u128("b", Ok(11));
        single_parse_crockford_u128("c", Ok(12));
        single_parse_crockford_u128("d", Ok(13));
        single_parse_crockford_u128("e", Ok(14));
        single_parse_crockford_u128("f", Ok(15));
        single_parse_crockford_u128("g", Ok(16));
        single_parse_crockford_u128("h", Ok(17));
        single_parse_crockford_u128("j", Ok(18));
        single_parse_crockford_u128("k", Ok(19));
        single_parse_crockford_u128("m", Ok(20));
        single_parse_crockford_u128("n", Ok(21));
        single_parse_crockford_u128("p", Ok(22));
        single_parse_crockford_u128("q", Ok(23));
        single_parse_crockford_u128("r", Ok(24));
        single_parse_crockford_u128("s", Ok(25));
        single_parse_crockford_u128("t", Ok(26));
        single_parse_crockford_u128("v", Ok(27));
        single_parse_crockford_u128("w", Ok(28));
        single_parse_crockford_u128("x", Ok(29));
        single_parse_crockford_u128("y", Ok(30));
        single_parse_crockford_u128("z", Ok(31));

        single_parse_crockford_u128("10", Ok(32));

        // special characters
        single_parse_crockford_u128("o", Ok(0));
        single_parse_crockford_u128("O", Ok(0));
        single_parse_crockford_u128("i", Ok(1));
        single_parse_crockford_u128("I", Ok(1));
        single_parse_crockford_u128("l", Ok(1));
        single_parse_crockford_u128("L", Ok(1));

        single_parse_crockford_u128(PAST_TIMESTAMP_PART, Ok(PAST_TIMESTAMP.into()));
        single_parse_crockford_u128(MAX_TIMESTAMP_PART, Ok(MAX_TIMESTAMP.into()));

        single_parse_crockford_u128("ZZZZZZZZZZZZ", Ok(0xFFF_FFFF_FFFF_FFFF));
        single_parse_crockford_u128("FZZZZZZZZZZZZ", Ok(0xFFFF_FFFF_FFFF_FFFF));
        single_parse_crockford_u128("G000000000000", Ok(0x1_0000_0000_0000_0000));
        single_parse_crockford_u128(
            "7ZZZZZZZZZZZZZZZZZZZZZZZZZ",
            Ok(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF),
        );
        single_parse_crockford_u128(
            "80000000000000000000000000",
            Err(DecodingError::DataTypeOverflow),
        );

        single_parse_crockford_u128("U", Err(DecodingError::InvalidChar('U')));

        single_parse_crockford_u128(
            "123456789012345678901234567",
            Err(DecodingError::InvalidLength),
        );
    }

    fn single_parse_crockford_u128(value: &str, expected_result: Result<u128, DecodingError>) {
        let result = parse_crockford_u128(value);
        println!("parse_crockford_u128({}) => {:?}", value, result);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn decoding_error_display_trait() {
        single_decoding_error_display_trait(DecodingError::InvalidLength, "invalid length");
        single_decoding_error_display_trait(
            DecodingError::InvalidChar('U'),
            "invalid character 'U'",
        );
        single_decoding_error_display_trait(DecodingError::DataTypeOverflow, "data type overflow");
    }

    fn single_decoding_error_display_trait(error: DecodingError, expected_result: &str) {
        let result = format!("{}", error);
        assert_eq!(result, expected_result)
    }

    #[test]
    fn decoding_error_causes() {
        assert!(DecodingError::InvalidLength.cause().is_none());
        assert!(DecodingError::InvalidChar('a').cause().is_none());
        assert!(DecodingError::DataTypeOverflow.cause().is_none());
    }
}
