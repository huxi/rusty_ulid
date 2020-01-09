/*
 * The MIT License (MIT)
 * Copyright (c) 2018-2019 Joern Huxhorn
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
 * Copyright 2018-2019 Joern Huxhorn
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

#[rustfmt::skip]
static ENCODING_DIGITS: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
    'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X',
    'Y', 'Z',
];

fn resolve_u64_value_for_char(c: char) -> Result<u64, DecodingError> {
    let index = c as usize;
    if index < DECODING_DIGITS.len() {
        if let Some(u8_value) = DECODING_DIGITS[index] {
            return Ok(u64::from(u8_value));
        }
    }
    Err(DecodingError::InvalidChar(c))
}

fn resolve_u128_value_for_char(c: char) -> Result<u128, DecodingError> {
    let index = c as usize;
    if index < DECODING_DIGITS.len() {
        if let Some(u8_value) = DECODING_DIGITS[index] {
            return Ok(u128::from(u8_value));
        }
    }
    Err(DecodingError::InvalidChar(c))
}

#[rustfmt::skip]
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
    /// The length of the parsed string or given slice of bytes does not conform to requirements.
    InvalidLength,

    /// The parsed string contains a character that is not allowed in a
    /// [crockford Base32][crockford] string.
    ///
    /// [crockford]: https://crockford.com/wrmg/base32.html
    InvalidChar(char),

    /// Parsing the string overflowed the result value bits.
    DataTypeOverflow,
}

impl Error for DecodingError {}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            DecodingError::InvalidLength => write!(f, "invalid length"),
            DecodingError::InvalidChar(c) => write!(f, "invalid character '{}'", c),
            DecodingError::DataTypeOverflow => write!(f, "data type overflow"),
        }
    }
}

const MASK_U64: u64 = 0b11111;
const MASK_U128: u128 = 0b11111;

/// Appends the [crockford Base32][crockford] representation of the `u128` to `to_append_to`.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
///
/// # Examples
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(1, &mut a_string);
/// assert_eq!(a_string, "00000000000000000000000001");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(0xFF, &mut a_string);
/// assert_eq!(a_string, "0000000000000000000000007Z");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF, &mut a_string);
/// assert_eq!(a_string, "0000000000000FZZZZZZZZZZZZ");
/// ```
///
/// ```
/// # use rusty_ulid::crockford::*;
/// let mut a_string = String::new();
/// append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, &mut a_string);
/// assert_eq!(a_string, "7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
/// ```
pub fn append_crockford_u128(value: u128, to_append_to: &mut String) {
    to_append_to.push(ENCODING_DIGITS[(value >> 125) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 120) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 115) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 110) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 105) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 100) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 95) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 90) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 85) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 80) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 75) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 70) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 65) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 60) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 55) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 50) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 45) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 40) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 35) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 30) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 25) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 20) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 15) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 10) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value >> 5) & MASK_U128) as usize]);
    to_append_to.push(ENCODING_DIGITS[(value & MASK_U128) as usize]);
}

/// Parses the given [crockford Base32][crockford] string into a `u128`.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
///
/// # Examples
/// ```
/// # use rusty_ulid::crockford::*;
/// let parsed = parse_crockford_u128("0000000000000000000000007Z");
///
/// assert_eq!(Ok(0xFF), parsed);
/// ```
///
/// When decoding, upper and lower case letters are accepted,
/// `i` and `l` will be treated as `1` and `o` will be treated as `0`.
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let parsed = parse_crockford_u128("00000000000000000x1iIlLoO0")?;
///
/// let mut string_representation = String::new();
/// append_crockford_u128(parsed, &mut string_representation);
///
/// assert_eq!(string_representation, "00000000000000000X11111000");
/// # Ok::<(), rusty_ulid::DecodingError>(())
/// ```
///
/// # Errors
/// Parsing a string with other than 26 bytes results in `InvalidLength`.
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let nope = parse_crockford_u128("1234567890123456789012345");
///
/// assert_eq!(Err(DecodingError::InvalidLength), nope);
/// ```
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let nope = parse_crockford_u128("123456789012345678901234567");
///
/// assert_eq!(Err(DecodingError::InvalidLength), nope);
/// ```
///
/// Parsing 26 bytes results in `DataTypeOverflow` if the `u128` would overflow.
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let yeah = parse_crockford_u128("7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
///
/// assert_eq!(Ok(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF), yeah);
///
/// let nope = parse_crockford_u128("80000000000000000000000000");
///
/// assert_eq!(Err(DecodingError::DataTypeOverflow), nope);
/// ```
///
/// Parsing a string containing an invalid character results in `InvalidChar` containing
/// the character.
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let nope = parse_crockford_u128("0000000000000000000000000U");
///
/// assert_eq!(Err(DecodingError::InvalidChar('U')), nope);
/// ```
pub fn parse_crockford_u128(input: &str) -> Result<u128, DecodingError> {
    let length = input.len();
    if length != 26 {
        return Err(DecodingError::InvalidLength);
    }

    let mut chars = input.chars();

    let highest = resolve_u128_value_for_char(chars.next().unwrap())?;
    if highest > 7 {
        return Err(DecodingError::DataTypeOverflow);
    }

    let mut result: u128 = highest << 125;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 120;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 115;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 110;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 105;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 100;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 95;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 90;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 85;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 80;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 75;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 70;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 65;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 60;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 55;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 50;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 45;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 40;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 35;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 30;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 25;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 20;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 15;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 10;
    result |= resolve_u128_value_for_char(chars.next().unwrap())? << 5;
    result |= resolve_u128_value_for_char(chars.next().unwrap())?;

    Ok(result)
}

/// Appends the [crockford Base32][crockford] representation of the `(u64, u64)` to `to_append_to`.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
///
/// # Examples
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let mut a_string = String::new();
/// append_crockford_u64_tuple((0, 1), &mut a_string);
///
/// assert_eq!(a_string, "00000000000000000000000001");
/// ```
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let mut a_string = String::new();
/// append_crockford_u64_tuple((0, 0xFF), &mut a_string);
///
/// assert_eq!(a_string, "0000000000000000000000007Z");
/// ```
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let mut a_string = String::new();
/// append_crockford_u64_tuple((0, 0xFFFF_FFFF_FFFF_FFFF), &mut a_string);
///
/// assert_eq!(a_string, "0000000000000FZZZZZZZZZZZZ");
/// ```
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let mut a_string = String::new();
/// append_crockford_u64_tuple((0xFFFF_FFFF_FFFF_FFFF, 0xFFFF_FFFF_FFFF_FFFF), &mut a_string);
///
/// assert_eq!(a_string, "7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
/// ```
pub fn append_crockford_u64_tuple(value: (u64, u64), to_append_to: &mut String) {
    to_append_to.push(ENCODING_DIGITS[(value.0 >> 61) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 56) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 51) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 46) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 41) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 36) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 31) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 26) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 21) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 16) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 11) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 6) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.0 >> 1) & MASK_U64) as usize]);

    let split = ((value.0 << 4) & MASK_U64) | ((value.1 >> 60) & MASK_U64);
    to_append_to.push(ENCODING_DIGITS[split as usize]);

    to_append_to.push(ENCODING_DIGITS[((value.1 >> 55) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 50) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 45) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 40) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 35) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 30) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 25) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 20) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 15) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 10) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[((value.1 >> 5) & MASK_U64) as usize]);
    to_append_to.push(ENCODING_DIGITS[(value.1 & MASK_U64) as usize]);
}

/// Parses the given [crockford Base32][crockford] string into a `(u64, u64)`.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
///
/// # Examples
/// ```
/// use rusty_ulid::crockford::*;
///
/// let parsed = parse_crockford_u64_tuple("0000000000000000000000007Z");
///
/// assert_eq!(Ok((0, 0xFF)), parsed);
/// ```
///
/// When decoding, upper and lower case letters are accepted,
/// `i` and `l` will be treated as `1` and `o` will be treated as `0`.
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let parsed = parse_crockford_u64_tuple("00000000000000000x1iIlLoO0")?;
///
/// let mut string_representation = String::new();
/// append_crockford_u64_tuple(parsed, &mut string_representation);
///
/// assert_eq!(string_representation, "00000000000000000X11111000");
/// # Ok::<(), rusty_ulid::DecodingError>(())
/// ```
///
/// # Errors
/// Parsing a string with other than 26 bytes results in `InvalidLength`.
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let nope = parse_crockford_u64_tuple("1234567890123456789012345");
///
/// assert_eq!(Err(DecodingError::InvalidLength), nope);
/// ```
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let nope = parse_crockford_u64_tuple("123456789012345678901234567");
///
/// assert_eq!(Err(DecodingError::InvalidLength), nope);
/// ```
///
/// Parsing 26 bytes results in `DataTypeOverflow` if the `(u64, u64)` would overflow.
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let yeah = parse_crockford_u64_tuple("7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
///
/// assert_eq!(Ok((0xFFFF_FFFF_FFFF_FFFF, 0xFFFF_FFFF_FFFF_FFFF)), yeah);
///
/// let nope = parse_crockford_u64_tuple("80000000000000000000000000");
///
/// assert_eq!(Err(DecodingError::DataTypeOverflow), nope);
/// ```
///
/// Parsing a string containing an invalid character results in `InvalidChar` containing
/// the character.
///
/// ```
/// use rusty_ulid::crockford::*;
///
/// let nope = parse_crockford_u64_tuple("0000000000000000000000000U");
///
/// assert_eq!(Err(DecodingError::InvalidChar('U')), nope);
/// ```
pub fn parse_crockford_u64_tuple(input: &str) -> Result<(u64, u64), DecodingError> {
    let length = input.len();
    if length != 26 {
        return Err(DecodingError::InvalidLength);
    }

    let mut chars = input.chars();
    let highest = resolve_u64_value_for_char(chars.next().unwrap())?;
    if highest > 7 {
        return Err(DecodingError::DataTypeOverflow);
    }

    let mut high: u64 = highest << 61;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 56;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 51;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 46;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 41;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 36;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 31;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 26;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 21;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 16;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 11;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 6;
    high |= resolve_u64_value_for_char(chars.next().unwrap())? << 1;

    let split = resolve_u64_value_for_char(chars.next().unwrap())?;
    high |= split >> 4;

    let mut low: u64 = split << 60;

    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 55;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 50;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 45;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 40;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 35;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 30;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 25;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 20;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 15;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 10;
    low |= resolve_u64_value_for_char(chars.next().unwrap())? << 5;
    low |= resolve_u64_value_for_char(chars.next().unwrap())?;

    Ok((high, low))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_crockford_u128_test_cases() {
        single_append_crockford_u128(0, "00000000000000000000000000");
        single_append_crockford_u128(1, "00000000000000000000000001");
        single_append_crockford_u128(2, "00000000000000000000000002");
        single_append_crockford_u128(3, "00000000000000000000000003");
        single_append_crockford_u128(4, "00000000000000000000000004");
        single_append_crockford_u128(5, "00000000000000000000000005");
        single_append_crockford_u128(6, "00000000000000000000000006");
        single_append_crockford_u128(7, "00000000000000000000000007");
        single_append_crockford_u128(8, "00000000000000000000000008");
        single_append_crockford_u128(9, "00000000000000000000000009");
        single_append_crockford_u128(10, "0000000000000000000000000A");
        single_append_crockford_u128(11, "0000000000000000000000000B");
        single_append_crockford_u128(12, "0000000000000000000000000C");
        single_append_crockford_u128(13, "0000000000000000000000000D");
        single_append_crockford_u128(14, "0000000000000000000000000E");
        single_append_crockford_u128(15, "0000000000000000000000000F");
        single_append_crockford_u128(16, "0000000000000000000000000G");
        single_append_crockford_u128(17, "0000000000000000000000000H");
        single_append_crockford_u128(18, "0000000000000000000000000J");
        single_append_crockford_u128(19, "0000000000000000000000000K");
        single_append_crockford_u128(20, "0000000000000000000000000M");
        single_append_crockford_u128(21, "0000000000000000000000000N");
        single_append_crockford_u128(22, "0000000000000000000000000P");
        single_append_crockford_u128(23, "0000000000000000000000000Q");
        single_append_crockford_u128(24, "0000000000000000000000000R");
        single_append_crockford_u128(25, "0000000000000000000000000S");
        single_append_crockford_u128(26, "0000000000000000000000000T");
        single_append_crockford_u128(27, "0000000000000000000000000V");
        single_append_crockford_u128(28, "0000000000000000000000000W");
        single_append_crockford_u128(29, "0000000000000000000000000X");
        single_append_crockford_u128(30, "0000000000000000000000000Y");
        single_append_crockford_u128(31, "0000000000000000000000000Z");
        single_append_crockford_u128(32, "00000000000000000000000010");
        single_append_crockford_u128(194, "00000000000000000000000062");
        single_append_crockford_u128(45_678, "00000000000000000000001CKE");
        single_append_crockford_u128(393_619, "0000000000000000000000C0CK");
        single_append_crockford_u128(398_373, "0000000000000000000000C515");
        single_append_crockford_u128(421_562, "0000000000000000000000CVNT");
        single_append_crockford_u128(456_789, "0000000000000000000000DY2N");
        single_append_crockford_u128(519_571, "0000000000000000000000FVCK");
        single_append_crockford_u128(3_838_385_658_376_483, "0000000000000003D2ZQ6TVC93");
        single_append_crockford_u128(0x1F, "0000000000000000000000000Z");
        single_append_crockford_u128(0x1F << 5, "000000000000000000000000Z0");
        single_append_crockford_u128(0x1F << 10, "00000000000000000000000Z00");
        single_append_crockford_u128(0x1F << 15, "0000000000000000000000Z000");
        single_append_crockford_u128(0x1F << 55, "00000000000000Z00000000000");
        single_append_crockford_u128(0x1F << 60, "0000000000000Z000000000000");
        single_append_crockford_u128(0x1F << 120, "0Z000000000000000000000000");
        single_append_crockford_u128(0x1F << 125, "70000000000000000000000000");
        single_append_crockford_u128(0xFFFF_FFFF_FFFF_FFFF, "0000000000000FZZZZZZZZZZZZ");
        single_append_crockford_u128(
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            "7ZZZZZZZZZZZZZZZZZZZZZZZZZ",
        );
    }

    #[test]
    fn parse_crockford_u128_test_cases() {
        single_parse_crockford_u128("00000000000000000000000000", Ok(0));
        single_parse_crockford_u128("00000000000000000000000001", Ok(1));
        single_parse_crockford_u128("00000000000000000000000002", Ok(2));
        single_parse_crockford_u128("00000000000000000000000003", Ok(3));
        single_parse_crockford_u128("00000000000000000000000004", Ok(4));
        single_parse_crockford_u128("00000000000000000000000005", Ok(5));
        single_parse_crockford_u128("00000000000000000000000006", Ok(6));
        single_parse_crockford_u128("00000000000000000000000007", Ok(7));
        single_parse_crockford_u128("00000000000000000000000008", Ok(8));
        single_parse_crockford_u128("00000000000000000000000009", Ok(9));

        single_parse_crockford_u128("0000000000000000000000000A", Ok(10));
        single_parse_crockford_u128("0000000000000000000000000B", Ok(11));
        single_parse_crockford_u128("0000000000000000000000000C", Ok(12));
        single_parse_crockford_u128("0000000000000000000000000D", Ok(13));
        single_parse_crockford_u128("0000000000000000000000000E", Ok(14));
        single_parse_crockford_u128("0000000000000000000000000F", Ok(15));
        single_parse_crockford_u128("0000000000000000000000000G", Ok(16));
        single_parse_crockford_u128("0000000000000000000000000H", Ok(17));
        single_parse_crockford_u128("0000000000000000000000000J", Ok(18));
        single_parse_crockford_u128("0000000000000000000000000K", Ok(19));
        single_parse_crockford_u128("0000000000000000000000000M", Ok(20));
        single_parse_crockford_u128("0000000000000000000000000N", Ok(21));
        single_parse_crockford_u128("0000000000000000000000000P", Ok(22));
        single_parse_crockford_u128("0000000000000000000000000Q", Ok(23));
        single_parse_crockford_u128("0000000000000000000000000R", Ok(24));
        single_parse_crockford_u128("0000000000000000000000000S", Ok(25));
        single_parse_crockford_u128("0000000000000000000000000T", Ok(26));
        single_parse_crockford_u128("0000000000000000000000000V", Ok(27));
        single_parse_crockford_u128("0000000000000000000000000W", Ok(28));
        single_parse_crockford_u128("0000000000000000000000000X", Ok(29));
        single_parse_crockford_u128("0000000000000000000000000Y", Ok(30));
        single_parse_crockford_u128("0000000000000000000000000Z", Ok(31));

        single_parse_crockford_u128("0000000000000000000000000a", Ok(10));
        single_parse_crockford_u128("0000000000000000000000000b", Ok(11));
        single_parse_crockford_u128("0000000000000000000000000c", Ok(12));
        single_parse_crockford_u128("0000000000000000000000000d", Ok(13));
        single_parse_crockford_u128("0000000000000000000000000e", Ok(14));
        single_parse_crockford_u128("0000000000000000000000000f", Ok(15));
        single_parse_crockford_u128("0000000000000000000000000g", Ok(16));
        single_parse_crockford_u128("0000000000000000000000000h", Ok(17));
        single_parse_crockford_u128("0000000000000000000000000j", Ok(18));
        single_parse_crockford_u128("0000000000000000000000000k", Ok(19));
        single_parse_crockford_u128("0000000000000000000000000m", Ok(20));
        single_parse_crockford_u128("0000000000000000000000000n", Ok(21));
        single_parse_crockford_u128("0000000000000000000000000p", Ok(22));
        single_parse_crockford_u128("0000000000000000000000000q", Ok(23));
        single_parse_crockford_u128("0000000000000000000000000r", Ok(24));
        single_parse_crockford_u128("0000000000000000000000000s", Ok(25));
        single_parse_crockford_u128("0000000000000000000000000t", Ok(26));
        single_parse_crockford_u128("0000000000000000000000000v", Ok(27));
        single_parse_crockford_u128("0000000000000000000000000w", Ok(28));
        single_parse_crockford_u128("0000000000000000000000000x", Ok(29));
        single_parse_crockford_u128("0000000000000000000000000y", Ok(30));
        single_parse_crockford_u128("0000000000000000000000000z", Ok(31));

        single_parse_crockford_u128("00000000000000000000000010", Ok(32));

        // special characters
        single_parse_crockford_u128("0000000000000000000000000o", Ok(0));
        single_parse_crockford_u128("0000000000000000000000000O", Ok(0));
        single_parse_crockford_u128("0000000000000000000000000i", Ok(1));
        single_parse_crockford_u128("0000000000000000000000000I", Ok(1));
        single_parse_crockford_u128("0000000000000000000000000l", Ok(1));
        single_parse_crockford_u128("0000000000000000000000000L", Ok(1));

        single_parse_crockford_u128("00000000000000ZZZZZZZZZZZZ", Ok(0xFFF_FFFF_FFFF_FFFF));
        single_parse_crockford_u128("0000000000000FZZZZZZZZZZZZ", Ok(0xFFFF_FFFF_FFFF_FFFF));
        single_parse_crockford_u128("0000000000000G000000000000", Ok(0x1_0000_0000_0000_0000));
        single_parse_crockford_u128(
            "7ZZZZZZZZZZZZZZZZZZZZZZZZZ",
            Ok(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF),
        );
        single_parse_crockford_u128(
            "80000000000000000000000000",
            Err(DecodingError::DataTypeOverflow),
        );

        single_parse_crockford_u128(
            "0000000000000000000000000U",
            Err(DecodingError::InvalidChar('U')),
        );

        single_parse_crockford_u128(
            "123456789012345678901234567",
            Err(DecodingError::InvalidLength),
        );
    }

    #[test]
    fn append_crockford_u64_tuple_test_cases() {
        single_append_crockford_u64_tuple(
            (0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F),
            "0H48SM8NB6EY49KANVSKEYXW0F",
        );
        single_append_crockford_u64_tuple(
            (0xFFFF_FFFF_FFFF_FFFF, 0xFFFF_FFFF_FFFF_FFFF),
            "7ZZZZZZZZZZZZZZZZZZZZZZZZZ",
        );

        single_append_crockford_u64_tuple((0, 0), "00000000000000000000000000");
    }

    #[test]
    fn parse_crockford_u64_tuple_test_cases() {
        single_parse_crockford_u64_tuple("00000000000000000000000000", Ok((0, 0)));
        single_parse_crockford_u64_tuple("00000000000000000000000001", Ok((0, 1)));
        single_parse_crockford_u64_tuple("00000000000000000000000002", Ok((0, 2)));
        single_parse_crockford_u64_tuple("00000000000000000000000003", Ok((0, 3)));
        single_parse_crockford_u64_tuple("00000000000000000000000004", Ok((0, 4)));
        single_parse_crockford_u64_tuple("00000000000000000000000005", Ok((0, 5)));
        single_parse_crockford_u64_tuple("00000000000000000000000006", Ok((0, 6)));
        single_parse_crockford_u64_tuple("00000000000000000000000007", Ok((0, 7)));
        single_parse_crockford_u64_tuple("00000000000000000000000008", Ok((0, 8)));
        single_parse_crockford_u64_tuple("00000000000000000000000009", Ok((0, 9)));

        single_parse_crockford_u64_tuple("0000000000000000000000000A", Ok((0, 10)));
        single_parse_crockford_u64_tuple("0000000000000000000000000B", Ok((0, 11)));
        single_parse_crockford_u64_tuple("0000000000000000000000000C", Ok((0, 12)));
        single_parse_crockford_u64_tuple("0000000000000000000000000D", Ok((0, 13)));
        single_parse_crockford_u64_tuple("0000000000000000000000000E", Ok((0, 14)));
        single_parse_crockford_u64_tuple("0000000000000000000000000F", Ok((0, 15)));
        single_parse_crockford_u64_tuple("0000000000000000000000000G", Ok((0, 16)));
        single_parse_crockford_u64_tuple("0000000000000000000000000H", Ok((0, 17)));
        single_parse_crockford_u64_tuple("0000000000000000000000000J", Ok((0, 18)));
        single_parse_crockford_u64_tuple("0000000000000000000000000K", Ok((0, 19)));
        single_parse_crockford_u64_tuple("0000000000000000000000000M", Ok((0, 20)));
        single_parse_crockford_u64_tuple("0000000000000000000000000N", Ok((0, 21)));
        single_parse_crockford_u64_tuple("0000000000000000000000000P", Ok((0, 22)));
        single_parse_crockford_u64_tuple("0000000000000000000000000Q", Ok((0, 23)));
        single_parse_crockford_u64_tuple("0000000000000000000000000R", Ok((0, 24)));
        single_parse_crockford_u64_tuple("0000000000000000000000000S", Ok((0, 25)));
        single_parse_crockford_u64_tuple("0000000000000000000000000T", Ok((0, 26)));
        single_parse_crockford_u64_tuple("0000000000000000000000000V", Ok((0, 27)));
        single_parse_crockford_u64_tuple("0000000000000000000000000W", Ok((0, 28)));
        single_parse_crockford_u64_tuple("0000000000000000000000000X", Ok((0, 29)));
        single_parse_crockford_u64_tuple("0000000000000000000000000Y", Ok((0, 30)));
        single_parse_crockford_u64_tuple("0000000000000000000000000Z", Ok((0, 31)));

        single_parse_crockford_u64_tuple("0000000000000000000000000a", Ok((0, 10)));
        single_parse_crockford_u64_tuple("0000000000000000000000000b", Ok((0, 11)));
        single_parse_crockford_u64_tuple("0000000000000000000000000c", Ok((0, 12)));
        single_parse_crockford_u64_tuple("0000000000000000000000000d", Ok((0, 13)));
        single_parse_crockford_u64_tuple("0000000000000000000000000e", Ok((0, 14)));
        single_parse_crockford_u64_tuple("0000000000000000000000000f", Ok((0, 15)));
        single_parse_crockford_u64_tuple("0000000000000000000000000g", Ok((0, 16)));
        single_parse_crockford_u64_tuple("0000000000000000000000000h", Ok((0, 17)));
        single_parse_crockford_u64_tuple("0000000000000000000000000j", Ok((0, 18)));
        single_parse_crockford_u64_tuple("0000000000000000000000000k", Ok((0, 19)));
        single_parse_crockford_u64_tuple("0000000000000000000000000m", Ok((0, 20)));
        single_parse_crockford_u64_tuple("0000000000000000000000000n", Ok((0, 21)));
        single_parse_crockford_u64_tuple("0000000000000000000000000p", Ok((0, 22)));
        single_parse_crockford_u64_tuple("0000000000000000000000000q", Ok((0, 23)));
        single_parse_crockford_u64_tuple("0000000000000000000000000r", Ok((0, 24)));
        single_parse_crockford_u64_tuple("0000000000000000000000000s", Ok((0, 25)));
        single_parse_crockford_u64_tuple("0000000000000000000000000t", Ok((0, 26)));
        single_parse_crockford_u64_tuple("0000000000000000000000000v", Ok((0, 27)));
        single_parse_crockford_u64_tuple("0000000000000000000000000w", Ok((0, 28)));
        single_parse_crockford_u64_tuple("0000000000000000000000000x", Ok((0, 29)));
        single_parse_crockford_u64_tuple("0000000000000000000000000y", Ok((0, 30)));
        single_parse_crockford_u64_tuple("0000000000000000000000000z", Ok((0, 31)));

        single_parse_crockford_u64_tuple("00000000000000000000000010", Ok((0, 32)));

        // special characters
        single_parse_crockford_u64_tuple("0000000000000000000000000o", Ok((0, 0)));
        single_parse_crockford_u64_tuple("0000000000000000000000000O", Ok((0, 0)));
        single_parse_crockford_u64_tuple("0000000000000000000000000i", Ok((0, 1)));
        single_parse_crockford_u64_tuple("0000000000000000000000000I", Ok((0, 1)));
        single_parse_crockford_u64_tuple("0000000000000000000000000l", Ok((0, 1)));
        single_parse_crockford_u64_tuple("0000000000000000000000000L", Ok((0, 1)));

        single_parse_crockford_u64_tuple(
            "00000000000000ZZZZZZZZZZZZ",
            Ok((0, 0xFFF_FFFF_FFFF_FFFF)),
        );
        single_parse_crockford_u64_tuple(
            "0000000000000FZZZZZZZZZZZZ",
            Ok((0, 0xFFFF_FFFF_FFFF_FFFF)),
        );
        single_parse_crockford_u64_tuple("0000000000000G000000000000", Ok((1, 0)));

        single_parse_crockford_u64_tuple(
            "80000000000000000000000000",
            Err(DecodingError::DataTypeOverflow),
        );

        single_parse_crockford_u64_tuple(
            "0000000000000000000000000U",
            Err(DecodingError::InvalidChar('U')),
        );

        single_parse_crockford_u64_tuple(
            "1234567890123456789012345",
            Err(DecodingError::InvalidLength),
        );
        single_parse_crockford_u64_tuple(
            "123456789012345678901234567",
            Err(DecodingError::InvalidLength),
        );
        single_parse_crockford_u64_tuple("00000000000000000000000000", Ok((0, 0)));
        single_parse_crockford_u64_tuple(
            "7ZZZZZZZZZZZZZZZZZZZZZZZZZ",
            Ok((0xFFFF_FFFF_FFFF_FFFF, 0xFFFF_FFFF_FFFF_FFFF)),
        );
        single_parse_crockford_u64_tuple(
            "0H48SM8NB6EY49KANVSKEYXW0F",
            Ok((0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F)),
        );
        single_parse_crockford_u64_tuple(
            "7G1ZQDVK5VNACRGXV6AN2368GH",
            Ok((0xF00F_EEDD_CCBB_AA99, 0x8877_6655_4433_2211)),
        );
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

    #[test]
    fn decoding_error_sources() {
        assert!(DecodingError::InvalidLength.source().is_none());
        assert!(DecodingError::InvalidChar('a').source().is_none());
        assert!(DecodingError::DataTypeOverflow.source().is_none());
    }

    fn single_append_crockford_u128(value: u128, expected_result: &str) {
        let mut a_string = String::new();
        append_crockford_u128(value, &mut a_string);
        println!("{}", a_string);
        assert_eq!(expected_result, a_string);
    }

    fn single_parse_crockford_u128(value: &str, expected_result: Result<u128, DecodingError>) {
        let result = parse_crockford_u128(value);
        println!("parse_crockford_u128({}) => {:?}", value, result);
        assert_eq!(result, expected_result);
    }

    fn single_append_crockford_u64_tuple(value: (u64, u64), expected_result: &str) {
        let mut a_string = String::new();
        append_crockford_u64_tuple(value, &mut a_string);
        println!("{}", a_string);
        assert_eq!(expected_result, a_string);
    }

    fn single_parse_crockford_u64_tuple(
        value: &str,
        expected_result: Result<(u64, u64), DecodingError>,
    ) {
        let result = parse_crockford_u64_tuple(value);
        assert_eq!(result, expected_result);
    }

    fn single_decoding_error_display_trait(error: DecodingError, expected_result: &str) {
        let result = format!("{}", error);
        assert_eq!(result, expected_result)
    }
}
