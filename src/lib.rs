/*
 * The MIT License (MIT)
 * Copyright (c) 2018-2020 Joern Huxhorn
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
 * Copyright 2018-2020 Joern Huxhorn
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

#![doc(html_root_url = "https://docs.rs/rusty_ulid/0.9.3")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! # ULID - Universally Unique Lexicographically Sortable Identifier
//!
//! UUID can be suboptimal for many uses-cases because:
//!
//! - It isn't the most character efficient way of encoding 128 bits of randomness
//! - UUID v1/v2 is impractical in many environments, as it requires access to a unique, stable MAC address
//! - UUID v3/v5 requires a unique seed and produces randomly distributed IDs, which can cause fragmentation in many data structures
//! - UUID v4 provides no other information than randomness which can cause fragmentation in many data structures
//!
//! Instead, herein is proposed [ULID][ulidspec]:
//!
//! `01ARZ3NDEKTSV4RRFFQ69G5FAV`
//!
//! - 128-bit compatibility with UUID
//! - 1.21e+24 unique ULIDs per millisecond
//! - Lexicographically sortable!
//! - Canonically encoded as a 26 character string, as opposed to the 36 character UUID
//! - Uses [Crockford's base32][crockford] for better efficiency and readability (5 bits per character)
//! - Case insensitive
//! - No special characters (URL safe)
//! - Monotonic sort order (correctly detects and handles the same millisecond)
//!
//! ## Specification
//!
//! Below is the current specification of [ULID][ulidspec] as implemented in this crate.
//!
//!
//! ```text
//!  01AN4Z07BY      79KA1307SR9X4MV3
//!
//! |----------|    |----------------|
//!  Timestamp          Randomness
//!    48bits             80bits
//! ```
//!
//! ### Components
//!
//! #### Timestamp
//! - 48 bit integer
//! - UNIX-time in milliseconds
//! - Won't run out of space until `+10889-08-02T05:31:50.655Z`.
//!
//! #### Randomness
//! - 80 bits
//! - Cryptographically secure source of randomness, if possible
//!
//! ### Sorting
//!
//! The left-most character must be sorted first, and the right-most character
//! sorted last (lexical order). The default ASCII character set must be used.
//! Within the same millisecond, sort order is not guaranteed
//!
//! ### Canonical String Representation
//!
//! ```text
//! ttttttttttrrrrrrrrrrrrrrrr
//!
//! where
//! t is Timestamp (10 characters)
//! r is Randomness (16 characters)
//! ```
//!
//! ### Encoding
//!
//! [Crockford's Base32][crockford] is used as shown.
//! This alphabet excludes the letters I, L, O, and U to avoid confusion and abuse.
//!
//! `0123456789ABCDEFGHJKMNPQRSTVWXYZ`
//!
//! ### Monotonicity
//!
//! When generating a ULID within the same millisecond, we can provide some
//! guarantees regarding sort order. Namely, if the same millisecond is detected,
//! the `random` component is incremented by 1 bit in the least significant bit position
//! (with carrying).
//!
//! If, in the extremely unlikely event that, you manage to generate more than 2<sup>80</sup>
//! ULIDs within the same millisecond, or cause the random component to overflow with less,
//! the generation will fail.
//!
//! ### Overflow Errors when Parsing Base32 Strings
//!
//! Technically, a 26-character Base32 encoded string can contain 130 bits of
//! information, whereas a ULID must only contain 128 bits. Therefore, the largest
//! valid ULID encoded in Base32 is `7ZZZZZZZZZZZZZZZZZZZZZZZZZ`, which corresponds to
//! an epoch time of `281474976710655` or `2 ^ 48 - 1`.
//!
//! Any attempt to decode or encode a ULID larger than this should be rejected by all
//! implementations, to prevent overflow bugs.
//!
//! ### Binary Layout and Byte Order
//!
//! The components are encoded as 16 octets. Each component is encoded with the
//! Most Significant Byte first (network byte order).
//!
//! ```text
//! 0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                      32_bit_uint_time_high                    |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |     16_bit_uint_time_low      |       16_bit_uint_random      |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                       32_bit_uint_random                      |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                       32_bit_uint_random                      |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! ```
//!
//! [ulidspec]: https://github.com/ulid/spec
//! [crockford]: https://crockford.com/wrmg/base32.html

#[cfg(feature = "chrono")]
use chrono::prelude::{DateTime, TimeZone, Utc};

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Contains functions for encoding and decoding of
/// [crockford Base32][crockford] strings.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
pub mod crockford;
pub use crate::crockford::DecodingError;

/// Returns the number of non-leap milliseconds since January 1, 1970 0:00:00 UTC
/// (aka "UNIX timestamp").
#[cfg(all(feature = "rand", feature = "chrono"))]
fn unix_epoch_ms() -> u64 {
    let now: DateTime<Utc> = Utc::now();

    now.timestamp_millis() as u64
}

/// Returns a new ULID string.
///
/// This function is a shortcut for `Ulid::generate().to_string()`.
///
/// # Example
/// ```
/// # use rusty_ulid::generate_ulid_string;
/// let ulid_string = generate_ulid_string();
///
/// // every ulid has exactly 26 characters
/// assert_eq!(ulid_string.len(), 26);
/// ```
#[cfg(all(feature = "rand", feature = "chrono"))]
pub fn generate_ulid_string() -> String {
    Ulid::generate().to_string()
}

/// Returns new ULID bytes.
///
/// This function is a shortcut for `Ulid::generate().into()`.
///
/// # Example
/// ```
/// # use rusty_ulid::generate_ulid_bytes;
/// let ulid_bytes = generate_ulid_bytes();
///
/// // a binary ulid has exactly 16 bytes
/// assert_eq!(ulid_bytes.len(), 16);
/// ```
#[cfg(all(feature = "rand", feature = "chrono"))]
pub fn generate_ulid_bytes() -> [u8; 16] {
    Ulid::generate().into()
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
/// The ULID data type.
pub struct Ulid {
    value: (u64, u64),
}

impl Ulid {
    /// Creates a new ULID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::generate();
    ///
    /// assert_ne!(0, ulid.timestamp());
    ///
    /// let ulid_string = ulid.to_string();
    /// // every ulid has exactly 26 characters
    /// assert_eq!(ulid_string.len(), 26);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if called after `+10889-08-02T05:31:50.655Z`.
    #[cfg(all(feature = "rand", feature = "chrono"))]
    pub fn generate() -> Ulid {
        Ulid::from_timestamp_with_rng(unix_epoch_ms(), &mut rand::thread_rng())
    }

    /// Creates the next monotonic ULID for the given `previous_ulid`.
    ///
    /// If the random part of `previous_ulid` would overflow, this function returns a ULID with
    /// the random part set to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let previous_ulid = Ulid::from(0);
    /// let ulid = Ulid::next_monotonic(previous_ulid);
    ///
    /// assert_ne!(0, ulid.timestamp());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if called after `+10889-08-02T05:31:50.655Z`.
    #[cfg(all(feature = "rand", feature = "chrono"))]
    pub fn next_monotonic(previous_ulid: Ulid) -> Ulid {
        Ulid::next_monotonic_from_timestamp_with_rng(
            previous_ulid,
            unix_epoch_ms(),
            &mut rand::thread_rng(),
        )
    }

    /// Creates the next strictly monotonic ULID for the given `previous_ulid`.
    ///
    /// If the random part of `previous_ulid` would overflow, this function returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let previous_ulid = Ulid::from(0);
    /// let ulid = Ulid::next_strictly_monotonic(previous_ulid);
    ///
    /// if let Some(ulid) = ulid {
    ///     assert_ne!(0, ulid.timestamp());
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if called after `+10889-08-02T05:31:50.655Z`.
    #[cfg(all(feature = "rand", feature = "chrono"))]
    pub fn next_strictly_monotonic(previous_ulid: Ulid) -> Option<Ulid> {
        Ulid::next_strictly_monotonic_from_timestamp_with_rng(
            previous_ulid,
            unix_epoch_ms(),
            &mut rand::thread_rng(),
        )
    }

    /// Creates a new ULID with the given `timestamp` obtaining randomness from
    /// `rng`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from_timestamp_with_rng(0, &mut rand::thread_rng());
    ///
    /// let timestamp = ulid.timestamp();
    ///
    /// assert_eq!(timestamp, 0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `timestamp` is larger than `0xFFFF_FFFF_FFFF`.
    #[cfg(feature = "rand")]
    pub fn from_timestamp_with_rng<R>(timestamp: u64, rng: &mut R) -> Ulid
    where
        R: rand::Rng,
    {
        if (timestamp & 0xFFFF_0000_0000_0000) != 0 {
            panic!("ULID does not support timestamps after +10889-08-02T05:31:50.655Z");
        }

        let high = timestamp << 16 | u64::from(rng.gen::<u16>());
        let low = rng.gen::<u64>();
        let value = (high, low);

        Ulid { value }
    }

    /// Creates the next monotonic ULID with the given `previous_ulid`, `timestamp`
    /// obtaining randomness from `rng`.
    ///
    /// If the random part of `previous_ulid` would overflow, this function returns a ULID with
    /// the random part set to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let previous_ulid = Ulid::from(0);
    /// let ulid = Ulid::next_monotonic_from_timestamp_with_rng(previous_ulid, 0, &mut rand::thread_rng());
    ///
    /// assert_eq!(ulid, Ulid::from(1));
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let previous_ulid = Ulid::from(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFE);
    /// let ulid = Ulid::next_monotonic_from_timestamp_with_rng(previous_ulid, 0, &mut rand::thread_rng());
    ///
    /// assert_eq!(ulid, Ulid::from(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFF));
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let previous_ulid = Ulid::from(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFF);
    /// let ulid = Ulid::next_monotonic_from_timestamp_with_rng(previous_ulid, 0, &mut rand::thread_rng());
    ///
    /// // overflow results in zero random part
    /// assert_eq!(ulid, Ulid::from(0));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `timestamp` is larger than `0xFFFF_FFFF_FFFF`.
    #[cfg(feature = "rand")]
    pub fn next_monotonic_from_timestamp_with_rng<R>(
        previous_ulid: Ulid,
        timestamp: u64,
        rng: &mut R,
    ) -> Ulid
    where
        R: rand::Rng,
    {
        if previous_ulid.timestamp() == timestamp {
            previous_ulid.increment()
        } else {
            Ulid::from_timestamp_with_rng(timestamp, rng)
        }
    }

    /// Creates the next strictly monotonic ULID with the given `previous_ulid`, `timestamp`
    /// obtaining randomness from `rng`.
    ///
    /// If the random part of `previous_ulid` would overflow, this function returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let previous_ulid = Ulid::from(0);
    /// let ulid = Ulid::next_strictly_monotonic_from_timestamp_with_rng(previous_ulid, 0, &mut rand::thread_rng());
    ///
    /// assert_eq!(ulid, Some(Ulid::from(1)));
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let previous_ulid = Ulid::from(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFE);
    /// let ulid = Ulid::next_strictly_monotonic_from_timestamp_with_rng(previous_ulid, 0, &mut rand::thread_rng());
    ///
    /// assert_eq!(ulid, Some(Ulid::from(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFF)));
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let previous_ulid = Ulid::from(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFF);
    /// let ulid = Ulid::next_strictly_monotonic_from_timestamp_with_rng(previous_ulid, 0, &mut rand::thread_rng());
    ///
    /// // overflow results in None
    /// assert_eq!(ulid, None);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `timestamp` is larger than `0xFFFF_FFFF_FFFF`.
    #[cfg(feature = "rand")]
    pub fn next_strictly_monotonic_from_timestamp_with_rng<R>(
        previous_ulid: Ulid,
        timestamp: u64,
        rng: &mut R,
    ) -> Option<Ulid>
    where
        R: rand::Rng,
    {
        let result = Ulid::next_monotonic_from_timestamp_with_rng(previous_ulid, timestamp, rng);

        if previous_ulid < result {
            Some(result)
        } else {
            None
        }
    }

    /// Returns the timestamp of this ULID as number
    /// of non-leap milliseconds since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    /// use std::str::FromStr;
    ///
    /// let ulid = Ulid::from_str("01CAH7NXGRDJNE9B1NY7PQGYV7")?;
    /// let timestamp = ulid.timestamp();
    ///
    /// assert_eq!(timestamp, 1523144390168);
    /// # Ok::<(), rusty_ulid::DecodingError>(())
    /// ```
    pub fn timestamp(&self) -> u64 {
        (self.value.0 >> 16) as u64
    }

    /// Returns the timestamp of this ULID as a `DateTime<Utc>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    /// use std::str::FromStr;
    ///
    /// let ulid = Ulid::from_str("01CAH7NXGRDJNE9B1NY7PQGYV7")?;
    /// let datetime = ulid.datetime();
    ///
    /// assert_eq!(datetime.to_string(), "2018-04-07 23:39:50.168 UTC");
    /// # Ok::<(), rusty_ulid::DecodingError>(())
    /// ```
    #[cfg(feature = "chrono")]
    pub fn datetime(&self) -> DateTime<Utc> {
        let timestamp = self.timestamp();
        let seconds: i64 = (timestamp / 1000) as i64;
        let nanos: u32 = ((timestamp % 1000) * 1_000_000) as u32;

        Utc.timestamp(seconds, nanos)
    }

    /// Returns a new ULID with the random part incremented by one.
    ///
    /// Overflowing the random part resets it to zero without influencing
    /// the timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from(0);
    /// let incremented = ulid.increment();
    ///
    /// assert_eq!(incremented, Ulid::from(1));
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFE);
    /// let incremented = ulid.increment();
    ///
    /// assert_eq!(incremented, Ulid::from(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFF));
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFF);
    /// let incremented = ulid.increment();
    ///
    /// assert_eq!(incremented, Ulid::from(0));
    /// ```
    pub fn increment(self) -> Ulid {
        const TIMESTAMP_PART_MASK: u128 = 0xFFFF_FFFF_FFFF_0000_0000_0000_0000_0000;
        const RANDOM_PART_MASK: u128 = !TIMESTAMP_PART_MASK;

        let value: u128 = self.into();

        if value & RANDOM_PART_MASK == RANDOM_PART_MASK {
            // overflow, set random part to zero
            (value & TIMESTAMP_PART_MASK).into()
        } else {
            (value + 1).into()
        }
    }

    /// Returns the string representaton of this ULID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from(0);
    ///
    /// assert_eq!(ulid.to_string(), "00000000000000000000000000");
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
    ///
    /// assert_eq!(ulid.to_string(), "7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
    /// ```
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::inherent_to_string_shadow_display)]
    // impl fmt::Display is using this method
    // https://github.com/rust-lang/rust-clippy/issues/4396
    pub fn to_string(&self) -> String {
        let mut string = String::with_capacity(26);

        crockford::append_crockford_u64_tuple(self.value, &mut string);

        string
    }
}

impl fmt::Display for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.to_string())
    }
}

impl FromStr for Ulid {
    type Err = crockford::DecodingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = crockford::parse_crockford_u64_tuple(s)?;

        Ok(Ulid { value })
    }
}

impl From<[u8; 16]> for Ulid {
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let bytes: [u8; 16] = [
    ///     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ///     0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xF0, 0x0F,
    /// ];
    ///
    /// let ulid = Ulid::from(bytes);
    ///
    /// let expected_ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let bytes: [u8; 16] = [
    ///     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ///     0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xF0, 0x0F,
    /// ];
    ///
    /// let ulid : Ulid = bytes.into();
    ///
    /// let expected_ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    fn from(bytes: [u8; 16]) -> Self {
        #[rustfmt::skip]
        let high = u64::from(bytes[0]) << 56
            | u64::from(bytes[1]) << 48
            | u64::from(bytes[2]) << 40
            | u64::from(bytes[3]) << 32
            | u64::from(bytes[4]) << 24
            | u64::from(bytes[5]) << 16
            | u64::from(bytes[6]) << 8
            | u64::from(bytes[7]);

        #[rustfmt::skip]
        let low = u64::from(bytes[8]) << 56
            | u64::from(bytes[9]) << 48
            | u64::from(bytes[10]) << 40
            | u64::from(bytes[11]) << 32
            | u64::from(bytes[12]) << 24
            | u64::from(bytes[13]) << 16
            | u64::from(bytes[14]) << 8
            | u64::from(bytes[15]);

        let value = (high, low);
        Ulid { value }
    }
}

impl From<Ulid> for [u8; 16] {
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// let bytes = <[u8; 16]>::from(ulid);
    ///
    /// let expected_bytes: [u8; 16] = [
    ///     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ///     0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xF0, 0x0F,
    /// ];
    ///
    /// assert_eq!(bytes, expected_bytes);
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// let bytes: [u8; 16] = ulid.into();
    ///
    /// let expected_bytes: [u8; 16] = [
    ///     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ///     0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xF0, 0x0F,
    /// ];
    ///
    /// assert_eq!(bytes, expected_bytes);
    /// ```
    #[rustfmt::skip]
    fn from(ulid: Ulid) -> Self {
        let value = ulid.value;

        [
            ((value.0 >> 56) & 0xff) as u8,
            ((value.0 >> 48) & 0xff) as u8,
            ((value.0 >> 40) & 0xff) as u8,
            ((value.0 >> 32) & 0xff) as u8,
            ((value.0 >> 24) & 0xff) as u8,
            ((value.0 >> 16) & 0xff) as u8,
            ((value.0 >> 8) & 0xff) as u8,
            (value.0 & 0xff) as u8,
            ((value.1 >> 56) & 0xff) as u8,
            ((value.1 >> 48) & 0xff) as u8,
            ((value.1 >> 40) & 0xff) as u8,
            ((value.1 >> 32) & 0xff) as u8,
            ((value.1 >> 24) & 0xff) as u8,
            ((value.1 >> 16) & 0xff) as u8,
            ((value.1 >> 8) & 0xff) as u8,
            (value.1 & 0xff) as u8,
        ]
    }
}

impl From<(u64, u64)> for Ulid {
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let tuple = (0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// let ulid = Ulid::from(tuple);
    ///
    /// let expected_ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let tuple = (0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// let ulid : Ulid = tuple.into();
    ///
    /// let expected_ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    fn from(value: (u64, u64)) -> Self {
        Ulid { value }
    }
}

impl From<Ulid> for (u64, u64) {
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// let tuple = <(u64, u64)>::from(ulid);
    ///
    /// let expected_tuple = (0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(tuple, expected_tuple);
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// let tuple : (u64, u64) = ulid.into();
    ///
    /// let expected_tuple = (0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(tuple, expected_tuple);
    /// ```
    fn from(ulid: Ulid) -> Self {
        ulid.value
    }
}

impl From<u128> for Ulid {
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let value = 0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F;
    ///
    /// let ulid = Ulid::from(value);
    ///
    /// let expected_ulid = Ulid::from((0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F));
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let value = 0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F;
    ///
    /// let ulid : Ulid = value.into();
    ///
    /// let expected_ulid = Ulid::from((0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F));
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    fn from(value: u128) -> Self {
        let value = ((value >> 64) as u64, (value & 0xFFFF_FFFF_FFFF_FFFF) as u64);
        Ulid { value }
    }
}

impl From<Ulid> for u128 {
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from((0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F));
    ///
    /// let value = <u128>::from(ulid);
    ///
    /// let expected_value = 0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F;
    ///
    /// assert_eq!(value, expected_value);
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    ///
    /// let ulid = Ulid::from((0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F));
    ///
    /// let value : u128 = ulid.into();
    ///
    /// let expected_value = 0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F;
    ///
    /// assert_eq!(value, expected_value);
    /// ```
    fn from(ulid: Ulid) -> Self {
        u128::from(ulid.value.0) << 64 | u128::from(ulid.value.1)
    }
}

impl TryFrom<&[u8]> for Ulid {
    type Error = DecodingError;

    /// Returns a ULID for the given slice of bytes or `DecodingError::InvalidLength`
    /// if the slice does not contain exactly 16 bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    /// use std::convert::TryFrom;
    /// use std::convert::TryInto;
    ///
    /// let bytes: [u8; 18] = [
    ///     0x00,
    ///     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ///     0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xF0, 0x0F,
    ///     0x00,
    /// ];
    ///
    /// let ulid : Ulid = Ulid::try_from(&bytes[1..17])?;
    ///
    /// let expected_ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    ///
    /// let ulid : Ulid = (&bytes[1..17]).try_into()?;
    ///
    /// let expected_ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// # Ok::<(), rusty_ulid::DecodingError>(())
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    /// use rusty_ulid::DecodingError;
    /// use std::convert::TryFrom;
    ///
    /// let mut bytes: [u8; 17] = [0; 17];
    /// let result = Ulid::try_from(&bytes[0..]);
    ///
    /// assert_eq!(result, Err(DecodingError::InvalidLength))
    /// ```
    ///
    /// ```
    /// use rusty_ulid::Ulid;
    /// use rusty_ulid::DecodingError;
    /// use std::convert::TryFrom;
    ///
    /// let mut bytes: [u8; 15] = [0; 15];
    /// let result = Ulid::try_from(&bytes[0..]);
    ///
    /// assert_eq!(result, Err(DecodingError::InvalidLength))
    /// ```
    fn try_from(bytes: &[u8]) -> Result<Ulid, DecodingError> {
        if bytes.len() != 16 {
            return Err(DecodingError::InvalidLength);
        }

        #[rustfmt::skip]
        let high = u64::from(bytes[0]) << 56
            | u64::from(bytes[1]) << 48
            | u64::from(bytes[2]) << 40
            | u64::from(bytes[3]) << 32
            | u64::from(bytes[4]) << 24
            | u64::from(bytes[5]) << 16
            | u64::from(bytes[6]) << 8
            | u64::from(bytes[7]);

        #[rustfmt::skip]
        let low = u64::from(bytes[8]) << 56
            | u64::from(bytes[9]) << 48
            | u64::from(bytes[10]) << 40
            | u64::from(bytes[11]) << 32
            | u64::from(bytes[12]) << 24
            | u64::from(bytes[13]) << 16
            | u64::from(bytes[14]) << 8
            | u64::from(bytes[15]);

        let value = (high, low);
        Ok(Ulid { value })
    }
}

#[cfg(feature = "serde")]
impl Serialize for Ulid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            let bytes: [u8; 16] = self.clone().into();
            serializer.serialize_bytes(&bytes)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Ulid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct UlidStringVisitor;

            impl<'vi> de::Visitor<'vi> for UlidStringVisitor {
                type Value = Ulid;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(formatter, "a ULID string")
                }

                fn visit_str<E: de::Error>(self, value: &str) -> Result<Ulid, E> {
                    value.parse::<Ulid>().map_err(E::custom)
                }
            }

            deserializer.deserialize_str(UlidStringVisitor)
        } else {
            struct UlidBytesVisitor;

            impl<'vi> de::Visitor<'vi> for UlidBytesVisitor {
                type Value = Ulid;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(formatter, "16 ULID bytes")
                }

                fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Ulid, E> {
                    Ulid::try_from(value).map_err(E::custom)
                }
            }
            deserializer.deserialize_bytes(UlidBytesVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAST_TIMESTAMP: u64 = 1_481_195_424_879;
    const PAST_TIMESTAMP_PART: &str = "01B3F2133F";

    const MAX_TIMESTAMP: u64 = 0xFFFF_FFFF_FFFF;
    const MAX_TIMESTAMP_PART: &str = "7ZZZZZZZZZ";

    const MIN_TIMESTAMP: u64 = 0;
    const MIN_TIMESTAMP_PART: &str = "0000000000";

    #[test]
    fn increment() {
        single_increment(0x0000_0000_0000_0000_0000_0000_0000_0000, Ulid::from(1));
        single_increment(
            0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFE,
            Ulid::from(0xFFFF_FFFF_FFFF_FFFF_FFFF),
        );
        single_increment(0x0000_0000_0000_FFFF_FFFF_FFFF_FFFF_FFFF, Ulid::from(0));
        single_increment(
            0x0000_0000_0001_0000_0000_0000_0000_0000,
            Ulid::from(0x0000_0000_0001_0000_0000_0000_0000_0001),
        );
        single_increment(
            0x0000_0000_0001_FFFF_FFFF_FFFF_FFFF_FFFF,
            Ulid::from(0x0000_0000_0001_0000_0000_0000_0000_0000),
        );
    }

    fn single_increment(input: u128, expected_result: Ulid) {
        let input_value: Ulid = input.into();
        let incremented = input_value.increment();

        assert_eq!(incremented, expected_result);
        assert_eq!(input_value.timestamp(), incremented.timestamp());
    }

    #[test]
    fn from_string_to_string() {
        single_from_string_to_string(
            &(PAST_TIMESTAMP_PART.to_owned() + "0000000000000000"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string(
            &(PAST_TIMESTAMP_PART.to_owned() + "ZZZZZZZZZZZZZZZZ"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string(
            &(PAST_TIMESTAMP_PART.to_owned() + "123456789ABCDEFG"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string(
            &(PAST_TIMESTAMP_PART.to_owned() + "1000000000000000"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string(
            &(PAST_TIMESTAMP_PART.to_owned() + "1000000000000001"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string(
            &(PAST_TIMESTAMP_PART.to_owned() + "0001000000000001"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string(
            &(PAST_TIMESTAMP_PART.to_owned() + "0100000000000001"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string(
            &(PAST_TIMESTAMP_PART.to_owned() + "0000000000000001"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string(
            &(MAX_TIMESTAMP_PART.to_owned() + "123456789ABCDEFG"),
            MAX_TIMESTAMP,
        );
        single_from_string_to_string(
            &(MIN_TIMESTAMP_PART.to_owned() + "123456789ABCDEFG"),
            MIN_TIMESTAMP,
        );

        let largest_legal_ulid_string = "7ZZZZZZZZZZZZZZZZZZZZZZZZZ";
        single_from_string_to_string(largest_legal_ulid_string, MAX_TIMESTAMP);
    }

    fn single_from_string_to_string(s: &str, timestamp: u64) {
        let ulid = Ulid::from_str(s).unwrap();
        assert_eq!(ulid.timestamp(), timestamp);
        assert_eq!(ulid.to_string(), s);
    }

    #[test]
    fn from_string_to_string_special_cases() {
        single_from_string_to_string_special_case(
            &(PAST_TIMESTAMP_PART.to_owned() + "00i0000000000000"),
            &(PAST_TIMESTAMP_PART.to_owned() + "0010000000000000"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string_special_case(
            &(PAST_TIMESTAMP_PART.to_owned() + "00I0000000000000"),
            &(PAST_TIMESTAMP_PART.to_owned() + "0010000000000000"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string_special_case(
            &(PAST_TIMESTAMP_PART.to_owned() + "00l0000000000000"),
            &(PAST_TIMESTAMP_PART.to_owned() + "0010000000000000"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string_special_case(
            &(PAST_TIMESTAMP_PART.to_owned() + "00L0000000000000"),
            &(PAST_TIMESTAMP_PART.to_owned() + "0010000000000000"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string_special_case(
            &(PAST_TIMESTAMP_PART.to_owned() + "00o0000000000000"),
            &(PAST_TIMESTAMP_PART.to_owned() + "0000000000000000"),
            PAST_TIMESTAMP,
        );
        single_from_string_to_string_special_case(
            &(PAST_TIMESTAMP_PART.to_owned() + "00O0000000000000"),
            &(PAST_TIMESTAMP_PART.to_owned() + "0000000000000000"),
            PAST_TIMESTAMP,
        );
    }

    fn single_from_string_to_string_special_case(s: &str, expected: &str, timestamp: u64) {
        let ulid = Ulid::from_str(s).unwrap();
        assert_eq!(ulid.timestamp(), timestamp);
        assert_eq!(ulid.to_string(), expected);
    }

    #[test]
    fn from_str_failure_too_long() {
        let result = Ulid::from_str("123456789012345678901234567");
        assert_eq!(result, Err(DecodingError::InvalidLength));
    }

    #[test]
    fn from_str_failure_too_short() {
        let result = Ulid::from_str("1234567890123456789012345");
        assert_eq!(result, Err(DecodingError::InvalidLength));
    }

    #[test]
    fn from_str_failure_invalid_unicode() {
        let string = "012345678🦀0123456789012";
        let result = Ulid::from_str(string);
        assert_eq!(result, Err(DecodingError::InvalidChar('🦀')));
    }

    #[test]
    fn from_str_failure_overflow() {
        let smallest_overflowing_ulid_string = "80000000000000000000000000";
        let result = Ulid::from_str(smallest_overflowing_ulid_string);
        assert_eq!(result, Err(DecodingError::DataTypeOverflow));
    }

    #[test]
    fn eq_cmp_sanity_checks() {
        // yes, this is pretty paranoid.

        use std::cmp::Ordering;

        let ulid_one_low: Ulid = (0, 1).into();
        let ulid_two_low: Ulid = (0, 2).into();
        let ulid_one_high: Ulid = (1, 0).into();

        let ulid_one_low_other: Ulid = (0, 1).into();

        assert_eq!(ulid_one_low.eq(&ulid_one_low), true);
        assert_eq!(ulid_one_low.cmp(&ulid_one_low), Ordering::Equal);

        assert!(ulid_one_low == ulid_one_low_other);
        assert_eq!(ulid_one_low.eq(&ulid_one_low_other), true);
        assert_eq!(ulid_one_low.cmp(&ulid_one_low_other), Ordering::Equal);

        assert!(ulid_one_low != ulid_two_low);
        assert!(ulid_two_low != ulid_one_low);
        assert!(ulid_one_low < ulid_two_low);
        assert!(ulid_two_low > ulid_one_low);
        assert_eq!(ulid_one_low.eq(&ulid_two_low), false);
        assert_eq!(ulid_two_low.eq(&ulid_one_low), false);
        assert_eq!(ulid_one_low.cmp(&ulid_two_low), Ordering::Less);
        assert_eq!(ulid_two_low.cmp(&ulid_one_low), Ordering::Greater);

        assert!(ulid_one_low != ulid_one_high);
        assert!(ulid_one_high != ulid_one_low);
        assert_eq!(ulid_one_low.eq(&ulid_one_high), false);
        assert_eq!(ulid_one_high.eq(&ulid_one_low), false);
        assert_eq!(ulid_one_low.cmp(&ulid_one_high), Ordering::Less);
        assert_eq!(ulid_one_high.cmp(&ulid_one_low), Ordering::Greater);
    }

    #[test]
    fn hash_sanity_checks() {
        // yes, this is also pretty paranoid.

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let ulid_one_low: Ulid = (0, 1).into();
        let ulid_two_low: Ulid = (0, 2).into();
        let ulid_one_high: Ulid = (1, 0).into();

        let ulid_one_low_other: Ulid = (0, 1).into();

        let mut hasher_one_low = DefaultHasher::new();
        ulid_one_low.hash(&mut hasher_one_low);
        let hash_one_low = hasher_one_low.finish();

        let mut hasher_one_low_other = DefaultHasher::new();
        ulid_one_low_other.hash(&mut hasher_one_low_other);
        let hash_one_low_other = hasher_one_low_other.finish();

        let mut hasher_two_low = DefaultHasher::new();
        ulid_two_low.hash(&mut hasher_two_low);
        let hash_two_low = hasher_two_low.finish();

        let mut hasher_one_high = DefaultHasher::new();
        ulid_one_high.hash(&mut hasher_one_high);
        let hash_one_high = hasher_one_high.finish();

        // this must be true
        assert_eq!(hash_one_low, hash_one_low_other);

        // this should be true in case of a reasonable DefaultHasher implementation
        assert_ne!(hash_one_low, hash_two_low);
        assert_ne!(hash_one_low, hash_one_high);
    }

    #[cfg(not(miri))] // expected panic
    #[cfg(feature = "rand")]
    #[test]
    #[should_panic(expected = "ULID does not support timestamps after +10889-08-02T05:31:50.655Z")]
    fn y10889_bug() {
        use rand::rngs::mock::StepRng;

        let mut mock_rng = StepRng::new(0, 0);
        Ulid::from_timestamp_with_rng(0x0001_0000_0000_0000, &mut mock_rng);
    }

    #[cfg(feature = "rand")]
    #[test]
    fn test_from_timestamp_with_rng() {
        use rand::rngs::mock::StepRng;

        let mut mock_rng = StepRng::new(0, 0);
        let ulid = Ulid::from_timestamp_with_rng(0xFFFF_FFFF_FFFF, &mut mock_rng);

        let ulid_value: u128 = ulid.into();

        assert_eq!(ulid_value, 0xFFFF_FFFF_FFFF_0000_0000_0000_0000_0000);

        let mut mock_rng = StepRng::new(0xF00F, 0);
        let ulid = Ulid::from_timestamp_with_rng(0, &mut mock_rng);

        let ulid_value: u128 = ulid.into();

        assert_eq!(ulid_value, 0x0000_0000_0000_F00F_0000_0000_0000_F00F);
    }
}

#[cfg(all(feature = "doc-comment", feature = "rand", feature = "chrono"))]
mod doc_tests {
    use doc_comment::doctest;
    doctest!("../README.md", readme);
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;
    use serde_test::*;

    #[test]
    fn test_serde_readable() {
        use serde_test::Configure;

        let ulid = Ulid::from_str("7ZZZZZZZZZZZZZZZZZZZZZZZZZ").unwrap();
        assert_tokens(
            &ulid.readable(),
            &[Token::Str("7ZZZZZZZZZZZZZZZZZZZZZZZZZ")],
        );

        let ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
        assert_tokens(
            &ulid.readable(),
            &[Token::Str("0H48SM8NB6EY49KANVSKEYXW0F")],
        );
    }

    #[test]
    fn test_serde_compact() {
        use serde_test::Configure;

        let ulid = Ulid::from_str("7ZZZZZZZZZZZZZZZZZZZZZZZZZ").unwrap();
        assert_tokens(
            &ulid.compact(),
            &[Token::Bytes(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ])],
        );

        let ulid = Ulid::from(0x1122_3344_5566_7788_99AA_BBCC_DDEE_F00F);
        assert_tokens(
            &ulid.compact(),
            &[Token::Bytes(&[
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
                0xF0, 0x0F,
            ])],
        );
    }

    #[test]
    fn test_de_readable_error() {
        assert_de_tokens_error::<Readable<Ulid>>(
            &[Token::Bytes(&[
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
                0xF0, 0x0F,
            ])],
            "invalid type: byte array, expected a ULID string",
        );

        assert_de_tokens_error::<Readable<Ulid>>(
            &[Token::Str("0H48SM8NB6EY49KANUSKEYXW0F")],
            "invalid character 'U'",
        );

        assert_de_tokens_error::<Readable<Ulid>>(
            &[Token::Str("0H48SM8NB6EY49KANVSKEYXW0FF")],
            "invalid length",
        );

        assert_de_tokens_error::<Readable<Ulid>>(
            &[Token::Str("0H48SM8NB6EY49KANVSKEYXW0")],
            "invalid length",
        );

        assert_de_tokens_error::<Readable<Ulid>>(
            &[Token::Str("80000000000000000000000000")],
            "data type overflow",
        );
    }

    #[test]
    fn test_de_compact_error() {
        assert_de_tokens_error::<Compact<Ulid>>(
            &[Token::Str("0H48SM8NB6EY49KANVSKEYXW0F")],
            "invalid type: string \"0H48SM8NB6EY49KANVSKEYXW0F\", expected 16 ULID bytes",
        );

        assert_de_tokens_error::<Compact<Ulid>>(
            &[Token::Bytes(&[
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
                0xF0, 0x0F, 0xFF,
            ])],
            "invalid length",
        );

        assert_de_tokens_error::<Compact<Ulid>>(
            &[Token::Bytes(&[
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
                0xF0,
            ])],
            "invalid length",
        );
    }
}
