# performs a build with the default toolchain
default: (build "")

# performs a build with the given toolchain
build toolchain *FLAGS:
    cargo {{ toolchain }} --version
    cargo {{ toolchain }} clean {{FLAGS}}
    cargo {{ toolchain }} build {{FLAGS}}
    cargo {{ toolchain }} clippy {{FLAGS}} --all-targets --all-features -- -D warnings
    cargo {{ toolchain }} fmt --all -- --check
    cargo {{ toolchain }} test {{FLAGS}}
    cargo {{ toolchain }} doc {{FLAGS}}
    cargo {{ toolchain }} test {{FLAGS}} --no-default-features
    cargo {{ toolchain }} test {{FLAGS}} --no-default-features --features "rand"
    cargo {{ toolchain }} test {{FLAGS}} --no-default-features --features "chrono"
    cargo {{ toolchain }} test {{FLAGS}} --no-default-features --features "time"
    cargo {{ toolchain }} test {{FLAGS}} --no-default-features --features "serde"
    cargo {{ toolchain }} test {{FLAGS}} --no-default-features --features "chrono rand serde"
    cargo {{ toolchain }} test {{FLAGS}} --no-default-features --features "time rand serde"
    cargo {{ toolchain }} test {{FLAGS}} --no-default-features --features "chrono time rand serde"
    cargo {{ toolchain }} test {{FLAGS}} --no-default-features --features "chrono time rand serde rocket"

# perform a build for every supported toolchain
all *FLAGS:
    just build "+1.88" {{FLAGS}}
    just build "+stable" {{FLAGS}}
    just build "+beta" {{FLAGS}}
    just build "+nightly" {{FLAGS}}

# perform a build using Minimum Supported Rust Version toolchain
msrv *FLAGS:
    just build "+1.88" {{FLAGS}}
