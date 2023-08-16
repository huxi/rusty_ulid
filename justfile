# performs a build with the default toolchain
default: (build "")

# performs a build with the given toolchain
build toolchain:
    cargo {{ toolchain }} --version
    cargo {{ toolchain }} clean
    cargo {{ toolchain }} build --verbose
    cargo {{ toolchain }} clippy --verbose --all-targets --all-features -- -D warnings
    cargo {{ toolchain }} fmt --all -- --check
    cargo {{ toolchain }} test --verbose
    cargo {{ toolchain }} doc
    cargo {{ toolchain }} test --verbose --no-default-features
    cargo {{ toolchain }} test --verbose --no-default-features --features "rand"
    cargo {{ toolchain }} test --verbose --no-default-features --features "chrono"
    cargo {{ toolchain }} test --verbose --no-default-features --features "time"
    cargo {{ toolchain }} test --verbose --no-default-features --features "serde"
    cargo {{ toolchain }} test --verbose --no-default-features --features "chrono rand serde"
    cargo {{ toolchain }} test --verbose --no-default-features --features "time rand serde"
    cargo {{ toolchain }} test --verbose --no-default-features --features "chrono time rand serde"

# perform a build for every supported toolchain
all:
    just build "+1.67.0"
    just build "+stable"
    just build "+beta"
    just build "+nightly"

# perform a build using Minimum Supported Rust Version toolchain
msrv:
    just build "+1.67.0"
