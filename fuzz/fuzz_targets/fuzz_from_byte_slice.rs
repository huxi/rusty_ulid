#![no_main]
use libfuzzer_sys::fuzz_target;
use rusty_ulid::Ulid;
use std::convert::TryFrom;

fuzz_target!(|data: &[u8]| {
	let _ = Ulid::try_from(data);
});
