#![no_main]
use libfuzzer_sys::fuzz_target;
use rusty_ulid::Ulid;

fuzz_target!(|data: [u8; 16]| {
	let _ : Ulid = data.into();
});
