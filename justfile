# performs a build with the default toolchain
default: (build "")

# performs a build with the given toolchain
build toolchain:
	cargo {{toolchain}} --version
	cargo {{toolchain}} build --verbose
	cargo {{toolchain}} clippy --verbose --all-targets --all-features -- -D warnings;
	cargo {{toolchain}} fmt --all -- --check
	cargo {{toolchain}} test --verbose --no-default-features
	cargo {{toolchain}} test --verbose --no-default-features --features "rand"
	cargo {{toolchain}} test --verbose --no-default-features --features "chrono"
	cargo {{toolchain}} test --verbose --no-default-features --features "serde"
	cargo {{toolchain}} test --verbose --no-default-features --features "chrono rand serde"
	cargo {{toolchain}} test --verbose --no-default-features --features "chrono rand doc-comment serde"
	cargo {{toolchain}} test --verbose
	cargo {{toolchain}} doc

# perform a build for every supported toolchain
all:
	just build "+1.40.0"
	just build "+stable"
	just build "+beta"
	just build "+nightly"
