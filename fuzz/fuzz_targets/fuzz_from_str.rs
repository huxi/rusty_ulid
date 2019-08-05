#![no_main]
use libfuzzer_sys::fuzz_target;
use rusty_ulid::Ulid;
use std::str::FromStr;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Ulid::from_str(s);
    }
});
