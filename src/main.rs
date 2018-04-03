extern crate chrono;
extern crate rusty_ulid;

/// Contains functions for encoding and decoding of
/// [crockford Base32][crockford] strings.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
pub mod crockford;

pub use rusty_ulid::Ulid;
pub use rusty_ulid::new_ulid_string;

fn main() {
    let ulid = Ulid::new();
    println!("{:?}", ulid);
    println!("{}", ulid);
    println!("{}", ulid.datetime());
    println!("{}", new_ulid_string());
}
