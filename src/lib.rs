#![warn(missing_docs)]
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
//! - Won't run out of space till the year 10895 AD.
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
//! #### Encoding
//!
//! [Crockford's Base32][crockford] is used as shown.
//! This alphabet excludes the letters I, L, O, and U to avoid confusion and abuse.
//!
//! `0123456789ABCDEFGHJKMNPQRSTVWXYZ`
//!
//! [ulidspec]: https://github.com/ulid/spec
//! [crockford]: https://crockford.com/wrmg/base32.html
extern crate chrono;
extern crate rand;

use std::fmt;
use std::str::FromStr;
use chrono::prelude::{DateTime, TimeZone, Utc};

/// Contains functions for encoding and decoding of
/// [crockford Base32][crockford] strings.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
pub mod crockford;
pub use crockford::DecodingError;

/// Returns the number of non-leap milliseconds since January 1, 1970 0:00:00 UTC
/// (aka "UNIX timestamp").
fn unix_epoch_ms() -> u64 {
    let now: DateTime<Utc> = Utc::now();

    now.timestamp() as u64 * 1000 + u64::from(now.timestamp_subsec_millis())
}

/// Returns a new ULID string.
///
/// This function is a shortcut for `Ulid::new().to_string()`.
///
/// # Example
/// ```
/// # use rusty_ulid::new_ulid_string;
/// let ulid_string = new_ulid_string();
///
/// // every ulid has exactly 26 characters
/// assert_eq!(ulid_string.len(), 26);
/// ```
pub fn new_ulid_string() -> String {
    Ulid::new().to_string()
}

/// Returns new ULID bytes.
///
/// This function is a shortcut for `Ulid::new().into()`.
///
/// # Example
/// ```
/// # use rusty_ulid::new_ulid_bytes;
/// let ulid_bytes = new_ulid_bytes();
///
/// // a binary ulid has exactly 16 bytes
/// assert_eq!(ulid_bytes.len(), 16);
/// ```
pub fn new_ulid_bytes() -> [u8; 16] {
    Ulid::new().into()
}

#[derive(Debug, Default, PartialOrd, PartialEq, Copy, Clone)]
/// The ULID data type.
pub struct Ulid(pub u64, pub u64);

impl Ulid {
    /// Creates a new ULID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let ulid = Ulid::new();
    ///
    /// // ulid.0 contains the timestamp so it will never be 0.
    /// assert_ne!(0, ulid.0);
    ///
    /// let ulid_string = ulid.to_string();
    /// // every ulid has exactly 26 characters
    /// assert_eq!(ulid_string.len(), 26);
    /// ```
    pub fn new() -> Ulid {
        Ulid::from_timestamp_with_rng(unix_epoch_ms(), &mut rand::thread_rng())
    }

    /// Creates a new ULID with the given `timestamp` obtaining randomness from
    /// `rng`.
    ///
    /// ```
    /// // TODO: works in nightly and beta but not in stable. wat?
    /// // https://users.rust-lang.org/t/i-have-a-strange-documentation-test-issue-related-to-extern-crate/16709
    /// /*
    /// extern crate rand;
    /// # extern crate rusty_ulid;
    /// # use rusty_ulid::Ulid;
    /// let ulid = Ulid::from_timestamp_with_rng(0, &mut rand::thread_rng());
    ///
    /// let timestamp = ulid.timestamp();
    /// assert_eq!(timestamp, 0);
    /// */
    /// ```
    pub fn from_timestamp_with_rng<R>(timestamp: u64, rng: &mut R) -> Ulid
    where
        R: rand::Rng,
    {
        Ulid(
            timestamp << 16 | u64::from(rng.gen::<u16>()),
            rng.gen::<u64>(),
        )
    }

    /// Returns the timestamp of this ULID as number
    /// of non-leap milliseconds since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    /// ```
    /// # use std::error::Error;
    /// # use rusty_ulid::Ulid;
    /// # use std::str::FromStr;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let ulid = Ulid::from_str("01CAH7NXGRDJNE9B1NY7PQGYV7");
    /// let timestamp = ulid?.timestamp();
    /// assert_eq!(timestamp, 1523144390168);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn timestamp(&self) -> u64 {
        self.0 >> 16
    }

    /// Returns the timestamp of this ULID as a `DateTime<Utc>`.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use rusty_ulid::Ulid;
    /// # use std::str::FromStr;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// let ulid = Ulid::from_str("01CAH7NXGRDJNE9B1NY7PQGYV7");
    /// let datetime = ulid?.datetime();
    /// assert_eq!(datetime.to_string(), "2018-04-07 23:39:50.168 UTC");
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn datetime(&self) -> DateTime<Utc> {
        let timestamp = self.timestamp();
        let seconds: i64 = (timestamp / 1000) as i64;
        let nanos: u32 = ((timestamp % 1000) * 1_000_000) as u32;

        Utc.timestamp(seconds, nanos)
    }

    /// Returns the string representaton of this ULID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let ulid = Ulid(0, 0);
    /// assert_eq!(ulid.to_string(), "00000000000000000000000000");
    /// ```
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let ulid = Ulid(0xFFFF_FFFF_FFFF_FFFF, 0xFFFF_FFFF_FFFF_FFFF);
    /// assert_eq!(ulid.to_string(), "7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
    /// ```
    pub fn to_string(&self) -> String {
        let mut string = String::with_capacity(26);

        crockford::append_crockford(self.timestamp(), 10, &mut string);
        crockford::append_crockford(((self.0 & 0xFFFF) << 24) | (self.1 >> 40), 8, &mut string);
        crockford::append_crockford(self.1, 8, &mut string);

        string
    }
}

impl fmt::Display for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for Ulid {
    type Err = crockford::DecodingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 26 {
            return Err(crockford::DecodingError::InvalidLength);
        }

        let mut high: u64;
        let mut low: u64;

        if let Some(time_string) = s.get(0..10) {
            high = crockford::parse_crockford(time_string)? << 16;
        } else {
            return Err(crockford::DecodingError::InvalidChar(None));
        }

        if let Some(part_string) = s.get(10..18) {
            let part = crockford::parse_crockford(part_string)?;
            high |= part >> 24;
            low = part << 40;
        } else {
            return Err(crockford::DecodingError::InvalidChar(None));
        }

        if let Some(part_string) = s.get(18..) {
            let part = crockford::parse_crockford(part_string)?;
            low |= part;
        } else {
            // I have no idea how to cause this error.
            return Err(crockford::DecodingError::InvalidChar(None));
        }

        Ok(Ulid(high, low))
    }
}

impl From<[u8; 16]> for Ulid {
    /// # Examples
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let bytes: [u8; 16] = [
    ///     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ///     0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xF0, 0x0F,
    /// ];
    ///
    /// let ulid = Ulid::from(bytes);
    ///
    /// let expected_ulid = Ulid(0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let bytes: [u8; 16] = [
    ///     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    ///     0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xF0, 0x0F,
    /// ];
    ///
    /// let ulid : Ulid = bytes.into();
    ///
    /// let expected_ulid = Ulid(0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    fn from(bytes: [u8; 16]) -> Self {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let high = u64::from(bytes[0]) << 56
            | u64::from(bytes[1]) << 48
            | u64::from(bytes[2]) << 40
            | u64::from(bytes[3]) << 32
            | u64::from(bytes[4]) << 24
            | u64::from(bytes[5]) << 16
            | u64::from(bytes[6]) << 8
            | u64::from(bytes[7]);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let low = u64::from(bytes[8]) << 56
            | u64::from(bytes[9]) << 48
            | u64::from(bytes[10]) << 40
            | u64::from(bytes[11]) << 32
            | u64::from(bytes[12]) << 24
            | u64::from(bytes[13]) << 16
            | u64::from(bytes[14]) << 8
            | u64::from(bytes[15]);

        Ulid(high, low)
    }
}

impl From<Ulid> for [u8; 16] {
    /// # Examples
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let ulid = Ulid(0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
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
    /// # use rusty_ulid::Ulid;
    /// let ulid = Ulid(0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
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
    fn from(ulid: Ulid) -> Self {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        [
            ((ulid.0 >> 56) & 0xff) as u8,
            ((ulid.0 >> 48) & 0xff) as u8,
            ((ulid.0 >> 40) & 0xff) as u8,
            ((ulid.0 >> 32) & 0xff) as u8,
            ((ulid.0 >> 24) & 0xff) as u8,
            ((ulid.0 >> 16) & 0xff) as u8,
            ((ulid.0 >> 8) & 0xff) as u8,
            (ulid.0 & 0xff) as u8,

            ((ulid.1 >> 56) & 0xff) as u8,
            ((ulid.1 >> 48) & 0xff) as u8,
            ((ulid.1 >> 40) & 0xff) as u8,
            ((ulid.1 >> 32) & 0xff) as u8,
            ((ulid.1 >> 24) & 0xff) as u8,
            ((ulid.1 >> 16) & 0xff) as u8,
            ((ulid.1 >> 8) & 0xff) as u8,
            (ulid.1 & 0xff) as u8,
        ]
    }
}

impl From<(u64, u64)> for Ulid {
    /// # Examples
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let tuple = (0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// let ulid = Ulid::from(tuple);
    ///
    /// let expected_ulid = Ulid(0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let tuple = (0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// let ulid : Ulid = tuple.into();
    ///
    /// let expected_ulid = Ulid(0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(ulid, expected_ulid);
    /// ```
    fn from(tuple: (u64, u64)) -> Self {
        Ulid(tuple.0, tuple.1)
    }
}

impl From<Ulid> for (u64, u64) {
    /// # Examples
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let ulid = Ulid(0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// let tuple = <(u64, u64)>::from(ulid);
    ///
    /// let expected_tuple = (0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(tuple, expected_tuple);
    /// ```
    ///
    /// ```
    /// # use rusty_ulid::Ulid;
    /// let ulid = Ulid(0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// let tuple : (u64, u64) = ulid.into();
    ///
    /// let expected_tuple = (0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_F00F);
    ///
    /// assert_eq!(tuple, expected_tuple);
    /// ```
    fn from(ulid: Ulid) -> Self {
        (ulid.0, ulid.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAST_TIMESTAMP: u64 = 1481195424879;
    const PAST_TIMESTAMP_PART: &str = "01B3F2133F";

    const MAX_TIMESTAMP: u64 = 0xFFFF_FFFF_FFFF;
    const MAX_TIMESTAMP_PART: &str = "7ZZZZZZZZZ";

    const MIN_TIMESTAMP: u64 = 0;
    const MIN_TIMESTAMP_PART: &str = "0000000000";

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
        let result = Ulid::from_str("012345678901234567890123456");
        assert_eq!(result, Err(DecodingError::InvalidLength));
    }

    #[test]
    fn from_str_failure_too_short() {
        let result = Ulid::from_str("0123456789012345678901234");
        assert_eq!(result, Err(DecodingError::InvalidLength));
    }

    #[test]
    fn from_str_failure_split_1() {
        let string = "012345678🦀0123456789012";
        let result = Ulid::from_str(string);
        assert_eq!(result, Err(DecodingError::InvalidChar(None)));
    }

    #[test]
    fn from_str_failure_split_2() {
        let string = "01234567890123456🦀89012";
        let result = Ulid::from_str(string);
        assert_eq!(result, Err(DecodingError::InvalidChar(None)));
    }

    #[test]
    fn quickstart() {
        // Generate a ULID
        let ulid = Ulid::new();

        // Generate a string for a ULID
        let ulid_string = ulid.to_string();

        // Create ULID from a string
        let result = Ulid::from_str(&ulid_string);

        assert_eq!(Ok(ulid), result);
    }

    #[test]
    fn fn_quickstart() {
        // Generate a ULID string
        let ulid_string: String = new_ulid_string();
        assert_eq!(ulid_string.len(), 26);

        // Generate ULID bytes
        let ulid_bytes: [u8; 16] = new_ulid_bytes();
        assert_eq!(ulid_bytes.len(), 16);
    }

    /*
    StepRng requires rand 0.5
    #[test]
    fn test_from_rng() {
        let mut mock_rng = StepRng::new(2, 1);
        let ulid = Ulid::from_timestamp_with_rng(0, mock_rng);
        let ulid2 = Ulid::from_timestamp_with_rng(0, mock_rng);

        assert_eq!(ulid, ulid2);
    }
    */
}
